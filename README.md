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


## Inference V16–V17 — Generalization + Counterfactual Reasoning

This block extends causal reasoning beyond the V15 training-shaped cases:

- causal chains up to six nodes with bounded beam search;
- strong unrelated distractor edges;
- counterfactual queries by removing one causal condition without mutating durable world state;
- reversal tests where new inhibiting evidence must overturn the current direction;
- persistence checks after later unrelated world/memory additions;
- 128 held-out synthetic causal worlds with known answers;
- Android command `/generalize` runs the same proof natively.

The counterfactual engine compares factual vs. "without X" support/opposition while keeping reasoning bounded. Passing the suite proves these structural generalization invariants only; it does not prove unrestricted open-domain reasoning.


## Inference V18–V20 — Semantic Open Reasoning

This block bridges natural Vietnamese text into BIA's causal intelligence:

- **Semantic Scene Parser:** extracts bounded entities, causal/enabling/inhibiting clauses and causal/counterfactual queries from Vietnamese text.
- **Knowledge-to-Graph:** provenance-backed ingested documents are also parsed into the WorldGraph, so learned text can become causal structure rather than opaque memory.
- **Semantic Retrieval:** when a query cannot be answered from the active graph, BIA retrieves relevant provenance records from KnowledgeLedger, rehydrates them into the graph and retries reasoning.
- **Compositional Reasoning:** multi-sentence language chains can be composed into multi-hop causal answers.
- **Counterfactual Questions:** supports bounded "nếu bỏ X thì Y?" reasoning using the V16–V17 engine.
- **Contradiction Awareness:** conflicting textual causes remain explicit rather than being collapsed into a single forced answer.
- **Android proof:** `/openproof` runs the open-reasoning evaluation natively.

The parser is intentionally bounded and rule-guided. Passing the suite proves that BIA can turn a defined family of previously unseen Vietnamese causal sentences into graph reasoning; it does not prove unrestricted natural-language understanding.


## Inference V21–V24 — Abstraction, Induction and Analogy

This block deepens BIA's open intelligence without adding a large language model backend.

- **Concept abstraction:** explicit synonym/definition statements can merge multiple surface forms into one bounded canonical concept group.
- **Paraphrase resilience:** later causal queries can use a learned alias and still address the same concept graph.
- **Structural analogy:** when source and target entities are marked similar, BIA can transfer a causal/enabling/inhibiting relation with discounted confidence.
- **Induction from examples:** a target relation is induced only when at least two structurally similar source examples support the same relation kind.
- **Compositional reasoning after abstraction:** synonym learning and multi-hop causal reasoning work together.
- **Bounded memory:** concept groups are capped at 256, aliases at 8 per group, analogy hypotheses at 16, and induction scans at most 64 source relations.
- **Android proof:** `/deep` runs the V21–V24 proof suite natively.

These capabilities are structured symbolic/generalization mechanisms. They improve breadth and transfer but do not make BIA equivalent to a frontier-scale language model.


## Inference V25–V30 — Emergent Structured Intelligence

This block makes BIA learn more structure from repeated experience instead of relying only on explicitly provided definitions.

- **Episodic Memory:** stores up to 128 bounded semantic episodes, each with at most 12 clauses.
- **Context Discovery:** discovers Similar relations when previously separate entities repeatedly occupy the same causal role in at least two shared contexts.
- **Rule Synthesis:** repeated two-step episode patterns can become bounded composition rules such as Causes + Enables -> Causes.
- **Rule Transfer:** synthesized rules can be applied to a later unseen chain without replaying all prior episodes.
- **Hypothesis Competition:** causal, analogical, induced or rule-based candidates can compete; near-equal opposite hypotheses remain contradicted instead of being forced into one answer.
- **Multi-domain structural reasoning:** the same bounded reasoning machinery is tested across changing battery/network/game/risk-style symbolic domains.
- **Android proof:** `/emergent` runs the V25–V30 proof suite natively.

Hard bounds remain: 128 episodes, 12 clauses per episode, 16 discovered similarities per pass, 32 synthesized rules, 32 rule applications per pass, and bounded hypothesis scoring.

