//! geode-poly: polynomial with exact rational coefficients

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One};

pub type Rational = BigRational;

#[derive(Clone, Debug)]
pub struct Polynomial {
    // coeffs[i] is coefficient for x^i
    coeffs: Vec<Rational>,
}

fn bigint(n: i64) -> BigInt { BigInt::from(n) }
fn rat_from_i64(n: i64) -> Rational { Rational::from_integer(bigint(n)) }

impl Polynomial {
    pub fn new(mut coeffs: Vec<Rational>) -> Self {
        // trim trailing zeros
        while coeffs.last().map_or(false, |c| c.is_zero()) && coeffs.len() > 1 {
            coeffs.pop();
        }
        if coeffs.is_empty() { coeffs.push(Rational::zero()) }
        Polynomial { coeffs }
    }

    pub fn from_i64_vec(v: &[i64]) -> Self {
        let coeffs = v.iter().map(|&n| rat_from_i64(n)).collect();
        Polynomial::new(coeffs)
    }

    pub fn degree(&self) -> usize {
        if self.coeffs.len() == 1 && self.coeffs[0].is_zero() { 0 } else { self.coeffs.len() - 1 }
    }

    pub fn eval(&self, x: &Rational) -> Rational {
        // Horner's method
        let mut acc = Rational::zero();
        for c in self.coeffs.iter().rev() {
            acc = acc * x + c;
        }
        acc
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let n = self.coeffs.len().max(other.coeffs.len());
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(Rational::zero);
            let b = other.coeffs.get(i).cloned().unwrap_or_else(Rational::zero);
            out.push(a + b);
        }
        Polynomial::new(out)
    }

    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        let n = self.coeffs.len();
        let m = other.coeffs.len();
        let mut out = vec![Rational::zero(); n + m - 1];
        for i in 0..n {
            for j in 0..m {
                out[i + j] = out[i + j].clone() + (&self.coeffs[i] * &other.coeffs[j]);
            }
        }
        Polynomial::new(out)
    }

    pub fn derivative(&self) -> Polynomial {
        if self.coeffs.len() <= 1 { return Polynomial::from_i64_vec(&[0]); }
        let mut out = Vec::with_capacity(self.coeffs.len() - 1);
        for (i, c) in self.coeffs.iter().enumerate().skip(1) {
            let mul = BigInt::from(i as i64);
            out.push(c * Rational::from_integer(mul));
        }
        Polynomial::new(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_and_degree() {
        let p = Polynomial::from_i64_vec(&[1, 2, 3, 0, 0]);
        assert_eq!(p.degree(), 2);
        let zero = Polynomial::from_i64_vec(&[0]);
        assert_eq!(zero.degree(), 0);
    }

    #[test]
    fn add_mul_eval() {
        // p = 1 + 2x + 3x^2
        let p = Polynomial::from_i64_vec(&[1,2,3]);
        // q = x
        let q = Polynomial::from_i64_vec(&[0,1]);
        // p+q = 1 + 3x + 3x^2
        let s = p.add(&q);
        let x = rat_from_i64(2);
        // evaluate manually: p(2)=1+4+12=17, q(2)=2, s(2)=19
        assert_eq!(p.eval(&x), rat_from_i64(17));
        assert_eq!(q.eval(&x), rat_from_i64(2));
        assert_eq!(s.eval(&x), rat_from_i64(19));

        // multiplication p * q = x + 2x^2 + 3x^3
        let m = p.mul(&q);
        // evaluate m(2) = 2 + 8 + 24 =34
        assert_eq!(m.eval(&x), rat_from_i64(34));
    }

    #[test]
    fn derivative_test() {
        let p = Polynomial::from_i64_vec(&[1,2,3]); // 1 +2x +3x^2
        let d = p.derivative(); // 2 +6x
        let x = rat_from_i64(3);
        // d(3) = 2 + 18 = 20
        assert_eq!(d.eval(&x), rat_from_i64(20));
    }
}
