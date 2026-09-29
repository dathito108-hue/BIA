# BIA-DCA Architecture Contract

## 1. Definition

BIA-DCA (Dharma Cognitive Architecture) treats intelligence as a continuously renewed process of **world contact, conditioned interpretation, contemplation, choice, consequence and experiential consolidation**.

It is deliberately not specified as a neural-network family. Numerical functions are allowed as tools, but no dense weight matrix, token stream, attention operator, SSM recurrence or other existing model primitive is the architectural center.

## 2. Vietnamese Buddhist design inspirations

| Concept | Computational role |
|---|---|
| Duyên khởi | relations become active only when relevant conditions co-occur |
| Ngũ uẩn | separate form, relevance/feeling, recognition, formation and awareness roles |
| Lục căn/lục trần | interchangeable perception/action gates |
| Sát-na / tâm lưu | cognition is an event stream, not a permanent self-state |
| Vô ngã | self-model is optional, revisable metadata, never the identity of the core |
| Huân tập / chủng tử | experience leaves bounded reusable traces |
| Trung đạo | quality/latency/energy/memory trade-off is an architectural controller |
| Trúc Lâm / nhập thế | intelligence is completed by perception → understanding → action → consequence |

These mappings are engineering abstractions, not assertions that Buddhist doctrine is equivalent to computer science.

## 3. Tam Thiên world model

BIA represents the currently known world in three scalable levels:

- **Tiểu Thiên**: immediate phenomena and local objects.
- **Trung Thiên**: situations, environments, apps, conversations, tasks.
- **Đại Thiên**: durable systems, concepts, social/world structures and long-horizon context.

The implementation stores bounded `Phenomenon` nodes and explicit `Relation` edges. The active Cảnh is retrieved from this graph; the entire world never needs to reside in the active cognitive moment.

## 4. Cognitive matrix

The "matrix" is logical and sparse rather than a mandatory dense tensor.

```
M_t = {
  canh: active phenomena,
  duyen: active causal/context relations,
  tho: salience/urgency,
  tuong: recognized meanings,
  thuc: current integrated awareness event,
  quan: hypotheses under examination,
  tri: evaluated understanding,
  hanh: candidate intention
}
```

Only active entries exist. This keeps working memory bounded.

## 5. One cognitive cycle

```
1. CANH  : select relevant phenomena
2. XUC   : connect input gate with current world state
3. THO   : estimate salience/urgency/novelty
4. TUONG : recognize meanings from world + seed memory
5. THUC  : form the current cognitive event
6. QUAN  : activate causal conditions and competing hypotheses
7. TRI   : reinforce/discount hypotheses using evidence and outcomes
8. HANH  : select or abstain from an intention
9. QUA   : observe outcome
10.HUAN  : consolidate useful experience as seeds/relations
```

Abstention is a valid result. Under resource or safety pressure the system may enter **Tĩnh** and perform no deliberative action.

## 6. Duyên graph

A relation contains:
- source and target phenomenon ids;
- relation kind (causes/enables/inhibits/contains/similar/follows/goal-relevant);
- strength;
- confidence.

Reasoning is graph activation plus bounded hypothesis revision. There is no architectural requirement to generate hidden reasoning tokens.

## 7. Chủng tử memory

A seed stores:
- a compact experience signature;
- learned meaning/action category;
- strength;
- utility;
- repetitions;
- last-seen time.

Repeated similar experiences merge. When capacity is full, weak/low-utility traces are evicted. This gives learning without requiring global retraining.

## 8. Trung Đạo compute controller

Device state includes battery, thermal pressure, load and available memory. Task state includes importance and uncertainty. The budgeter chooses:

- **Tĩnh**: event monitoring only;
- **Nhanh**: one shallow contemplation;
- **Thường**: several contemplation cycles;
- **Sâu**: bounded deeper reasoning.

All capacities remain explicit and bounded.

## 9. Safety/action contract

Intentions carry expected benefit, harm, reversibility and confidence. A production action layer must separately enforce permissions and irreversible-action approval; the cognitive core cannot grant itself external authority.

## 10. Portability contract

The architecture must compile without AI-framework dependencies and expose capacity limits. Device-specific acceleration may optimize primitives but may not redefine cognition. A BIA instance on a weak device and one on a powerful machine share the same world, memory, relation and cognitive-moment formats.

## 11. What BIA-DCA is not

It is not complete AGI merely because the runtime exists. General intelligence requires learned world knowledge, perception/language grounding, robust planning, action feedback, evaluation and extensive training/experience. This repository establishes the canonical architecture on which those capabilities can be built without changing BIA into an existing model family.