These mechanisms demonstrate structured learning and transfer over the defined proof families. They do not establish human-level or frontier-model open intelligence.


## Inference V31–V36 — Autonomous Knowledge Formation

This block lets BIA create and govern higher-level knowledge from existing experience.

- **Hierarchical Abstraction:** entities with the same bounded causal-role signature are grouped into higher-level abstract concepts.
- **Autonomous Hypothesis Generation:** missing two-hop causal/enabling/inhibiting closures are proposed as hypotheses rather than silently treated as facts.
- **Falsification / Knowledge Governor:** every autonomous hypothesis is assessed against direct supporting and opposing evidence before promotion.
- **Meta-Rule Compression:** multiple specific synthesized rules sharing the same output relation can be compressed into a bounded higher-order rule.
- **Layered Knowledge Formation:** learning runs through episodes → discovery → hierarchy → rules → meta-rules → analogy → autonomous hypotheses → validation.
- **Bounded promotion:** only hypotheses above a support threshold and below an opposition threshold are written back to the durable WorldGraph.
- **Android proof:** `/autonomy` runs the V31–V36 knowledge-formation suite natively.

Hard bounds remain on abstract concepts, hypothesis candidates, meta-rules and promotion passes. The proof demonstrates autonomous structured knowledge formation inside the defined causal representation; it does not establish unrestricted autonomous scientific discovery or frontier-model intelligence.


## Inference V37–V44 — World Model & Deliberative Intelligence

This block adds bounded internal simulation before action.

- **World Model:** stores up to 64 transition models with preconditions, add/remove effects, utility, cost and confidence.
- **Future-State Simulation:** an action can be simulated against a compact state of at most 64 active facts without mutating the durable WorldGraph.
- **Goal-Directed Deliberation:** bounded beam search evaluates candidate action sequences against desired and avoided outcomes.
- **Safety-by-Outcome:** plans that technically reach a goal but also produce an explicitly avoided state are penalized.
- **Bounded Search:** depth is capped at 4, branching at 8 and beam width at 12.
- **Prediction Audit:** observed outcomes are compared with predicted states.
- **Adaptive Replanning:** when prediction accuracy falls below threshold or too many unexpected facts appear, planning restarts from the observed state.
- **Android proof:** `/world` runs the V37–V44 deliberation suite natively.

The world model is a compact structured simulator over explicit facts and transitions. It improves prospective reasoning and planning, but is not yet a learned high-dimensional simulator of unrestricted real-world environments.


## Inference V45–V60 — Metacognitive Self-Directed Intelligence

This block adds bounded self-monitoring and self-directed reasoning control on top of BIA's causal, autonomous-knowledge and world-model layers.

- **Metacognition:** estimates certainty, conflict, complexity and evidence gaps, then selects Answer / SeekEvidence / Deepen / Hold.
- **Self-Calibration:** tracks recent prediction confidence vs. correctness, computes bias/Brier score and adjusts later certainty.
- **Active Evidence Seeking:** generates a bounded evidence request for support, opposition, missing links or fresh observations when confidence is insufficient.
- **Recursive Deliberation:** allows up to four refinement passes and stops early when marginal reasoning gain collapses.
- **Self-Directed Compute:** routes reasoning to Instant / Normal / Deep / EvidenceFirst based jointly on uncertainty and device battery/thermal/load/memory pressure.
- **Cross-Domain Skill Transfer:** extracts a small structural action pattern from one WorldModel and maps it onto another domain using utility-sign/effect structure.
- **Android proof:** `/maxintel` runs the V45–V60 suite natively.

All new loops are strictly bounded. These mechanisms raise BIA's ability to regulate its own reasoning, but do not make it equivalent to a frontier-scale foundation model or establish unrestricted AGI.


## Inference V61–V80 — Learned Semantic Intelligence

This block reduces dependence on fixed semantic markers while keeping BIA mobile-bounded and native.

