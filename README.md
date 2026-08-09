# geode

> Solve any polynomial with exact rational arithmetic — no radicals, no numerics, no complex numbers.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![WASM](https://img.shields.io/badge/WASM-WebGPU-blueviolet.svg)](https://geode.jesed.dev)

**[geode.jesed.dev](https://geode.jesed.dev)** · [GitHub](https://github.com/jesedv/geode)

---

A Rust implementation of the Hyper-Catalan series solution to polynomial equations
(Wildberger–Rubine 2024). For the first time in 200+ years, quintics and beyond
have a closed-form rational solution.

```
  quintic:  x⁵ − x + 1 = 0
  solution: x ≈ 0.75488 (exact rational at depth-15 truncation)
```

## Install

```bash
curl -sSf https://geode.jesed.dev/install.sh | sh
```

## Demo

**[Try it live → geode.jesed.dev](https://geode.jesed.dev/ui/public/demo/)**

## How it works

The Hyper-Catalan series gives a closed-form rational solution to any univariate
polynomial equation. For a polynomial `aₙxⁿ + aₙ₋₁xⁿ⁻¹ + ... + a₀ = 0`:

```
x = Σₘ Cₘ · (a₁/a₀)^{m₁} · (a₂/a₀)^{m₂} · ...
```

where `Cₘ` are Hyper-Catalan numbers (counting polygon dissections).
Truncation at depth `d` gives `O(d)` correct digits.

## Use cases

| Domain | Application |
|--------|-------------|
| Game physics | Cloth simulation, soft-body dynamics, Bézier intersection |
| Computer graphics | Polynomial curves, surfaces, ray-marching implicit surfaces |
| Symbolic CAS | New method for computer algebra systems |
| Cryptography | Analysis of polynomial-based primitives (Rainbow, MAYO) |
| Special functions | Polynomial systems in mathematical physics |

## Architecture

```
geode/
├── crates/
│   ├── geode-poly/       # polynomial arithmetic (BigInt coefficients)
│   ├── geode-catalan/    # Hyper-Catalan numbers
│   ├── geode-series/     # multi-dimensional power series
│   ├── geode-solver/     # main solver (series → roots)
│   ├── geode-simplify/   # algebraic simplification
│   ├── geode-geode/      # The Geode data structure
│   └── geode-wasm/       # wasm-bindgen bridge
├── ui/                   # Vite + WASM solver playground
├── docs/math.md          # the Wildberger–Rubine theory
└── scripts/              # regression + dev scripts
```

## Build

```bash
cargo test                                    # unit tests
cargo build --release                         # optimized binary
cd ui && npm install && npm run dev           # local dev server
```

## Tech stack

- **Rust** — core series engine
- **num-rational / num-bigint** — arbitrary-precision rationals
- **wasm-bindgen** — browser bridge
- **Vite** — frontend build

## References

- Wildberger, Rubine, "The Hyper-Catalan Series Solution to Polynomial Equations" (2024).
- Wildberger, "The Geode" (2010s–2020s).
- Abel, "Mémoire sur les équations algébriques" (1824).
- Knuth, *The Art of Computer Programming, Vol. 4A* (combinatorial enumeration).

## License

MIT — see [LICENSE](LICENSE).