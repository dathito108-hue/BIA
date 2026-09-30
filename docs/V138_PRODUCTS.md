# V138 — Product Studio

BIA now compiles a bounded product specification into an independently usable HTML/CSS/JavaScript tool on Android. This extends the native BIA implementation; it adds no external AI/model backend and is not a general autonomous software engineer.

## Workflow

Open **Xưởng sản phẩm**, select quotation, inventory or tasks, enter name, audience, problem, author, currency and color, then build. The Rust compiler validates the structured specification; it does not infer arbitrary features from prose. Preview the stored version, exercise operations, and export its ZIP using Android's document picker. Changed form fields are only applied by generating a new version.

Each immutable build stores its source bundle and SHA-256 digest. Up to 200 versions are retained; the user can export/delete individual versions. Draft form and selected version survive reopen. ZIP contains eleven source/document files plus a per-file SHA-256 manifest. A previous build is never overwritten by a failed generation. Preview is local, disallows file/content/network access and navigation, and exposes no JavaScript-to-native bridge. Preview data is separate from the exported browser product and is not included in the source ZIP.

## Products and limits

- Quotation: integer quantities, exact integer minor-unit arithmetic, line items, totals before tax, notes; VND/USD/EUR. No e-invoice, tax engine, payments or accounting claim.
- Inventory: unique SKU, current quantity, low-stock threshold and notes. No inventory movement audit trail, concurrency guarantees or multi-user synchronization.
- Tasks: due date, priority, status and notes. No background reminders or accounts.
- All: edit/delete/search, browser storage, bounded validated JSON backup/restore, CSV export hardened against spreadsheet formula prefixes, print/PDF via browser, built-in arithmetic/format smoke checks. Maximum 500 rows and 1 MB imported JSON. No user data is sent to a server by generated code.

localStorage availability for file URLs varies by browser; use a static server if persistence is unavailable, keep a stable origin and make JSON backups. Browser data deletion loses unsaved data. This is a single-user local tool. Concurrent tabs are refreshed through storage events, but simultaneous writes are not transactionally coordinated. Currency/type changes use a separate data namespace; no automatic currency conversion. Browser CSV/backup downloads and printing are intentionally unavailable in Android preview; export and open the standalone bundle for those functions.

The generated README states deployment/acceptance limits. SALES-DRAFT.md is a draft based on the supplied audience/problem with a blank pricing worksheet, not market research or a sales promise. QA.json marks product-specific runtime acceptance and market validation as unverified. Source is distributed under Apache-2.0 with LICENSE and NOTICE; no exclusive rights to the common runtime are promised. No posting, messaging, payment setup, spending or sale is performed.

## Validation

Four Rust compiler tests cover determinism, all product types, malicious strings and invalid specs. Node runtime tests exercise exact money, bounds, duplicate SKUs, dates, CSV escaping and backup identity. Four Android tests cover native generation, ZIP checksums, durable versions/corruption detection, all three generated products in WebView (CRUD/persistence/invalid input), and workbench build/reopen. Existing trading/game tests remain required (17 Android cases total).

Runtime UI evidence is for the generated product families, not universal browser compatibility, customer acceptance or revenue. Real customer testing and a chosen sales channel remain separate work.
