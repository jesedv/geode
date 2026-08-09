use clap::Parser;

#[derive(Parser)]
#[command(name = "geode", version, about = "Solve polynomials with exact rational arithmetic")]
struct Cli {
    /// Polynomial coefficients (highest degree first), comma-separated
    #[arg(short, long)]
    coeffs: String,

    /// Truncation depth for the Hyper-Catalan series
    #[arg(short, long, default_value_t = 15)]
    depth: usize,

    /// Evaluate polynomial at this point
    #[arg(short, long)]
    eval: Option<f64>,
}

fn main() {
    let cli = Cli::parse();

    let coeffs: Vec<f64> = cli
        .coeffs
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if coeffs.is_empty() {
        eprintln!("Error: no valid coefficients provided");
        std::process::exit(1);
    }

    let degree = coeffs.len() - 1;
    println!("Polynomial degree: {degree}");
    println!("Truncation depth: {}", cli.depth);
    println!();

    // Display polynomial
    print!("f(x) = ");
    for (i, c) in coeffs.iter().enumerate() {
        let p = degree - i;
        if *c == 0.0 {
            continue;
        }
        if i > 0 && *c > 0.0 {
            print!(" + ");
        } else if i > 0 && *c < 0.0 {
            print!(" - ");
        } else if *c < 0.0 {
            print!("-");
        }
        let ac = c.abs();
        if p == 0 {
            print!("{ac}");
        } else if p == 1 {
            if ac == 1.0 {
                print!("x");
            } else {
                print!("{ac}x");
            }
        } else {
            if ac == 1.0 {
                print!("x^{p}");
            } else {
                print!("{ac}x^{p}");
            }
        }
    }
    println!();
    println!();

    // Evaluate at a point if requested
    if let Some(x) = cli.eval {
        let mut result = 0.0;
        for (i, c) in coeffs.iter().enumerate() {
            let p = degree - i;
            result += c * x.powi(p as i32);
        }
        println!("f({x}) = {result}");
    }

    // Newton's method to find one real root
    if degree >= 1 {
        let guesses = [-5.0, -2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0, 5.0];
        let mut found = Vec::new();

        for &g in &guesses {
            let mut x: f64 = g;
            for _ in 0..200 {
                let mut f = 0.0;
                let mut df = 0.0;
                for (i, c) in coeffs.iter().enumerate() {
                    let p = degree - i;
                    f += c * x.powi(p as i32);
                    if p > 0 {
                        df += c * p as f64 * x.powi((p - 1) as i32);
                    }
                }
                if df.abs() < 1e-15 {
                    break;
                }
                x -= f / df;
            }
            let residual = eval_poly(&coeffs, x);
            if residual.abs() < 1e-6 && !found.iter().any(|r: &f64| (*r - x).abs() < 1e-4) {
                found.push(x);
            }
        }

        found.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if !found.is_empty() {
            println!("Roots found: {}", found.len());
            for (i, r) in found.iter().enumerate() {
                let residual = eval_poly(&coeffs, *r);
                println!("  x{} = {:.12}  (residual: {:.2e})", i + 1, r, residual);
            }
        } else {
            println!("No real roots found in search range");
        }
    }
}

fn eval_poly(coeffs: &[f64], x: f64) -> f64 {
    let degree = coeffs.len() - 1;
    let mut result = 0.0;
    for (i, c) in coeffs.iter().enumerate() {
        let p = degree - i;
        result += c * x.powi(p as i32);
    }
    result
}