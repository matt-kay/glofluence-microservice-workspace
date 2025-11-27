Nice, Codespaces is perfect for this—it’s basically just Linux in the cloud, so Rover works great there.

Here’s a straight-through setup for **local composition with Rover inside GitHub Codespaces**.

---

## 1. Install Rover in your Codespace

In the Codespaces terminal:

```bash
curl -sSL https://rover.apollo.dev/nix/latest | sh
```

That script:

* Downloads the `rover` binary
* Puts it in `~/.rover/bin`

Now add it to your PATH (if the script didn’t already):

```bash
echo 'export PATH="$HOME/.rover/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Confirm it works:

```bash
rover --help
rover --version
```

If that prints usage + a version number, you’re good.

> Tip: Codespaces usually uses bash or zsh. If you’re on zsh, do the same but with `~/.zshrc`.

---

## 2. Generate subgraph SDLs locally

Assuming you already have your subgraphs running in Codespaces:

* products at `http://localhost:4001/graphql`
* users at `http://localhost:4002/graphql`
* etc.

From the repo root in Codespaces, run:

```bash
mkdir -p supergraph-schemas

rover subgraph introspect http://localhost:4001/graphql \
  > supergraph-schemas/products.graphql

rover subgraph introspect http://localhost:4002/graphql \
  > supergraph-schemas/users.graphql
```

You can repeat that for any other subgraph.

---

## 3. Create `supergraph.yaml` in the repo

At the root of your project (where you’ll later run the router), create `supergraph.yaml`:

```yaml
format: "1"
subgraphs:
  products:
    routing_url: http://localhost:4001/graphql
    schema:
      file: ./supergraph-schemas/products.graphql

  users:
    routing_url: http://localhost:4002/graphql
    schema:
      file: ./supergraph-schemas/users.graphql
```

Commit this file into your repo; it’s just config.

---

## 4. Compose the supergraph with Rover

From the project root in Codespaces:

```bash
rover supergraph compose --config ./supergraph.yaml \
  > supergraph.graphql
```

Now you’ll have `supergraph.graphql` at the root. That’s what Apollo Router will use.

You can re-run this command anytime schemas change.

---

## 5. Wire it into Apollo Router (local)

Download the router binary in Codespaces:

```bash
curl -sSL https://router.apollo.dev/download/nix/latest | sh
```

Create `router.yaml`:

```yaml
supergraph:
  path: ./supergraph.graphql

cors:
  allow_any_origin: true

telemetry:
  tracing:
    enabled: true
```

Run the router:

```bash
./router -s ./supergraph.graphql -c ./router.yaml
# Router will listen on http://localhost:4000
```

Then query it:

```bash
curl -X POST http://localhost:4000/ \
  -H "Content-Type: application/json" \
  -d '{"query":"{ product(id:\"1\"){ id name price } }"}'
```

---

## 6. Optional: Make Rover available in all Codespace rebuilds

Codespaces re-builds containers sometimes. To keep Rover installed automatically, you can:

1. Add the install command into your `.devcontainer/Dockerfile` or `postCreateCommand` in `.devcontainer/devcontainer.json`.

Example `postCreateCommand`:

```jsonc
// .devcontainer/devcontainer.json
{
  "postCreateCommand": "curl -sSL https://rover.apollo.dev/nix/latest | sh && echo 'export PATH=\"$HOME/.rover/bin:$PATH\"' >> ~/.bashrc"
}
```

That way, every new Codespace has Rover ready.

---

If you paste me your current folder structure (subgraph dirs, etc.), I can write a `supergraph.yaml` and exact commands tailored to *your* setup.






intropect subgraph
rover subgraph introspect http://localhost:<PORT> > ./rust-workspace/gateway/schemas/<NAME>.graphql

create supergraph
rover supergraph compose --config ./configs/supergraph.yaml > ./schemas/supergraph.graphql

run supergraph
./router -s ./schemas/supergraph.graphql -c ./configs/router.yaml
