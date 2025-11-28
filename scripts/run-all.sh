#!/usr/bin/env bash
set -euo pipefail

# Determine project ROOT relative to this script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PIDS=()

# ------------------------------------------------------------------------------
# PORTS TO CLEAN BEFORE STARTING ANYTHING
# ------------------------------------------------------------------------------
PORTS_TO_KILL=(3001 3002 3003 3004 3005 3006 4000)

preclean_ports() {
  echo "🧹 Cleaning up any existing processes on subgraph/router ports..."
  for PORT in "${PORTS_TO_KILL[@]}"; do
    PIDS_ON_PORT=$(lsof -ti tcp:"$PORT" || true)
    if [[ -n "$PIDS_ON_PORT" ]]; then
      echo "   🔪 Killing processes on port $PORT..."
      kill -9 $PIDS_ON_PORT 2>/dev/null || true
    else
      echo "   ✔️  Port $PORT is free."
    fi
  done
  echo "🧼 Ports cleaned!"
  echo
}

cleanup() {
  echo
  echo "🛑 Caught interrupt — stopping all started services..."
  if [[ ${#PIDS[@]} -gt 0 ]]; then
    kill "${PIDS[@]}" 2>/dev/null || true
    wait "${PIDS[@]}" 2>/dev/null || true
  fi
  echo "✅ All services stopped."
  exit 0
}

trap cleanup INT TERM

# ------------------------------------------------------------------------------
# HEALTH CHECK HELPERS
# ------------------------------------------------------------------------------

# Map service -> health URL
health_url_for_service() {
  case "$1" in
    identity) echo "http://localhost:3005/graphql" ;;
    wallet) echo "http://localhost:3006/graphql" ;;
    user)     echo "http://localhost:3001" ;;
    term)     echo "http://localhost:3002" ;;
    taxonomy) echo "http://localhost:3003" ;;
    business) echo "http://localhost:3004" ;;
    *)        echo "" ;;
  esac
}

wait_for_service() {
  local service="$1"
  local url
  url="$(health_url_for_service "$service")"

  if [[ -z "$url" ]]; then
    echo "⚠️  No health URL configured for service '$service', skipping wait."
    return 0
  fi

  echo "⏳ Waiting for $service to be healthy at $url ..."
  local max_retries=60
  local attempt=1

  while (( attempt <= max_retries )); do
    # NOTE: removed -f so ANY HTTP response counts as healthy
    if curl -sS "$url" >/dev/null 2>&1; then
      echo "✅ $service is UP at $url"
      return 0
    fi
    sleep 1
    ((attempt++))
  done

  echo "❌ Timed out waiting for $service at $url after ${max_retries}s"
  return 1
}

# ------------------------------------------------------------------------------
# START
# ------------------------------------------------------------------------------

echo "🚀 Starting workspace services from root: $ROOT_DIR"
echo

# 1) Make sure all ports are free
preclean_ports

# 2) Start subgraphs
NODE_SERVICES=("identity" "wallet")
RUST_SERVICES=("user" "term" "taxonomy" "business")

# --- NODE SUBGRAPHS ----------------------------------------------------------
for SERVICE in "${NODE_SERVICES[@]}"; do
  echo "📦 Starting Node subgraph: $SERVICE"
  (
    cd "$ROOT_DIR/node-workspace"
    pnpm run start "$SERVICE"
  ) &
  PIDS+=($!)
done

# --- RUST SUBGRAPHS ----------------------------------------------------------
for SERVICE in "${RUST_SERVICES[@]}"; do
  echo "🦀 Starting Rust subgraph: $SERVICE"
  (
    cd "$ROOT_DIR/rust-workspace"
    cargo run --package "$SERVICE" --release
  ) &
  PIDS+=($!)
done

# 3) Wait for all subgraphs to become healthy
echo
echo "🩺 Waiting for all subgraphs to be healthy before composing supergraph..."

for SERVICE in "${NODE_SERVICES[@]}" "${RUST_SERVICES[@]}"; do
  if ! wait_for_service "$SERVICE"; then
    echo "❌ Aborting: service '$SERVICE' did not become healthy. Check its logs."
    exit 1
  fi
done

echo "✅ All subgraphs reported healthy."
echo

# 4) Compose supergraph
echo "🧩 Composing supergraph..."
(
  cd "$ROOT_DIR/gateway"

  if ! command -v rover >/dev/null 2>&1; then
    echo "❌ rover CLI is not installed"
    echo "   Install:"
    echo "   curl -sSL https://rover.apollo.dev/nix/latest | sh"
    echo "   echo 'export PATH=\"\$HOME/.rover/bin:\$PATH\"' >> ~/.bashrc"
    exit 1
  fi

  # Auto-accept ELv2 license (non-interactive)
  APOLLO_ELV2_LICENSE=accept rover supergraph compose \
    --elv2-license accept \
    --config ./configs/supergraph.yaml \
    --output ./schemas/supergraph.graphql
)

echo "✅ Supergraph built → gateway/schemas/supergraph.graphql"
echo

# 5) Start router
echo "🌐 Starting Apollo Router..."
(
  cd "$ROOT_DIR/gateway"
  ./router -s ./schemas/supergraph.graphql -c ./configs/router.yaml
) &
PIDS+=($!)

echo
echo "🎉 All services started!"
echo "   - Node subgraphs: ${NODE_SERVICES[*]}"
echo "   - Rust subgraphs: ${RUST_SERVICES[*]}"
echo "   - Router: gateway"
echo
echo "Press Ctrl+C to kill all services."
echo

wait
