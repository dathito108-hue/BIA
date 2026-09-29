# BIA — Buddhist Intelligence Architecture

**BIA-DCA** is an experimental computational-intelligence architecture inspired by Vietnamese Buddhist thought, especially dependent arising, momentary cognition, the five aggregates, the Middle Way and the engaged/this-life emphasis associated with Trúc Lâm.

BIA is **not** an LLM, Transformer, SSM, Mamba, or a renamed version of an existing model family. Buddhist terms are used as computational design inspiration, not as scientific proof or a claim that software possesses religious consciousness.

## Canonical intelligence loop

```
Tam Thien World
      ↓
Canh → Xuc → Tho/Tuong → Thuc
      ↓
Duyen Quan
      ↓
Ky uc + Gia thuyet + Muc tieu
      ↓
Tri
      ↓
Hanh
      ↓
Qua
      ↓
Huan tap ──────────────┐
                       └→ future Canh/Duyen
```

### What is implemented now

- **Tam Thiên WorldGraph**: bounded three-level world representation (Tiểu/Trung/Đại Thiên).
- **Duyên relations**: explicit causal/enabling/inhibiting/context relations.
- **Cognitive Moment**: sparse active causes, recognition, hypotheses, uncertainty and intention.
- **Seed Memory (Chủng tử)**: bounded experiential memory that can strengthen/merge/evict without retraining a global parameter matrix.
- **Trung Đạo Budgeter**: selects Tĩnh/Nhanh/Thường/Sâu from task need and device pressure.
- **Quán–Trí–Hành runtime**: builds hypotheses from conditions, recalls experience, evaluates benefit/harm/reversibility and proposes an intention.
- deterministic hard ceilings for world nodes, edges, memories and active reasoning.

## Non-negotiable invariants

1. No token prediction is the definition of intelligence.
2. No Transformer attention requirement.
3. No SSM/recurrent neural family requirement.
4. No permanent hidden "self vector".
5. World, memory, reasoning and action are explicit first-class structures.
6. Learning may modify experience/relations without retraining the entire system.
7. The same core must scale by capacity, not by changing into another architecture.
8. Offline execution is the default target; cloud is optional, never required by the core.

## Device scalability

BIA adapts capacity instead of architecture:

- tiny device: small world/memory limits, mostly Tĩnh/Nhanh;
- phone: larger active world, memory and several Quán cycles;
- PC/server: deeper world and more parallel Cảnh, using the same data contracts.

The goal is broad portability, not the physically impossible claim that identical workloads fit every device.

## Status

The repository now includes a usable Android shell backed by the native Rust **BIA-DCA runtime**. Mobile V2 adds a redesigned offline chat interface, human-readable Vietnamese responses, native Dharma-memory save/restore across app restarts, live cognition status, and ARM64 APK CI. It is still an experimental intelligence architecture rather than a proven AGI. Capability V3 adds bounded multi-turn dialogue context, explicit goals, user-taught local memory, structured Android actions with confirmation, and action-outcome learning.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/ROADMAP.md](docs/ROADMAP.md).


## Capability V3 mobile commands

The Android shell now recognizes a small, explicit set of useful capabilities while keeping authority separate from cognition:

- `Nhớ rằng ...` / `Ghi nhớ ...` — imprint a local experiential memory.
- `Mục tiêu: ...` — keep an active goal in the runtime.
- `Tìm web ...` — propose opening an external web search.
- `Mở YouTube` or `Mở https://...` — propose an external VIEW action.
- `Mở cài đặt` — propose opening Android system settings.
- `Mở ứng dụng <package.name>` — propose launching an installed package.
- `Sao chép ...` — propose writing text to the clipboard.

Every device action is surfaced to the Android UI for explicit confirmation. Success/failure is sent back into BIA and becomes experiential feedback.


## Capability V4–V5

The mobile runtime now adds faster practical continuity and voice interaction:

- Vietnamese speech input through Android's speech recognition intent.
- Vietnamese TTS output through the device TTS service.
- Real battery and thermal state feeding the BIA Middle-Way compute budget.
- A bounded multi-step action queue.
- Runtime continuity for active goals and pending actions across app restarts.
- `Tiếp tục` can derive the next action from an active research/search goal.
- Every queued external action still requires a separate user confirmation.
- Action success/failure continues to become experiential memory.


## Capability V6–V8

This capability pack expands BIA without introducing an LLM/Transformer/SSM backend:

- **Perception intake:** Android Share can send plain text directly into BIA; the app can also open local text documents through the system document picker.
- **Provenance ledger:** every ingested source is recorded as user, shared text, local document, web excerpt or system context.
- **Knowledge-to-experience:** ingested content is converted into bounded BIA phenomena and seed experience rather than stored only as opaque text.
- **Goal decomposition:** explicit goals can be split into multiple executable device actions when the goal contains recognizable action clauses.
- **Continuity v2:** active goals, pending actions and provenance records survive restart.
- **No broad storage permission:** local document access uses Android's Storage Access Framework.


## Inference V9 — Tam-Thiên Matrix

