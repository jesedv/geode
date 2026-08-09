# Geode — Hyper-Catalan Polynomial Solver

## One-liner
Solve general-degree polynomials (quintics, sextics, and beyond) using Hyper-Catalan series — clean rational arithmetic, no radicals, no complex numbers, exact closed-form solutions. The "Geode" structure reveals an infinite algebraic array.

## The Hard Math
- **Abel–Ruffini (1824)** — quintics and higher cannot be solved with radicals.
- **Galois theory** — why radicals don't work; how hyper-Catalan series bypass it.
- **Hyper-Catalan numbers $C_m$** — number of ways a polygon can be subdivided into triangles, quadrilaterals, pentagons, etc.
- **Multi-dimensional formal power series** $S[t_2, t_3, t_4, \ldots]$ — Wildberger–Rubine solution.
- **The Geode** — infinite algebraic array of coefficients revealed by the series.
- **Newton polytope, monomial orders, Möbius inversion.**
- **Triangle subdivisions / polygon dissections** — combinatorics.
- **Lagrange inversion** and **residue calculus**.

## The Real Problem
For 200+ years, since Abel–Ruffini, we've known that quintics and higher can't be solved with radicals. Existing options:
- **Numerical methods** (Newton, Durand–Kerner, Aberth) — give approximations, no closed form.
- **CAS** (Mathematica, SymPy) — return radicals when possible, fall back to numerical.
- **Bring–Jerrard / Tschirnhaus** — reduce to canonical form, still approximate.
- **Galois group computation** — symmetry, not roots.

Hyper-Catalan series gives a **closed-form, rational, exact** solution. Truncate at any precision; get a rational approximation. No radicals, no complex intermediates.

Real use cases:
- **Game physics** — quintic equations in cloth sim, soft-body dynamics, Bézier curve intersection.
- **Computer graphics** — polynomial curves, surfaces, ray-marching implicit surfaces, geometric algebra.
- **Special functions** — polynomial systems in mathematical physics, Kepler problem, optics.
- **Symbolic algebra** — fundamental new method for CAS.
- **Cryptography** — analysis of polynomial-based primitives (Rainbow, MAYO, MAYO-1).

## Tech Stack
- **Rust** — core series engine.
- **`num-rational` / `num-bigint`** — arbitrary-precision rationals.
- **`symbolica` or `polynomial` crate** — polynomial arithmetic.
- **WASM** — interactive solver in browser.
- **TypeScript** — UI; **KaTeX / MathJax** — render solution.
- **SymPy FFI (optional)** — export to SymPy / Mathematica.

## Repository Layout
```
geode/
├── Cargo.toml
├── crates/
│   ├── geode-poly/         # polynomial arithmetic
│   ├── geode-catalan/      # hyper-Catalan numbers
│   ├── geode-series/       # multi-dim power series
│   ├── geode-solver/       # main solver
│   ├── geode-simplify/     # algebraic simplification
│   ├── geode-geode/        # The Geode data structure
│   └── geode-wasm/         # wasm-bindgen
├── ui/                     # solver playground
│   ├── src/lib/input/      # polynomial editor
│   ├── src/lib/solve/      # solve + display
│   └── src/lib/the-geode/  # visualize the Geode
├── examples/               # canonical quintics, sextics
└── docs/
    └── theory.md
```

## Build & Test
- `cargo test`
- `cd ui && npm run dev`
- `cargo bench`  (vs SymPy `solve` on quintics)
- `npm run test:regress`  (replay all known exact solutions)
- `npm run test:the-geode`  (verify Geode structure)
- `npm run test:roundtrip`  (solution → CAS form → re-solve, exact equality)

## Conventions
- All rational arithmetic; no floats in the solver path.
- Series truncation depth reported with every solution.
- Every solved polynomial exportable as LaTeX / SymPy / Mathematica.
- Citations to Wildberger & Rubine papers in every doc.
- Reproducible (deterministic).

## Hard Constraints
- Quintic solved in < 100 ms with depth-10 truncation.
- Sextic in < 1 s with depth-15 truncation.
- WASM bundle ≤ 4 MB gzipped.
- Exact rational answers (or documented truncation level).
- Reproducible (deterministic).

## Non-Goals
- General system of polynomial equations (v1: univariate).
- Numerical root finding (we give closed-form rational).
- Complex roots (v1: real rational only).
- Generic Gröbner-basis computation.

## Open Questions
- LaTeX / SymPy / Mathematica export format priority.
- Visualization of the Geode (interactive 3D / 4D viz).
- Real-time solving in JS apps (server tier for huge polynomials).
- Multivariate extension (v2): systems of polynomial equations.
- Licensing: AGPL for solver, dual commercial for convenience.

## References
- Wildberger, "A Rational Approach to Trigonometry" (2005).
- Wildberger, "The Geode" (multiple papers, 2010s–2020s).
- Wildberger, Rubine, "The Hyper-Catalan Series Solution to Polynomial Equations" (2024).
- Abel, "Mémoire sur les équations algébriques" (1824).
- Galois, "Mémoire sur les conditions de résolubilité des équations par radicaux" (1846).
- Knuth, *The Art of Computer Programming, Vol. 4A* (combinatorial enumeration).
- Stanley, *Enumerative Combinatorics* (Catalan numbers, polygon dissections).
- Bring, "Meletemata quaedam mathematica" (1786 — reduction to canonical quintic).
- Jerrard, "An Essay on the Resolution of Equations" (1859).
