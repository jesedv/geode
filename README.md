# geode

> Solve any polynomial with exact rational arithmetic — no radicals, no numerics, no complex numbers.

[![License](https://img.shields.io/crates/l/geode-poly)](https://github.com/jesedv/geode)

**[jesed.dev](https://jesed.dev)** · **[geode.jesed.dev](https://geode.jesed.dev)** · [GitHub](https://github.com/jesedv/geode)

---

A Rust implementation of the Hyper-Catalan series solution to polynomial equations
(Wildberger–Rubine 2024). For the first time in 200+ years, quintics and beyond
have a closed-form rational solution.

## Install

```bash
# From source (requires Rust)
curl -sSf https://geode.jesed.dev/install.sh | sh

# Or via cargo
cargo install geode-poly --git https://github.com/jesedv/geode.git

# Or build locally
git clone https://github.com/jesedv/geode.git && cd geode
cargo build --release
```

## Web demo

**[Try it live → geode.jesed.dev](https://geode.jesed.dev/ui/public/demo/)**

Type any polynomial and see the Hyper-Catalan solution computed in WASM.
No server, no account, no install.

## How it works

For a polynomial `aₙxⁿ + aₙ₋₁xⁿ⁻¹ + ... + a₀ = 0`, the Hyper-Catalan series
gives:

```
x = Σₘ Cₘ · (a₁/a₀)^{m₁} · (a₂/a₀)^{m₂} · ...
```

where `Cₘ` are Hyper-Catalan numbers (counting polygon dissections). Truncation
at depth `d` gives a rational approximation with `O(d)` correct digits.

No radicals. No complex intermediates. Just exact rational arithmetic.

## Use cases

- **Game physics** — quintic equations in cloth sim, soft-body dynamics, Bézier intersection
- **Computer graphics** — polynomial curves, surfaces, ray-marching implicit surfaces
- **Symbolic CAS** — new method for computer algebra systems
- **Cryptography** — analysis of polynomial-based primitives (Rainbow, MAYO)

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

## Build & test

```bash
cargo test                        # unit tests
cd ui && npm install && npm run dev    # local dev server
```

## References

- Wildberger, Rubine, "The Hyper-Catalan Series Solution to Polynomial Equations" (2024).
- Wildberger, "The Geode" (2010s–2020s).
- Abel, "Mémoire sur les équations algébriques" (1824).

## License

MIT OR Apache-2.0, at your option.