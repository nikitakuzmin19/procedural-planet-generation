# Level 2 — Noise-Based Terrain (Introduction Plan)

## Goal

Replace “random tiles” with a **continuous height field** produced by **procedural noise**.
Instead of each tile having its own unrelated dice roll, tiles get heights from one smooth function.

This level should make terrain look **natural**: smooth hills, gentle transitions, and visible regions.

## What you will build (end state)

A function that, for each tile `(x, y)`:

- computes noise sample coordinates `(nx, ny)`
- computes a height-like value `h` from noise
- maps `h` into a tile type (water/grass/mountains) using thresholds

## Core concepts (keep in mind, don’t overthink)

- **Noise**: a function that returns a number for coordinates `[noise: “given (x,y) -> output height value”]`
- **Height field**: the grid of height values `h[x][y]` you generate
- **Lattice / grid corners**: integer grid points used as anchors `[lattice: the invisible grid of integer points]`
- **Interpolation**: blending between corner values so nearby points are similar `[interpolation: “blend smoothly between two numbers”]`
- **Fade**: a smoothing curve that makes blends change gently across cell borders `[fade: “make smoothing not look like a seam”]`
- **Frequency**: controls “zoom” / feature size `[frequency: “how fast hills repeat across space”]`
- **Offset**: shifts the sampling position so your map doesn’t line up awkwardly `[offset: “move the noise world around”]`
- **Seed**: controls which world you generate `[seed: “deterministic randomness knob”]`
- **Normalization**: converting noise output into a predictable range `[normalization: “rescale output into something like [0,1]”]`

## Step-by-step plan (slow and verifiable)

### Step 1 — Build the minimum noise tools first (required)

Your current code has no noise yet, so first add these helper functions:

- `lerp(a, b, t)` [blend between two values]
- `fade(t)` [smooth blend weight]
- `hash2(ix, iy, seed)` [deterministic pseudo-random value at integer grid points]
- `noise2d(x, y, seed)` [uses 4 corners + interpolation]

Why this step exists:

- Without these helpers, `h = noise(nx, ny, seed)` cannot exist yet.

Deliverable:

- You can call `noise2d(x, y, seed)` and get a float-like height value.

### Step 2 — Keep your current loops, but swap the “source”

From your current code:

- Keep the nested loops `for y { for x { ... } }`
- Keep output chars (`~ . ^`)
- Replace random tile pick:
  - from: `n = rng.gen_range(0..3)`
  - to:   `h = noise2d(nx, ny, seed)`
- Then classify `h` with thresholds (`sea_level`, `mountain_level`)

Deliverable:

- You can print an ASCII map where neighboring cells look related.

### Step 3 — Confirm that noise is sampled at “positions”, not just integers

Noise is usually smooth because you sample it at non-integer coordinates.
So inside the loops do:

- `nx = x * freq + offset_x`
- `ny = y * freq + offset_y`
- `h = noise2d(nx, ny, seed)`

Deliverable:

- If you temporarily set `freq` to something tiny or huge, you should see the terrain change size/scale.

Quick mental check:

- If you accidentally sample at pure integers all the time, you often get less smooth / more “grid-like” results.

### Step 4 — Make noise output visible before classifying tiles

Before mapping to `~ . ^`, print/debug the raw height values.
Options:

- Print a “grayscale” ASCII by thresholding `h` into ~10 buckets.
- Or print a single row of `h` values for fixed `y` while varying `x`.

Deliverable:

- You can clearly see that `h(x)` changes gradually (not random jumps).

### Step 5 — Learn “corners + blending” with a mental model

Your typical noise2D implementation is:

1. Find the unit square that contains `(x, y)`
2. Read pseudo-random values at the 4 corners of that square
3. Blend between corners using:
  - `lerp` (linear blend) `[lerp: “blend between a and b by t”]`
  - `fade` (smooth the blend weight) `[fade: “make blending gentler near edges”]`

Deliverable:

- You can explain to yourself: “nearby tiles share corner anchors, so they correlate.”

### Step 6 — Normalize height so thresholds make sense

Different noise functions return values in different ranges.
You want `h01` so thresholds are stable:

- Example concept: convert noise output to `[0,1]`
- Then use:
  - `water if h01 < sea_level`
  - `mountain if h01 > mountain_level`

Deliverable:

- Tuning `sea_level` and `mountain_level` behaves predictably.

### Step 7 — Tune the knobs (the only knobs you need right now)

Tune in this order:

1. `freq` (terrain scale) `[frequency: “how large bumps are”]`
2. `sea_level` (how much water) `[sea level: height threshold for water]`
3. `mountain_level` (how much mountains) `[mountain threshold]`
4. `offset_x/y` (changes alignment of features) `[offset: shift noise sampling]`
5. `seed` (new world) `[seed: different deterministic randomness]`

Deliverable:

- You can produce 3 different-looking maps quickly by changing only seed/freq/thresholds.

## Debug checklist (when it doesn’t look right)

- If terrain looks like salt-and-pepper:
  - you might be effectively sampling noise at integer corners too often
  - increase variety via `freq` and/or add offsets
- If terrain looks overly smooth / bland:
  - increase `freq` (more detail)
  - consider later (Level 2.5 / Level 3) adding octaves [octaves: multiple zoom levels combined]
- If thresholds produce almost all one type:
  - adjust `sea_level` and `mountain_level`

## Minimal equations / recipe (what you’ll actually do in the loops)

- `nx = x * freq + offset_x`
- `ny = y * freq + offset_y`
- `h = noise2d(nx, ny, seed)`
- `tile = classify(h)` using thresholds (sea + mountain levels)

## Done when…

- Re-running the program produces a *smooth* terrain-like map (not random noise per tile).
- Changing `freq` clearly changes the scale of hills/mountains.
- You can explain: “we generate heights from one function sampled across the grid, then threshold heights into types.”

