# V140 — Xưởng ảnh & 3D

Adds a deterministic, bounded procedural graphics skill to BIA's Android execution layer. No neural image model, external AI service, downloaded weights, or architecture substitution. Existing cognition and product compiler are unchanged.

## Implemented
- Mandala polygons and seeded layered landscape: 1024×1024 PNG plus editable SVG.
- Box, ellipsoid, and lathed decorative vase: indexed outward triangle meshes and OBJ export. Vase is capped solid geometry, not a hollow vessel.
- Parameters: signed 64-bit seed, detail 8–48, RGB colour, height ratio 0.5–3.
- Native Canvas orthographic shaded preview; touch drag rotates model. Preview is illustrative, not physically based rendering.
- Persist latest generated parameters; reopen exported `project.bia-art.json` (bounded to 4 KB). One draft, not a project library; no arbitrary script import.
- User-selected ZIP export through Android document picker on a worker thread: project JSON, PNG, SVG or OBJ, explanatory README. OBJ contains geometry only; colour remains in PNG/project, not an MTL file. Exported PNG uses canonical camera, not the current dragged angle.
- Export errors shown to user; a failed write may leave a partial file at the chosen destination. Cancel does not produce a successful export claim.

## Validation
Android instrumentation checks finite geometry, non-degenerate triangles, closed manifold edges, consistent outward winding, Euler characteristic, visible rendered models, PNG decoding/dimensions, ZIP contents, parameter roundtrip, invalid input rejection and persisted activity restart. Document import/export and oversized input are exercised through the activity result handler. Four additional tests, expected total 23. Test artifacts include two image packs, one OBJ pack, and a rendered preview.

## Boundaries
Not text-to-image synthesis, photorealism, image-to-3D reconstruction, arbitrary object generation, sculpting, rigging, animation, CAD tolerances or automatic sales. No new spending, account access, publishing or trading actions. Generated work needs domain review before commercial manufacturing or 3D printing. No performance claims on a physical S21 FE until measured there.
