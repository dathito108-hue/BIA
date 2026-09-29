# BIA V137 — trading hardening

This release adds defensive execution infrastructure to the V136 Solana mainnet path. It is not evidence of profitable trading or a claim of complete DEX coverage.

## Implemented

- Required wallet-specific per-order and daily turnover/count caps; SOL/USDC portfolio marks in USDC; daily equity-decrease and peak drawdown latches; projected SOL allocation check; persistent emergency halt. Policy reset explicitly establishes a new baseline without deleting daily consumed turnover.
- SQLite reservations before signing, single dispatch transition, many-order journal, finalized receipt asset deltas, idempotent reconciliation, and JSON export. Export is bounded to 10,000 records per table and reports total/truncation fields.
- Read-only persisted Android JobScheduler reconciliation. Unknown dispatch remains locked; no automatic resubmission. Restart cancels unsubmitted reservations, preserves potentially submitted orders. Scheduling is subject to Android restrictions, connectivity, and force-stop behavior.
- Strict legacy/v0 and ALT decoding; known Jupiter route arguments, source/destination ATAs, amount/slippage/platform fee validation; limited setup instruction allowlist; existing token account ownership and authority checks; real RPC preflight account deltas; Ed25519 wallet signature verification with Bouncy Castle.
- Mainnet reference quotes every 15 seconds while foreground; native Rust sequential signal scoring against subsequent observations, cost hurdle and Wilson bound. At least 21 contiguous observations are required; last 240 used, gaps reset the window. These are price-direction observations, not fills, realized P&L, calibrated predictions or demonstrated strategy edge.
- Backup exclusion for the authority ledger, no private keys or signed payload persistence, no third-party AI backend.

## Explicit limits

- Actual swap execution remains SOL/USDC on Solana only. Existing EVM paths produce unsigned intents, not this execution/risk pipeline. Other chains, derivatives and arbitrary tokens are not covered.
- Only known Jupiter V6 `route` and `shared_accounts_route` layouts and the pinned 39 legacy AMM enum variants are accepted. Unknown modern routers/variants, Token-2022, seeded non-ATA WSOL accounts, nonzero existing WSOL and unsupported setup instructions fail closed. Therefore a valid market quote may still be untradable through this release.
- Canonical Jupiter/SPL programs and the configured RPC remain trust dependencies. This is not an audit of every underlying AMM, upgrade authority or inner instruction implementation. Reference schema: https://github.com/jup-ag/jupiter-cpi/blob/main/idl.json (Apache-2.0).
- Portfolio includes native SOL and standard USDC only; it is not full-wallet NAV. External balance changes require explicit baseline review. Allocation is a pretrade estimate using the quoted output/current mark; it is not an on-chain maximum holdings constraint. Slippage, fees and valuation movement affect actual allocation. No invented realized P&L/cost basis.
- Wallet connection does not grant background spend authority. Every order still needs a wallet signature; automatic signing is not enabled. No actual funded wallet signing/fill has been performed as release validation.
- Real RPC `simulateTransaction` is a read-only safety check, not a paper-trading environment. Synthetic transactions exist only in test code and are never broadcast.

## Validation gates

Core CI: clippy, full Rust tests and established benchmarks, including new no-lookahead/freshness/cost-hurdle tests. Android CI: build both ABIs and execute 13 emulator tests (existing game/realtime integration plus five hardening cases covering signature bytes, routes, reservations/restart, caps/latches, receipts and scheduled recovery/native quality). Public mainnet read tests do not validate funded execution or profitability.
