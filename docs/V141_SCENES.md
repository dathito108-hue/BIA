# V141 — Multi-object procedural creative studio

Extends V140 on Android without replacing BIA cognition or adding external AI services.

## Delivered
- Scene editor: 1–12 objects, maximum 30,000 triangles. Box, sphere, solid decorative vase, cylinder, cone and torus; detail 8–48.
- Per-object XYZ translation, positive nonuniform scale, XYZ Euler rotation (X then Y then Z), colour, illustrative gloss and smooth/flat lighting. New objects, duplicates and deletion; last object cannot be removed.
- Local 20-step undo/redo during the activity session. Last applied scene persists; un-applied editor text and undo history do not persist. Apply before changing the selected object.
- Orthographic CPU renderer: depth buffer, outward-face culling, directional diffuse/specular lighting, optional averaged vertex normals. Camera orbit and pinch zoom; latest preview requests coalesce on a worker, with cancellation on activity teardown. Preview 384px; saved PNG 512/1024/2048px.
- Transparent PNG background; exported camera matches the saved scene. No PBR, shadows, UV/texture, neural generation, image reconstruction, sculpting, Boolean union, rigging or animation.
- ZIP: scene JSON, OBJ, MTL colour/basic material, ASCII STL, PNG and README. OBJ normals and index offsets cover all objects. STL discards materials and keeps separate shells; intersections are not fused. Geometry uses Y-up and arbitrary units; not a manufacturing guarantee.
- Scene JSON max 64 KB, bounded finite parameters and topology budgets. Legacy V140 3D parameter files import as single-object scenes; 2D stays in the original studio. Rejected import does not overwrite the scene.
- 2D studio adds seeded wave ribbons and triangular mosaic, output sizes up to 2048px and transparent PNG/SVG. Existing single-object exports remain supported. For those exports, shape parameters remain in V140 JSON; image size/transparency/camera are recorded in README, not imported from that JSON.
- Document picker controls exports; IO runs off UI thread. A failed export may leave a partial chosen file; no successful status is reported. In-flight export is not a durable background job across process death.

## Validation gates
Five added Android tests (expected total 28): transformed manifold topology and winding including torus genus, parameter/object/triangle bounds, depth order independence, transparency, camera and light changes, OBJ normal indices and MTL/STL correspondence, 2K PNG decode, valid SVG, seed variants, editor undo/redo, persistence and document IO/oversized rejection. Existing Rust and product/creative Android regression suites remain enabled. CI emits actual image and scene packs from the APK.

Physical S21 FE throughput and thermals are not measured by emulator tests. “Maximum” is not a claim of parity with unrestricted professional DCC or learned image generators.
