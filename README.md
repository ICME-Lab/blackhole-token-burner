blackhole-token-burner
======================

A minimal Internet Computer canister that permanently burns KINIC by transferring it to the Kinic token minter account (which is Kinic SNS governance canister). It exposes a single update method, `burn`, which calls the Kinic ICRC-1 ledger’s `icrc1_transfer` with the burn destination, and an `info` query for quick verification of the configured ledger and minter principals.

## Canister interface
- `info() -> text` — Returns the hard-coded Kinic minter and ledger principals the canister uses.
- `burn(e8s: nat) -> variant { Ok : nat; Err : BurnError }` — Sends the given amount of KINIC (in 10^-8 units) to the minter’s burn address. On success it returns the ledger block index; on failure it returns the ledger transfer error or a call rejection string.

## Prerequisites
- Rust toolchain with `wasm32-unknown-unknown` target installed.
- `dfx` for building/deploying to a local replica or the IC.
- Docker (optional) for reproducible builds.

## Reproducible build
Build and compile in a deterministic Docker environment:
```bash
docker build -t wasm-reproducible-build-env .

docker run --rm \
  -v "$(pwd)":/app \
  -w /app \
  wasm-reproducible-build-env \
  cargo build --release --target wasm32-unknown-unknown
```
Verify the artifact hash:
```bash
shasum -a 256 target/wasm32-unknown-unknown/release/blackhole_token_burner.wasm
# Expected: bd9a23136a8d1949046ee44085a615c83960a61b810bd58cd63dec056f4030d2
```

## Black-hole
Make the canister its own controller so no one (including us) can upgrade it:
```bash
dfx --ic canister update-settings blackhole-token-burner \
  --set-controller "$(dfx --ic canister id blackhole-token-burner)"

dfx --ic canister info blackhole-token-burner
```

## Usage
Check configured principals:
```bash
dfx canister --network ic call blackhole-token-burner info
```

Burn 1 KINIC (100_000_000 e8s). The call returns either the ledger block index or a `BurnError`:
```bash
dfx canister --network ic call blackhole-token-burner burn '(100_000_000:nat)'
```

## Project layout
- `src/blackhole-token-burner/src/lib.rs`: Canister logic and Candid export.
- `src/blackhole-token-burner/src/types.rs`: Candid types mirroring ledger transfer arguments and errors.
