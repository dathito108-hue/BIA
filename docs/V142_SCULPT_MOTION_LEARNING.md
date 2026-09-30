# V142 — consolidated native sculpt, motion and image-learning pilot

## Native tools
- Replayable local-space push/indent, grab and Laplacian smoothing stamps, optional X symmetry. Numeric centres or tap a visible selected surface. Each tap applies one stamp; this is not continuous freehand sculpting. 24 stamps/object, 48/scene.
- Midpoint triangle subdivision with shared-edge vertices, at most twice; existing 30,000-triangle scene cap applies. No remesh, Boolean fusion or self-intersection guarantee.
- Scene schema BIA_SCENE_2 preserves geometry edits; reads V140 parameter and V141 scene files. Original scene editor retains edits when changing object parameters, replaying them on the chosen primitive.
- Separate Motion editor preserves imported scene, strokes and per-object tracks, session undo/redo and durable project JSON. 8 keyframes/object, 0–20 seconds, millisecond time quantization. Translation offsets, additional Y-axis rotation and uniform scale multiplier. Shortest-path yaw interpolation. No bones, skinning, IK, physics, shape keys or deforming-mesh animation.
- Playback uses renderer completion to schedule the next wall-clock frame, without claiming a fixed frame rate. Backgrounding stops playback. Motion preview/PNG uses fixed base-scene framing, so translating objects actually move; large displacements can leave the frame.
- ZIP: GLB 2.0, editable project, start/middle PNG and README. GLB embeds sculpted geometry, normals, basic PBR material values and rigid TRS animation. One scene unit exports as one metre. Viewer lighting/camera differs from the native renderer. Official Khronos glTF Validator 2.0.0-dev.3.10 is a CI gate (zero errors/warnings).

## Own-image training pilot — deliberately limited
A CPU-executable, closed-vocabulary factor-learning experiment. No external model, pretrained weights, LLM, diffusion backbone, or replacement of BIA cognition. It is a small supervised atlas-factor learner, not a claim of a new general-purpose model architecture.

Nine 64×64 images are produced by BIA's renderer: sphere/box/torus × red/green/blue, fixed camera/light. Six off-diagonal combinations train average shade/alpha fields per shape and material colour values from labelled scene parameters. Three diagonal combinations are held out; never used to update the checkpoint. The baseline is the mean RGB image of the six training samples. Report full-image and foreground RGB MAE, manifest/splits, dataset SHA-256, timing and checkpoint reload equality. Since labels include material values and shape categories are predefined, this does not test open-language understanding or discovery of unknown shapes.

The binary checkpoint persists atomically and can produce only the supported compositional captions in the laboratory screen. Unsupported prompts are rejected. Raw learned output is 64×64; enlarging the UI does not imply learned 2K detail. Report flags photorealism_qualified=false, open_vocabulary_qualified=false and production_activation_authorized=false. No general image-generator quality, photorealistic capability, arbitrary 3D generation or autonomous production activation is claimed.

The ZIP includes the full self-generated dataset, provenance, checkpoint, held-out output and report. Re-run using the APK's image-lab button. This completes a reproducible pilot, not the full research/training programme needed for the user's photorealistic goal. Next work requires broader rights-cleared data, a validated BIA-native learned representation beyond fixed atlases, compute experiments and independent quality evaluation.

## Tests
Six added Android tests: sculpt replay/subdivision/limits and picking; keyframes and fixed framing; GLB structure and actual output; held-out learning/checkpoint/OOD rejection; motion editor undo, persistence and document export; laboratory training/generation/restart. Expected total: 34. Existing 176 Rust tests remain enabled. Actual GLB, animation project, rendered frames and image-trial ZIP are exported from instrumented APK execution.

Physical Samsung S21 FE memory, latency and thermals have not been measured. No claim of professional DCC parity. File export follows document-picker selection; process death can interrupt it and a partial destination file may remain.
