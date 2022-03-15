# Pembrock Token Contract

Sources of PembRock Finance Token (PEM) contracts.

Deployed to [token.pembrock.near](https://explorer.near.org/accounts/token.pembrock.near)

## Building and deploying

#### Build

```bash
cargo build --release
```

#### Deploy

```bash
near deploy token.pembrock.near out/wasm32-unknown-unknown/release/token.wasm new '{"owner_id":"treasury.pembrock.near","total_supply":"160000000000000000000000000"}'
```
