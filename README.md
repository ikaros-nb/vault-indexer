# vault-indexer

Axum service that receives [Helius](https://www.helius.dev/) enhanced webhook
notifications for a [custom Solana Vault program](https://github.com/ikaros-nb/solana-token-vault), authenticates them, parses the
transaction payload, and logs token balance changes. This is the ingestion layer
of a Solana indexer; Postgres persistence is next.

Because the Vault is a custom program, Helius sends `type: "UNKNOWN"` with an
empty `description` — but the SOL/SPL balance movements are still in the payload,
and that's what this service reads.

## Requirements

- [Rust](https://www.rust-lang.org/) (edition 2024)
- A Helius enhanced webhook, with a shared secret set on both sides
- [ngrok](https://ngrok.com/) (or any tunnel) for local development

## Run

```bash
WEBHOOK_SECRET=<SECRET> cargo run   # required; must match the Helius auth header exactly
```

Binds to `http://127.0.0.1:3000`. Helius can't reach `localhost`, so expose it:

```bash
ngrok http 3000
```

Set the webhook URL in Helius to the forwarded HTTPS URL **plus `/webhook`**
(omitting the path makes Helius POST to `/`, which returns `405`).

## Endpoints

| Method | Path            | Auth | Description                 |
|--------|-----------------|------|-----------------------------|
| `GET`  | `/`             | No   | Hello world                 |
| `GET`  | `/health_check` | No   | Health check (`OK`)         |
| `POST` | `/webhook`      | Yes  | Helius payload (JSON array) |

`POST /webhook` requires `Authorization: <SECRET>`, enforced by a `route_layer`
(so unknown paths still return `404`, not `401`). A bad secret returns `401`.

## Test

```bash
# authorized -> 200 OK
curl -i -X POST http://127.0.0.1:3000/webhook \
  -H "Authorization: <SECRET>" -H "Content-Type: application/json" \
  -d '[{"signature":"t","slot":1,"timestamp":1,"fee":5000,"feePayer":"x","source":"UNKNOWN","description":"","type":"UNKNOWN","accountData":[]}]'

# missing secret -> 401 Unauthorized
curl -i -X POST http://127.0.0.1:3000/webhook -d '[]'
```

## Payload gotchas

- `nativeBalanceChange` is a signed `i64` — deltas can be negative (e.g. fees).
- `rawTokenAmount.tokenAmount` is a **string** (raw, un-scaled, can be negative);
  divide by `10^decimals` only for display.

## Dependencies

axum, tokio, serde / serde_json.

## Roadmap

- Persist to Postgres via `sqlx` (store raw amounts losslessly)
- Decode Vault instruction data against the IDL to recover `deposit` / `withdraw`
- Backfill history (Helius API or `getSignaturesForAddress` + `getTransaction`)
