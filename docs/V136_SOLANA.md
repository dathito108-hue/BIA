# BIA V136 — Solana mainnet and external-wallet execution

## Implemented
- Adds a separate Android Solana terminal alongside Ethereum/BNB V2.
- Reads actual Solana mainnet genesis, confirmed slot/block time and wallet SOL balance using PublicNode HTTPS RPC. Rejects wrong genesis or block time older than 60 seconds / over 5 seconds in the future.
- Jupiter Swap V2 `/order` for SOL ↔ USDC, fixed exact-input amounts, 0–100 bps slippage, Metis routing only. Keyless access or optional user API key held in memory. Quote monitoring polls every 15 seconds after completion while foreground; no invented ticks/candles, no simulation environment, no unattended background service.
- Solana Mobile Wallet Adapter Java client 2.0.8: authorization, in-memory reauthorization, public address, deauthorization, and external-wallet `signTransactions` request. Missing/rejecting/incompatible wallets fail closed. Private keys never enter BIA. Some wallets may not support sign-only MWA requests.
- Per-order review shows input/output/minimum, slippage, impact, platform fee, network fee, rent, and wallet. Rejects price impact magnitude >1%, fee+rent over user budget (maximum 0.01 SOL), altered quote intent, expired quote (>60 seconds from request start), unsupported multisigner/sponsored transactions, changed wallet, changed signed message, or absent signature.
- Mainnet read-only preflight uses `isBlockhashValid`, `getFeeForMessage`, `simulateTransaction`, and SOL balance. This RPC preflight validates a proposed real transaction; it is not a paper-trading environment and moves no funds. SPL insufficiency is caught by execution preflight. Preflight repeats after signing, before sending.
- After the user reviews and approves in their wallet, the application submits the signed message once to Jupiter `/execute`. No network-level POST retries. Locally computes and records the signature **before** submission. RPC `getSignatureStatuses` determines UNKNOWN/PENDING/CONFIRMED/FINALIZED/FAILED; an API response alone does not prove a fill.
- A pending/unknown transaction blocks a new trade until reconciliation. If a submitted request remains absent from the chain, V136 intentionally keeps it blocked: no automatic replacement/retry/assumption of failure. Explorer link supports investigation.
- Stop cancels outstanding preparation/wallet session and invalidates drafts. Transactions already submitted cannot be canceled. Process death preserves the last pending signature/status but not signed payloads, API keys, or wallet auth tokens.

## Important limits
This is a manually approved mainnet integration, **not unattended automatic signing**. MWA authorization is not blanket spend authority. BIA does not click wallet approvals, acquire/export private keys, create delegated spending allowances, or bypass wallet prompts. Fully automatic signing requires a separately designed, wallet-supported bounded delegation, explicit allowed assets/caps/expiry, transaction semantic verification, and tests with the selected wallet.

BIA checks quote metadata, one fee-payer/signer, unchanged message bytes and RPC preflight. **It does not decode or independently audit all Jupiter program instructions, inner instructions, or address lookup tables.** The external wallet's transaction review and trust in Jupiter/PublicNode remain required. Preflight success is not a security audit or guarantee of output, profit, MEV protection, or finality. The quoted min-output is service metadata; on-chain enforcement is supplied by the Jupiter-generated program instruction. These limits prohibit unattended signing in this version.

The market feed does not yet build Solana OHLC history or provide a statistically validated trading strategy. There is no portfolio P&L, full historical ledger, stop-loss keeper, priority-fee optimizer, private RPC SLA, or professional-performance claim. Journal currently retains the last order, not a tax/accounting ledger.

## User flow
1. Open **Solana: ví và giao dịch mainnet**. Install/use an MWA-compatible wallet on the same Android device.
2. Choose SOL → USDC or USDC → SOL, enter quantity, slippage and fee/rent budget. Optional Jupiter API key belongs in the app field, never a seed/private key.
3. **Theo dõi báo giá thật** is read-only. **Liên kết ví Solana** opens the wallet's connection flow.
4. **Lấy lệnh mới & kiểm tra mainnet** requires a connected wallet and adequate funds. Read the report.
5. **Xem lệnh rồi yêu cầu ví ký** → review BIA dialog → review actual instructions in wallet → approve there only if correct. BIA sends after approval; slow approval can expire the 60-second quote and require a fresh order.
6. Use **Đối soát giao dịch gần nhất trên chuỗi** / Solscan. UNKNOWN is not success or failure and must not cause an automatic second trade.
7. **Thu hồi kết nối tại ví** revokes MWA authorization. Closing the app only discards its local token; it does not revoke wallet-side authorization.

## Validation scope
Android instrumentation covers real mainnet genesis/block freshness and a real keyless Jupiter quote; negative tests cover altered payer/message, amount precision, minOut, invalid impact, stale quote, write-RPC rejection, and no-wallet UI. Existing EVM, Binance and native game tests remain enabled. CI does not contain wallet keys or funded accounts and cannot prove a commercial wallet's approval flow, mainnet signature, or executed swap. No real trade was made by the assistant.

## Official references
- https://docs.solanamobile.com/android-native/using_mobile_wallet_adapter
- https://github.com/solana-mobile/mobile-wallet-adapter
- https://developers.jup.ag/docs/swap/order-and-execute
- https://developers.jup.ag/docs/api-reference/swap/order
- https://developers.jup.ag/docs/portal/plans
- https://solana.com/docs/rpc/http/simulatetransaction
- https://solana.com/docs/rpc/http/getsignaturestatuses
