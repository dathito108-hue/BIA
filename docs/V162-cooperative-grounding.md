# V162 — Cooperative Grounding & Dialogue Repair

V162 adds a bounded repair layer inside the existing BIA dialogue core. It does not create a second language model or a separate AI core.

## Repair moves

BIA recognizes explicit grounding corrections such as:

- “Không, ý tôi là A gây ra B.”
- “Không phải X mà Y.”
- “Tôi hỏi X chứ không phải Y.”

The repair parser first tries to recover a complete relation. If the user is correcting one entity, the current relation is used to determine whether that entity is the **nguyên nhân** or **kết quả**.

## Re-anchoring

After a successful repair:

1. the current discourse relation is corrected;
2. the last-question pointer is moved to the repaired relation;
3. the dialogue goal is re-anchored;
4. reasoning is run again from current sources;
5. later turns such as “Có chắc không?” or “Chốt lại” operate on the repaired relation.

The old relation is not silently carried forward as the active goal.

## Ambiguity safety

If the rejected entity is not grounded in the current relation, or appears in both roles, BIA asks which role should be changed instead of guessing.

## Authority safety

Grounding repair changes dialogue interpretation only. It cannot create a device action, executable plan, permission, or external side effect.

## Bounded state

GroundingState stores only a small repair counter and whether an unresolved repair remains. No unbounded conversation memory is introduced.

## Mobile

Android version is 1.62.0. The instrumentation suite is extended to 56 tests.

V162 remains inside the single BIA core and does not introduce LLM, Transformer, SSM, Mamba, or another AI backend.
