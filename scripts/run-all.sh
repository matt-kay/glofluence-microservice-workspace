#!/usr/bin/env bash
set -euo pipefail

# Determine project ROOT relative to this script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PIDS=()

# ------------------------------------------------------------------------------
# PORTS TO CLEAN BEFORE STARTING ANYTHING
# ------------------------------------------------------------------------------
PORTS_TO_KILL=(3001 3002 3003 3004 3005 4000)

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

echo "🚀 Starting workspace services from root: $ROOT_DIR"
echo

# ==============================================================================
# PRE-KILL: Ensure clean ports before anything starts
# ==============================================================================
preclean_ports

# ==============================================================================
# START NODE SUBGRAPHS
# ==============================================================================
NODE_SERVICES=("identity")
for SERVICE in "${NODE_SERVICES[@]}"; do
  echo "📦 Starting Node subgraph: $SERVICE"
  (
    cd "$ROOT_DIR/node-workspace"
    pnpm run start --project "$SERVICE"
  ) &
  PIDS+=($!)
done

# ==============================================================================
# START RUST SUBGRAPHS
# ==============================================================================
RUST_SERVICES=("user" "term" "taxonomy" "business")
for SERVICE in "${RUST_SERVICES[@]}"; do
  echo "🦀 Starting Rust subgraph: $SERVICE"
  (
    cd "$ROOT_DIR/rust-workspace"
    cargo run --package "$SERVICE" --release
  ) &
  PIDS+=($!)
done

# ==============================================================================
# BUILD SUPERGRAPH
# ==============================================================================
echo
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

# ==============================================================================
# START ROUTER
# ==============================================================================
echo "🌐 Starting Apollo Router..."
(
  cd "$ROOT_DIR/gateway"
  ./router -s ./schemas/supergraph.graphql -c ./configs/router.yaml
) &
PIDS+=($!)

echo
echo "🎉 All services started!"
echo "   - Node subgraphs: identity"
echo "   - Rust subgraphs: user, term, taxonomy, business"
echo "   - Router: gateway"
echo
echo "Press Ctrl+C to kill all services."
echo

wait
