# V139 — composable products and guarded schema evolution

V139 extends the V138 native product compiler; no external AI or existing model architecture is introduced. It remains a bounded compiler, not arbitrary software generation or a demonstrated sales business.

## New capabilities

- A combined product has quotation, inventory and task tabs in one generated application. Existing single-function products remain supported. Tabs have independent rows, balances and backups; there are no automatic relationships, stock deductions or cross-module transactions.
- Up to 12 user-defined fields, shared across the product's modules: text, decimal number (up to 12 integer / 4 fractional digits), calendar date and choice list (up to 12 distinct values). Required/optional rules are enforced in the generated UI and data importer. Stable field identities survive label edits. Values are included in edit, search, CSV, JSON backup and print output.
- The Android workbench adds field editing and configuration cloning to a new product identity. No code/identifier entry is required by the user. Cloning does not copy deployed customer data.
- Product-specific acceptance instructions are generated from the field definitions. These are instructions for verification, not fabricated acceptance evidence.

## Compatibility contract

BIA_PRODUCT_1 specifications and the original eleven-file bundle shape remain readable. SQLite v1 is migrated transactionally to v2; a retained schema head prevents deleting all builds from bypassing compatibility checks. Version numbers continue from the retained head. Labels/branding and additional optional fields are compatible. A single module can expand to the combined product. Currency changes, removal of modules/fields, type changes, removing choice values, making an optional field required or adding a required field to an existing product are blocked. A new product copy is available for such changes.

Generated applications read BIA_PRODUCT_DATA_1 backups and write BIA_PRODUCT_DATA_2 with field metadata. Legacy rows gain empty optional values without dropping their original fields. Incompatible metadata or unknown custom keys is rejected rather than silently stripped. Each module retains the V138 storage key (same product ID, module, currency and origin). Maximum 500 rows and 1 MB UTF-8 backup per module; oversized edits are rejected before the stored state changes. Save JSON for each tab before deployment upgrades. Sharing source ZIP does not include browser/preview data.

## Validation and limits

Rust tests cover V1/V2 round trips, schema changes and malformed fields. Node tests include legacy backup migration, required/typed data, unknown fields, choice removal, CSV and UTF-8 size limits. Android tests add a combined product exercising all four custom field types across three tabs, reload persistence, a generated ZIP, and a v1 database migration with retained compatibility checks after deletion. Existing product, trading and game tests remain (19 Android tests total).

The runtime is single-user and browser-local. It still has no backend, accounts, payments, cloud sync, arbitrary business logic, relational database or automatic selling. Simultaneous tabs are not transactional. Browser/OS data deletion can remove local records; user backups and customer-device acceptance are still necessary. This release does not establish customer demand or revenue.