- **Compact semantic embedding:** 32-dimensional deterministic word/subword hashed vectors.
- **Latent memory:** up to 128 semantic items with online blending and bounded retention.
- **Learned relation prototypes:** Causes / Enables / Inhibits prototypes are updated online from high-confidence parsed examples.
- **Latent fallback:** when the rule parser yields no clause, BIA may infer a relation from learned latent prototypes with discounted confidence.
- **Vector semantic retrieval:** provenance-backed KnowledgeLedger records can be recalled by compact vector similarity when exact token overlap returns nothing.
- **Semantic compression:** related phrases are merged into at most 32 online centroids to limit memory growth.
- **Hybrid reasoning:** learned latent relations feed back into the explicit causal WorldGraph, preserving inspectable causal reasoning.
- **Android proof:** `/semantic` runs the V61–V80 learned-semantic suite natively.

This is not a pretrained language model: the encoder is small, deterministic and online-learned. It improves paraphrase tolerance and semantic retrieval inside the defined proof families, but does not provide unrestricted language understanding comparable to frontier foundation models.


## Inference V81–V100 — Continual Semantic Learning & Generative Cognition

This milestone closes the loop between learned semantics, durable concepts, explicit reasoning and generated responses.

- **Continual semantic learner:** bounded online concept vectors with decreasing update rates to reduce semantic drift.
- **Anchor-based anti-forgetting:** important concepts can be anchored and restored when later updates cause excessive drift.
- **Concept composition:** up to eight semantic parts can be combined into one compact compositional vector.
- **Latent-symbol bridge:** compact semantic vectors remain linked to inspectable symbolic IDs/labels.
- **Semantic consolidation:** selected important concepts are anchored and repaired in a bounded consolidation pass.
- **Reasoning-conditioned generation:** causal, opposing, contradictory and counterfactual results are rendered according to evidence depth, confidence and uncertainty rather than the old fixed answer templates.
- **Uncertainty-aware wording:** strong support, weak support and contradiction produce different response stances.
- **Android proof:** `/continual` runs the V81–V100 suite natively.
- **App version:** 1.0.0.

The response generator is a small deterministic cognitive surface planner, not a pretrained autoregressive language model. It demonstrates reasoning-conditioned generation and continual semantic stability inside BIA's bounded architecture; it does not establish unrestricted human-level language generation or AGI.


## Inference V101–V128 — Autonomous Cognitive Loop

This milestone connects BIA's reasoning, metacognition, evidence seeking, generative cognition and authority controls into one bounded self-review loop.

- **Internal question agenda:** conflict, unknown answers, complexity and evidence gaps create bounded internal questions such as missing cause, counter-evidence and alternative explanation.
- **Answer critic:** every candidate answer is scored for evidence sufficiency, contradiction risk and overconfidence risk before final rendering.
- **Bounded cognitive loop:** QUESTION → EVIDENCE → DELIBERATE → CRITIQUE → STOP/REVISE runs for at most four passes and holds no more than eight internal questions.
- **Real chat integration:** causal answers now pass through autonomous self-review before being rendered to the Android user.
- **Confidence-aware revision:** unresolved conflict lowers confidence and can request more evidence instead of forcing a strong conclusion.
- **Idle cognition scheduler:** when battery/thermal/load/memory conditions are healthy and no external action is pending, BIA may schedule memory consolidation, anchor rehearsal or hypothesis reevaluation.
- **Authority isolation:** idle/autonomous cognition is explicitly disabled while an external device action is waiting for user approval; the loop never bypasses the existing authority gate.
- **Android proof:** `/loop` runs the V101–V128 proof suite natively.
- **App version:** 1.28.0.

The autonomous loop is bounded self-review over BIA's existing structured cognition. It does not grant unrestricted external autonomy, does not bypass user approval for side effects, and does not establish unrestricted AGI.


## V129 — Evidence-grounded review and query-scoped causal reasoning

This increment fixes measured correctness gaps in the V101 review and causal chat path:

