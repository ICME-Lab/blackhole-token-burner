# `blackhole-token-burner`

## Reproducible Build

```bash
docker build -t wasm-reproducible-build-env .
```

```bash
docker run --rm \
  -v "$(pwd)":/app \
  -w /app \
  wasm-reproducible-build-env \
  cargo build --release --target wasm32-unknown-unknown

shasum -a 256 target/wasm32-unknown-unknown/release/blackhole_token_burner.wasm
```

## Check canister controller

Set the canister as a black-hole

```bash
dfx --ic canister update-settings blackhole-token-burner --set-controller $(dfx canister id blackhole-token-burner)   

```

```bash
dfx --ic canister info blackhole-token-burner
```
