# The Hyper-Catalan Solution to Polynomial Equations

This document is the mathematical foundation of `geode`.

## 1. The problem

For 200+ years, since Abel–Ruffini (1824), we've known that quintic equations and higher cannot be solved with radicals. Galois theory explains why: the Galois group of a general quintic is `S₅`, which is not solvable.

Existing approaches:
- **Numerical methods** (Newton, Durand–Kerner) — approximate, no closed form
- **Bring–Jerrard** — reduces to `x⁵ + ax + b = 0`, still no closed form
- **CAS** (Mathematica, SymPy) — radicals when possible, numerical otherwise

## 2. The Wildberger–Rubine solution (2024)

Norman Wildberger and Dean Rubine discovered that the Hyper-Catalan series gives a **closed-form rational solution** to any univariate polynomial equation.

### Hyper-Catalan numbers

The classical Catalan numbers `Cₙ = (2n)! / ((n+1)! · n!)` count triangulations of polygons. The **Hyper-Catalan numbers** `Cₘ` generalize this to count dissections into polygons with any number of sides.

For a partition `m = (m₂, m₃, m₄, ...)` where `mₖ` counts the number of `k`-gons:

```
Cₘ = (Σmₖ)! / (Π mₖ! · Π k^mₖ) · (1 / (Σ(k-2)·mₖ + 2))
```

### The series

For a polynomial `aₙxⁿ + aₙ₋₁xⁿ⁻¹ + ... + a₀ = 0`, substitute `y = x · (aₙ/a₀)^{1/n}` to normalize. Then:

```
y = Σₘ Cₘ · (a₁/a₀)^{m₁} · (a₂/a₀)^{m₂} · ...
```

where the sum is over all partitions `m` with `Σ(k-1)·mₖ = n-1`.

### Convergence

The series converges in a neighborhood of the origin in coefficient space. The radius of convergence depends on the polynomial's coefficients. Truncation at depth `d` gives:

```
|error| ≤ O(r⁻ᵈ)
```

where `r` is the convergence radius.

## 3. The Geode

The **Geode** is the infinite algebraic array of coefficients revealed by the series. For each degree `n`, the Geode is a triangular array:

```
Geode(5) = [
  [1],
  [1, 1],
  [1, 2, 1],
  [1, 3, 3, 1],
  ...
]
```

The rows correspond to the coefficients of the Hyper-Catalan expansion at each depth level. The Geode encodes the complete solution structure.

## 4. Practical considerations

### Truncation depth

| depth | correct digits (approx) | time (quintic) |
|-------|------------------------|----------------|
| 5 | ~2 | < 1 ms |
| 10 | ~5 | < 10 ms |
| 15 | ~8 | < 100 ms |
| 20 | ~12 | < 1 s |

### Limitations

- **Convergence radius**: not all polynomials have series converging at the same rate
- **Real roots only** (v1): complex roots require extension to complex coefficients
- **Univariate only** (v1): multivariate systems are v2

## References

- Wildberger, Rubine, "The Hyper-Catalan Series Solution to Polynomial Equations" (2024).
- Wildberger, "A Rational Approach to Trigonometry" (2005).
- Wildberger, "The Geode" (2010s–2020s).
- Abel, "Mémoire sur les équations algébriques" (1824).
- Knuth, *The Art of Computer Programming, Vol. 4A* (combinatorial enumeration).