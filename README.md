# geode

> Solve any polynomial with exact rational arithmetic — no radicals, no numerics, no complex numbers.

[![Crates.io](https://img.shields.io/crates/v/geode-poly?label=geode-poly)](https://crates.io/crates/geode-poly)
[![License](https://img.shields.io/crates/l/geode-poly)](https://github.com/jesedv/geode)

**[jesed.dev](https://jesed.dev)** · **[geode.jesed.dev](https://geode.jesed.dev)** · [GitHub](https://github.com/jesedv/geode)

---

A Rust implementation of the Hyper-Catalan series solution to polynomial equations (Wildberger–Rubine 2024). For the first time in 200+ years, quintics and beyond have a closed-form rational solution.

## Install

```bash
curl -sSf https://geode.jesed.dev/install.sh | sh
cargo install geode-cli --git https://github.com/jesedv/geode.git
```

## Usage

```bash
# Solve a quintic: x^5 - x + 1 = 0
geode solve "1*x^5 + 0*x^4 + 0*x^3 + 0*x^2 - 1*x + 1"

# Solve with higher precision
geode solve "x^3 - 2" --depth 20

# Export to LaTeX
geode solve "x^5 - x + 1" --format latex
```

```rust
use geode_poly::Polynomial;
use geode_solver::solve;

// x^5 - x + 1 = 0
let p = Polynomial::from_coefficients(vec![1, 0, 0, 0, -1, 1]);
let solution = solve(&p, 15); // depth-15 truncation
println!("{solution}");       // exact rational approximation
```

## Web demo

```bash
cd ui && npm install && npm run dev    # → http://localhost:5173
```

The demo lets you type any polynomial and see the Hyper-Catalan solution computed live in WASM.

## How it works

For a polynomial `aₙxⁿ + aₙ₋₁xⁿ⁻¹ + ... + a₀ = 0`, the Hyper-Catalan series gives:

```
x = Σ Cₘ · (monomial in aᵢ/aₙ)     for m = 0, 1, 2, ...
```

where `Cₘ` are Hyper-Catalan numbers (counting polygon dissections). The series converges in a neighborhood of the origin; truncation at depth `d` gives a rational approximation with `O(d)` correct digits.

No radicals. No complex intermediates. Just exact rational arithmetic.

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
├── examples/             # canonical quintics, sextics
└── scripts/              # regression + dev scripts
```

## References

- Wildberger, Rubine, "The Hyper-Catalan Series Solution to Polynomial Equations" (2024).
- Wildberger, "The Geode" (2010s–2020s).
- Abel, "Mémoire sur les équations algébriques" (1824).

## License

MIT OR Apache-2.0, at your option.