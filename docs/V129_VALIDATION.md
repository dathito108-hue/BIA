# V129: evidence-grounded cognitive review

Base: `8a078f5c6e6010416be48d4526d1bd2aeca044c1` (V101–V128).

## Reproduced failures

Ten integration tests using the existing public API were run against the unchanged base checkout. Nine failed and one passed. Failures covered fabricated evidence during repeated review, unknown answers treated as answerable, uncertainty caps, stale agendas, irrelevant causes replacing a queried source, reasoning from an internal node, correlated-path confidence inflation and mobile unknown replies. The new depth-limit test uses the new `infer_between` API and was excluded from the base comparison.

## Changes

- Review a fixed evidence snapshot once. No artificial score increments or automatic resolution; stop and request evidence when needed.
- Reset the bounded agenda per query and cap final confidence by both evidence and uncertainty.
- Search causal paths from the exact queried source. Aggregate maximum support/opposition conservatively because edge provenance does not establish independence.
- Route unknown mobile causal questions through explicit abstention. Rendering honors the review confidence cap, and path evidence counts edges rather than vertices.
- Strengthen Android `/loop` proof to reject fabricated critique improvements.

## Evaluation corrections

The V18 conflict fixture previously used `A causes C; D inhibits C` to claim a contradiction about A. It now also includes `A causes D`, making both opposing paths originate at A. The V21 composition fixture now uses a fresh graph with the previously learned alias so that an earlier direct A-to-C edge cannot satisfy a test intended to require a multi-hop chain. Existing acceptance thresholds were retained.

## Local evidence

- Release library tests: 87 passed.
- New integration tests: 11 passed, including 128 varied graph fixtures with 40 unrelated inhibiting parents each, chain depths 1–6 and varying strengths.
- Clippy all targets with warnings denied: passed.
- The search has configured depth <= 8 and beam <= 32; each next frontier holds at most beam + 1 paths before pruning. Review without new evidence requires one pass rather than repeated synthetic improvement.

These results measure correctness on explicit structured cases. They do not prove open-domain language intelligence, general causal discovery, exhaustive search or on-device latency. Graph capacity, beam pruning, learned-edge provenance and the bounded Vietnamese parser remain limitations. Android build/CI status is recorded on the pull request.