BIA now has a dedicated hot-path inference primitive inspired by the *three nested thousands* structure of the Buddhist trichiliocosm. The inspiration is structural, not a claim that Buddhist cosmology is a numerical AI algorithm.

- **Tiểu Thiên:** sparse local signals are accumulated into 16 fixed Q15 lanes. Only active lanes are touched.
- **Trung Thiên:** each lane mixes only its nearest causal neighborhood, avoiding all-to-all interaction.
- **Đại Thiên:** four coarse sectors summarize local bundles and select the global winner/decision.
- **Early exit:** if Tiểu Thiên already has sufficient strength and margin, BIA emits an immediate first word-token without waiting for deeper contemplation.
- **Fixed-point path:** Q15 integer arithmetic avoids floating-point matrix multiplication in the hot path.
- **Bounded work:** input sampling is capped, lane count is fixed, and the core hierarchy remains O(1) with respect to stored world size for the fast path.
- **Asynchronous Android response:** the UI can render the immediate token first and run full native contemplation off the UI thread.

This is not Transformer token prediction. The current token path is a deterministic BIA word-token emitter driven by the hierarchical inference state.


## Inference V10 — Four-Matrix Cognitive Kernel

The hot path now runs:

`Realm Mask → Dependent Origination recurrence → Perspective Projection → Zero-State → Tam-Thien Matrix`

- Realm Mask gates five bounded engineering channels inspired by the five aggregates.
- Dependent Origination uses element-wise Q15 recurrent updates, not dense matrix multiplication.
- Projection derives technical, affective and global views from one conditioned state.
- Zero-State centers/prunes temporary scratch only; durable SeedMemory and KnowledgeLedger are not erased.
- The output feeds the existing 16-lane Tam-Thien hierarchy and instant-token path.
- All hot-path arrays are fixed size (5 and 16 lanes) and do not grow with context size.

The Buddhist terminology is architectural inspiration, not a claim that Buddhist doctrine is a literal numerical model of cognition.


## Inference V11 — Adaptive Realm + Duyên-Token

V11 moves the instant output path from a fixed discourse-word emitter to a recurrent Duyên-token decoder.

- Realm masks are now adaptive per event: the base realm prior is modulated by the actual five-channel signal distribution.
- The Duyên-token decoder feeds each generated token back into an 8-lane recurrent state and re-runs the bounded Four-Matrix → Tam-Thiên decision.
- Internal inference signals use fixed stack arrays; generated output is capped at 48 word-tokens.
- The Android instant-token JNI path now uses the Duyên-token decoder directly.
- A release evaluation suite verifies semantic first-token behavior, deterministic output, realm classification and a complexity-regression benchmark.

### What V11 proves — and what it does not

The automated evidence demonstrates that the current bounded task set is executable, deterministic, resource-bounded and fast on the GitHub Actions x86_64 runner, while the same Rust core cross-compiles into the ARM64 Android APK. It does **not** prove open-domain language quality or phone-specific latency; those require a device benchmark and a broader held-out corpus.


## Inference V12 — Mobile Proof + Open Vocabulary

V12 extends the recurrent Duyên-token path in two measurable directions:

- **Bounded learned vocabulary:** up to 128 learned word-tokens can be acquired from provenance-backed content already ingested by BIA.
- **Dynamic emission:** learned tokens can participate in generation after the first constrained control tokens.
- **Held-out Vietnamese evaluation:** V12 adds prompts not used by the V11 acceptance set.
- **On-device benchmark:** type `/bench` or `benchmark` in the Android app to execute the native decoder repeatedly on the actual phone and report elapsed time, ns/token, tokens/s and learned-vocabulary size.
- **CI benchmark:** release CI still protects against accidental complexity growth on x86_64.

The Android benchmark is the authoritative device-specific speed measurement. CI throughput must not be presented as phone throughput.


## Inference V14 — Stress / Robustness Proof

V14 adds stronger evidence for the bounded recurrent decoder:

- 5,000–50,000 iteration stress evaluation.
- Unicode, punctuation/noise and very long-input survival.
- Full learned-vocabulary pressure at the 128-token cap.
- Deterministic replay checks on every stress iteration.
- Token-count and recurrent-state boundedness checks.
- Release CI stress benchmark.
- Android `/stress` command runs 10,000 iterations natively on the actual device and reports pass/fail plus timing.

This proves robustness only for the explicit invariants and stress corpus above. It is not a claim of open-domain intelligence quality.


## Inference V15 — Reasoning Quality Proof

V15 strengthens reasoning quality rather than raw throughput.

- Bounded multi-hop causal reasoning up to depth 4 with beam 12.
- Explicit support vs opposition accumulation.
- Contradiction detection when both positive and inhibiting causal evidence are present.
- Confidence is reduced when evidence conflicts.
- New evidence can revise the current causal verdict.
- Unrelated graph updates do not disturb an existing causal conclusion.
- Multi-hop paths are fed back into BIA contemplation as hypotheses.
- Android command `/reason` runs the reasoning-quality suite natively.

The V15 suite uses synthetic causal graphs with known expected answers. Passing it proves these reasoning invariants, not general open-domain reasoning ability.