- Self-review cannot increase evidence sufficiency, erase conflict, or resolve a question merely by repeating a pass. With no evidence provider attached, it stops after one review and requests evidence; a later observation/query can produce a new verdict.
- Unknown and contradictory answers always require evidence. Review confidence is capped by the answer, assessment and uncertainty. Each agenda belongs to its current query, preventing stale targets from filling its eight slots.
- Causal A → B questions now search from A; stronger unrelated parents of B cannot replace A or create a spurious conflict about A. Paths may start inside a longer causal chain.
- Query confidence uses the strongest supporting/opposing path, avoiding noisy-OR inflation when multiple paths share evidence. This deliberately does not assume independent sources.
- Mobile chat routes unknown causal questions through review and explicit abstention, and supported/opposed wording respects the review confidence ceiling.
- Search remains bounded by configured depth (maximum 8) and beam (maximum 32). Beam pruning can still miss paths in dense graphs; this is not exhaustive reasoning.
- `cargo test --release --test grounded_cognition` covers repeated review, missing evidence, new evidence, stale agendas, distractors, shared paths, cycles, depth limits and mobile unknown replies. The source-specific test varies 128 graph fixtures in identity, chain depth and strength. Android `/loop` now also checks that self-review cannot fabricate evidence.
- Android app version: 1.29.0.

These are targeted improvements in BIA's explicit causal representation, not evidence of general-purpose or frontier-level intelligence. No on-phone latency improvement is claimed without device measurements.

## V130 — Multi-document causal evidence search

Causal chat now follows explicit concept links across local provenance records before
rendering an answer. It can recover missing A → … → B chains whose intermediate
documents share no query keywords, and inspect opposing evidence even when the active
world already supports a claim. Retrieval uses a temporary graph; it does not train
on the retrieved text, mutate durable facts, or execute commands found in documents.

- Parse each eligible excerpt once, then traverse source-linked clauses up to six passes.
- Preserve imported record IDs; source confidence discounts new edge strength.
- Re-reading an existing edge does not reinforce it; shared paths do not establish independent evidence.
- Hard limits: 96 records, 2,048 Unicode characters per excerpt, 12 clauses per record,
  512 candidate clauses, 128 imported clauses and 256 visited concepts. Oversized
  excerpts are skipped instead of asserting facts from truncated sentences.
- Device pressure reduces the pass budget; low-memory stillness skips document search.
  The response discloses when document search was budget-limited.
- Android `/evidence` runs the native 128-case evidence evaluation, including a
  top-three lexical retrieval baseline, conflict detection and durable-memory isolation.
- Android version: 1.30.0. Existing counterfactual retrieval remains unchanged.

The benchmark varies chain depth (2–6), source quality and identifiers, with reversed
document order and 24 distracting records per case. It measures this bounded family,
not unrestricted language understanding or a general intelligence score. No phone
latency claim is made from desktop/CI timing. Rules, existing graph provenance and
retrieval/search caps still limit correctness outside these cases.

## V131 — Integrated language, source revision, feedback, planning and transfer

One bounded chat coordinator now connects the existing BIA semantic reasoner,
cognitive review and world-model planner. See [Vietnamese command guide](docs/V131_GUIDE.md).

- Additional Vietnamese causal paraphrases and inverse-cause wording; recognized
  negation/uncertainty cannot fall through to positive latent relation learning.
- Named observations/reports/hypotheses, explicit correction and withdrawal;
  `Hỏi:` recomputes its answer from currently active managed sources and cites
  the sources examined. Hypotheses are excluded from factual evidence.
- Conservative single-topic pronoun resolution, with clarification on ambiguity.
- Declarative skill simulation with preconditions/outcomes, goal/avoid constraints,
  user-confirmed success/failure feedback and replanning away from failed skills.
- Shared-outcome generalization to a new named case only after two distinct examples;
  counterexamples/conflicting outcomes block transfer, and results remain hypotheses.
- Bounded cognition journal persisted through existing mobile continuity; malformed
  restore is atomic and never invokes device actions.
- Planner ranks before branch pruning and prevents newly introduced avoid-facts.
- Android 1.31.0 adds `/integrated`, a native 32-session combined evaluation.

The managed-source commands are an explicit workspace within BIA, not an imported
model/backend. `Hỏi:` does not silently mix legacy learned graph edges into a corrected
source. Free-form language, unconstrained planning, genuine independent-source
verification and autonomous execution of declared skills remain outside this release.
