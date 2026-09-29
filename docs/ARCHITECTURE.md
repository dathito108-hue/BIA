# BIA-KSANA-1 Architecture Contract

## 1. Philosophical inspiration → computational abstraction

BIA uses Buddhist philosophical models as design metaphors and structural inspiration, not as scientific proof.

| Buddhist concept | BIA abstraction |
|---|---|
| momentary conditioned experience | discrete recurrent cognitive moment (Ksana) |
| five aggregates | five functional sub-states in each moment |
| dependent arising | sparse condition-dependent update graph |
| six sense bases | modality and internal-state routing |
| mind-stream / continuity | recurrent state transition without permanent self token |
| store consciousness / seeds | sparse long-term trace memory |
| afflicted manas / self-grasping | explicit revisable self-model, confidence bounded |
| middle way | runtime resource controller balancing quality and cost |

## 2. State

At time t the model carries:

```
S_t = {
  santati: H_t,
  alaya_query: M_t,
  self_model: Q_t,
  budget: B_t
}
```

The state is not treated as a permanent identity. It is a transient computational support for the next event.

Each cognitive moment computes five sub-states:

```
R_t = f_r(x_t)                    // rupa
V_t = f_v(R_t, H_{t-1})           // vedana
N_t = f_n(R_t, H_{t-1})           // sanna
K_t = f_k(N_t, V_t, H_{t-1})      // sankhara
C_t = f_c(R_t, V_t, N_t, K_t)     // vinnana
```

Then sparse conditional integration:

```
G_t = TopK(conditions(x_t, H_{t-1}, M_t), k)
H_t = decay(H_{t-1}) + sum_{e in G_t} gate_e * update_e(C_t, H_{t-1})
```

Only a small subset of conditional edges is active per moment.

## 3. No permanent self center

BIA deliberately separates useful self-modeling from the core recurrent state:

```
Q_t = revise(Q_{t-1}, evidence_t, confidence_t)
```

Q may encode body state, capabilities, user-assigned role, active goals, and tool permissions. It must be revisable and may be discarded/reconstructed.

## 4. Alaya Seed Memory

Long-term memory is modeled as sparse traces:

```
seed_i = {
  key,
  value,
  strength,
  recency,
  context_signature,
  action_credit
}
```

Retrieval is bounded:

```
M_t = TopKSeed(similarity(context_t, key_i) * strength_i, k_m)
```

Learning changes strengths and representations rather than replaying an unlimited context window.

## 5. Mobile complexity target

For hidden width d, active graph degree k, and retrieved memories m:

- recurrent state update: O(d)
- active conditional routing: O(k d), k << d
- memory retrieval: bounded top-k/indexed search
- working memory: O(d + m d)
- sequence processing: streaming; no O(n²) attention requirement

## 6. Initial deployment profiles

### BIA-S
- d = 256
- 8 process blocks
- active edges per block <= 4
- int8 state / int4 weights target
- intended for continuous mobile assistant use

### BIA-M
- d = 512
- 16 process blocks
- active edges per block <= 8
- int8 state / int4 weights target
- intended for stronger local reasoning

### BIA-L Mobile
- d = 768
- 24 process blocks
- aggressive sparse activation
- int8 state / int4 weights target
- intended for high-memory phones

Exact parameter counts are not frozen at M0; they depend on projection sharing, vocabulary/adapters, and memory modules.

## 7. Training objective

A future trainable BIA combines:
- next-event / next-symbol prediction;
- latent state prediction;
- contrastive recognition;
- action-value / outcome prediction;
- memory write/read consistency;
- self-model calibration;
- energy-aware sparse-routing penalty.

The architecture does not require a Transformer teacher at inference time.

## 8. M0 invariants

1. No Transformer attention block.
2. No imported prior-project model core.
3. No global permanent self embedding.
4. Cognitive state is recurrent and bounded.
5. Conditional computation is sparse by contract.
6. Long-term memory is external to the fixed recurrent state.
7. Every production implementation must expose hard limits for RAM, compute and retrieval count.
