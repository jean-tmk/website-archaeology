# Website Archaeology

An interactive excavation of four buried website eras. Drag across the present-day surface, catalogue interface fragments, and descend from the optimized web of 2026 to the handmade source of 1996.

The excavation model, stratigraphic records, artifact catalogue, preservation scoring, and reconstruction rules live in Rust. The browser layer provides the canvas renderer and exhibit interface.

## Field controls

- Press and drag to excavate.
- Change brush diameter for broad or precise removal.
- Use Survey Sweep to expose a transect quickly.
- Recover three artifacts from each era.
- Expose 68% of a layer to descend.

## Run locally

Open `index.html` with any static web server.

## Structure

- `src/lib.rs` — Rust excavation engine and archival model
- `app.js` — browser interaction and canvas renderer
- `styles.css` — exhibit and buried-site visual systems
- `index.html` — semantic interface shell
