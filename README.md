# BIA — Buddhist-Inspired Intelligence Architecture

BIA is a new mobile-first AI architecture inspired by selected models of mind found in Buddhist philosophy, especially process-oriented analyses of cognition, the five aggregates, dependent arising, sense bases, and Yogācāra layered consciousness.

This is a computational architecture, not a claim that Buddhist doctrine is reducible to machine learning or that a machine has Buddhist consciousness.

## Design goal

Build one native architecture that can run entirely on a phone for inference, memory, planning, tool selection, continual adaptation, and bounded local learning.

## Core idea

BIA does not center intelligence on a permanent global "self" vector. Cognition is modeled as a stream of short-lived conditioned events. Each event is assembled from perception, affective relevance, recognition, formations/policy, and integrative awareness, then dissolves into the next event while leaving sparse memory traces.

The first reference core is **BIA-KSANA-1**.

### BIA-KSANA-1 components

- **Ayatana Router** — routes external and internal input streams into sparse active channels.
- **Skandha Cell** — factorizes each cognitive moment into five functional sub-states:
  - Rupa: encoded sensory/form features.
  - Vedana: valence, urgency, novelty and relevance.
  - Sanna: recognition and compact concept binding.
  - Sankhara: intentions, candidate actions and generative transformations.
  - Vinnana: transient integration state for the current cognitive moment.
- **Pratitya Graph** — sparse conditional dependency graph. Active factors update only when their conditions are present.
- **Santati Stream** — recurrent continuity across moments without a permanent self-state.
- **Alaya Seed Memory** — sparse long-term dispositions/traces that can be reactivated by context.
- **Manas Self-Model** — an optional, revisable working hypothesis about agent/body/goals; never the immutable center of the model.
- **Madhyama Budgeter** — balances latency, energy, memory use and reasoning depth for mobile hardware.

## Mobile constraints

BIA is designed around:
- recurrent constant-size working state rather than quadratic full-context attention;
- sparse top-k conditional activation;
- chunked streaming input;
- int8/int4-ready matrices and state;
- bounded memory retrieval;
- deterministic memory ceilings;
- native Rust/C/C++ compatible execution;
- Android ARM64/NEON as the primary deployment target.

## Repository milestone

**M0 — Process Core Foundation**

M0 defines the architecture contract and a dependency-free Rust reference implementation of one cognitive moment. It intentionally avoids importing Transformer, Mamba, SSM, or any previous project architecture.

Next milestones:
1. M1 — trainable Skandha projections and sparse Pratitya routing;
2. M2 — Alaya seed-memory storage/retrieval;
3. M3 — quantized ARM64 kernels;
4. M4 — tokenizer/byte and multimodal Ayatana adapters;
5. M5 — recurrent reasoning and action formation;
6. M6 — Android runtime and JNI;
7. M7 — on-device continual adaptation;
8. M8 — full mobile assistant integration.
