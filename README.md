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

The repository now contains a dependency-free Rust **BIA-DCA reference runtime** with portable checked persistence and incremental meaning formation. It is a functioning architecture kernel, not yet a fully developed general intelligence. The next engineering work is the Vietnamese language gate, perception adapters, deeper contemplation/planning, action interfaces, benchmarking and Android packaging.

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/ROADMAP.md](docs/ROADMAP.md).
