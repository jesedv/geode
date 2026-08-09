use geode_poly::Polynomial;
use geode_poly::Rational;

#[test]
fn roundtrip_eval() {
    let p = Polynomial::from_i64_vec(&[3, 0, 1]); // 3 + x^2
    let x = Rational::from_integer(3.into());
    let v = p.eval(&x);
    assert_eq!(v, Rational::from_integer((12 as i64).into()));
}
