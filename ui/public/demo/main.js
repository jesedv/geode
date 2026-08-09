import init, * as geode from "../pkg/geode_wasm.js";

async function main() {
  await init(new URL("../pkg/geode_wasm_bg.wasm", import.meta.url));
  document.getElementById("meta").textContent =
    `geode · WASM loaded (${geode.hello()})`;
  wireUp();
}

function el(id) { return document.getElementById(id); }

function parseCoeffs(str) {
  return str.split(",").map(s => parseFloat(s.trim())).filter(n => !isNaN(n));
}

function polyEval(coeffs, x) {
  let result = 0;
  for (let i = 0; i < coeffs.length; i++) {
    result = result * x + coeffs[i];
  }
  return result;
}

function solveNewton(coeffs, guess, iters) {
  let x = guess;
  const n = coeffs.length - 1;
  for (let i = 0; i < iters; i++) {
    let f = 0, df = 0;
    for (let j = 0; j < coeffs.length; j++) {
      const power = coeffs.length - 1 - j;
      f += coeffs[j] * Math.pow(x, power);
      if (power > 0) df += coeffs[j] * power * Math.pow(x, power - 1);
    }
    if (Math.abs(df) < 1e-15) break;
    x -= f / df;
  }
  return x;
}

function polyDegree(coeffs) {
  return coeffs.length - 1;
}

function formatPoly(coeffs) {
  const n = polyDegree(coeffs);
  const terms = [];
  for (let i = 0; i < coeffs.length; i++) {
    const c = coeffs[i];
    const p = n - i;
    if (c === 0) continue;
    let term = "";
    if (p === 0) term = `${c}`;
    else if (p === 1) term = c === 1 ? "x" : c === -1 ? "-x" : `${c}x`;
    else term = c === 1 ? `x^${p}` : c === -1 ? `-x^${p}` : `${c}x^${p}`;
    if (terms.length > 0 && c > 0) term = "+" + term;
    terms.push(term);
  }
  return terms.join(" ") || "0";
}

function findRoots(coeffs) {
  const n = polyDegree(coeffs);
  if (n <= 0) return [];
  if (n === 1) return [-coeffs[1] / coeffs[0]];

  const roots = [];
  const guesses = [-5, -2, -1, -0.5, 0, 0.5, 1, 2, 5, 10];
  const seen = new Set();

  for (const g of guesses) {
    const root = solveNewton(coeffs, g, 100);
    const rounded = Math.round(root * 1e8) / 1e8;
    if (!seen.has(rounded) && Math.abs(polyEval(coeffs, root)) < 1e-6) {
      seen.add(rounded);
      roots.push(root);
      if (roots.length >= n) break;
    }
  }
  return roots.sort((a, b) => a - b);
}

function wireUp() {
  el("solve-form").addEventListener("submit", (e) => {
    e.preventDefault();
    const coeffs = parseCoeffs(el("coeffs").value);
    if (coeffs.length < 2) { el("solution").textContent = "Need at least 2 coefficients"; return; }

    const n = polyDegree(coeffs);
    const depth = parseInt(el("depth").value) || 15;
    const roots = findRoots(coeffs);

    let out = `Polynomial: ${formatPoly(coeffs)}\n`;
    out += `Degree: ${n}\n`;
    out += `Solver depth: ${depth}\n`;
    out += `Hyper-Catalan series: (engine stub — showing Newton fallback)\n\n`;
    out += `Roots found: ${roots.length}\n`;
    roots.forEach((r, i) => {
      const residual = Math.abs(polyEval(coeffs, r));
      out += `  x${i+1} = ${r.toFixed(12)}  (residual: ${residual.toExponential(2)})\n`;
    });
    out += `\nNote: The Hyper-Catalan series engine is under development.`;
    out += `\nThis demo uses Newton's method as a temporary fallback.`;
    el("solution").textContent = out;
  });

  document.querySelectorAll(".ex").forEach(btn => {
    btn.addEventListener("click", () => {
      el("coeffs").value = btn.dataset.coeffs;
      el("solve-form").dispatchEvent(new Event("submit"));
    });
  });

  el("eval-form").addEventListener("submit", (e) => {
    e.preventDefault();
    const coeffs = parseCoeffs(el("coeffs").value);
    const x = parseFloat(el("eval-x").value);
    if (coeffs.length < 2 || isNaN(x)) return;
    const result = polyEval(coeffs, x);
    el("eval-out").textContent = `f(${x}) = ${result}\n\nPolynomial: ${formatPoly(coeffs)}`;
  });
}

main();