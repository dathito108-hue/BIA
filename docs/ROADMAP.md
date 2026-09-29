# BIA-DCA Roadmap

## D0 — Canonical Dharma Core
Tam Thiên WorldGraph, Duyên relations, bounded CognitiveMoment, Chủng tử memory, Trung Đạo device budgeter and Quán–Trí–Hành loop.

**Acceptance:** dependency-free Rust core; deterministic ceilings; tests; no imported model-family core.

## D1 — Persistent Dharma Memory ✅
Binary portable storage for phenomena, relations, seeds, confidence and provenance; crash-safe append/checkpoint/restore.

**Acceptance:** same memory file can move between Android/desktop builds; bounded corruption recovery.

## D2 — Meaning Formation ✅
Incremental concept formation from repeated phenomena, relation induction, contradiction tracking, confidence calibration and forgetting.

**Acceptance:** learns new categories/relations from experience without global weight retraining.

## D3 — Language as a Gate, Not the Mind ✅
Vietnamese-first byte/phoneme/word observation and expression layers that translate language into/from BIA phenomena and intentions.

**Acceptance:** internal reasoning remains phenomenon/relation based; language adapter can be disabled.

## D4 — Multi-Cảnh Perception ✅
Image/audio/sensor adapters produce normalized phenomena; cross-modal identity binding occurs in the Duyên graph.

**Acceptance:** adapters remain replaceable and do not alter the BIA core.

## D5 — Deep Quán and Planning ✅
Counterfactual branches, causal-chain search, goal decomposition, uncertainty-directed observation and explicit stop/abstain policy.

**Acceptance:** bounded cycles; inspectable hypotheses; deterministic resource caps.

## D6 — Cư Trần Action Fabric ✅
Typed tools and device actions, permission boundaries, consequence receipts and world feedback.

**Acceptance:** cognition proposes; authority layer permits/denies; irreversible actions require explicit policy approval.

## D7 — On-device Huân Tập ✅
Experience consolidation, causal-credit updates, relation revision, rollbackable skill packages and local evaluation.

**Acceptance:** learning is measurable and reversible; base contracts cannot silently mutate.

## D8 — Universal Runtime ✅
no_std-capable subset where practical, ARM64/x86_64/WASM targets, Android wrapper, persistence and benchmark suite.

**Acceptance:** one canonical data/runtime contract across device classes.

## D9 — BIA Intelligence Curriculum ✅
Progressive environment curriculum covering language, physical/common-sense relations, tool use, planning and self-correction.

**Acceptance:** benchmark improvements come from BIA learning mechanisms rather than substituting an external foundation model.

## D10 — Integrated Mobile BIA ✅
Offline-first app with memory, Vietnamese dialogue, perception, planning and permission-gated actions.

**Acceptance:** install-and-run mobile package; cloud optional; measured RAM/latency/energy tiers.

## Highest-level completion criterion

BIA is considered mature only when all of the following are demonstrated empirically:
1. portable cognition across device classes;
2. continual learning without catastrophic loss of validated skills;
3. grounded language/perception/action loop;
4. causal and counterfactual reasoning;
5. calibrated uncertainty and abstention;
6. bounded resource usage;
7. transparent, inspectable memory and hypotheses;
8. no hidden dependence on an LLM/Transformer/SSM/Mamba runtime.


### D1/D2 implementation note

D1 provides a versioned binary snapshot format with checksum verification, bounded decode limits, portable world/seed restoration and atomic checkpoint replacement.

D2 provides bounded emergent concepts, prototype consolidation, contradiction-based confidence revision and temporal relation induction. These mechanisms operate on BIA phenomena and relations rather than on a hidden external model.


### D3–D6 implementation note

D3 introduces a Vietnamese language gate that maps text into BIA phenomena/intents and expresses concepts without making language the cognitive core.

D4 introduces Multi-Cảnh binding across replaceable perception packets.

D5 introduces bounded Deep Quán planning and counterfactual scoring.

D6 introduces a typed Cư Trần authority policy separating cognition from permission: observation/reversible actions may pass policy while external-write and irreversible actions require authorization or are denied.


### D7–D10 implementation note

D7 adds measurable, rollback-friendly skill deltas and causal-credit scoring. Promotion is based on observed evaluation gain rather than silent mutation.

D8 adds device-capacity profiles for Android ARM64, Linux ARM64/x64, Windows x64 and WASM while preserving one BIA architecture contract.

D9 adds a curriculum scoring layer covering language, commonsense, causality, planning, tool use and self-correction with accuracy, calibration and efficiency metrics.

D10 adds an offline mobile integration shell connecting Vietnamese language perception, BIA cognition and permission-gated actions.
