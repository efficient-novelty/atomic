use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, PartialEq, Serialize)]
pub struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    pub const fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    pub const fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    pub fn new(num: i64, den: i64) -> Self {
        assert!(den != 0, "rational denominator must be non-zero");

        let sign = if den < 0 { -1 } else { 1 };
        let num = num * sign;
        let den = den * sign;
        let gcd = gcd_i64(num, den);

        Self {
            num: num / gcd,
            den: den / gcd,
        }
    }

    pub const fn from_integer(value: i64) -> Self {
        Self { num: value, den: 1 }
    }

    pub const fn num(self) -> i64 {
        self.num
    }

    pub const fn den(self) -> i64 {
        self.den
    }

    pub const fn is_zero(self) -> bool {
        self.num == 0
    }

    pub const fn is_positive(self) -> bool {
        self.num > 0
    }

    fn from_i128(num: i128, den: i128) -> Self {
        let num = i64::try_from(num).expect("rational numerator exceeded i64 range");
        let den = i64::try_from(den).expect("rational denominator exceeded i64 range");
        Self::new(num, den)
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = i128::from(self.num) * i128::from(other.den);
        let rhs = i128::from(other.num) * i128::from(self.den);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_i128(
            i128::from(self.num) * i128::from(rhs.den) + i128::from(rhs.num) * i128::from(self.den),
            i128::from(self.den) * i128::from(rhs.den),
        )
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_i128(
            i128::from(self.num) * i128::from(rhs.den) - i128::from(rhs.num) * i128::from(self.den),
            i128::from(self.den) * i128::from(rhs.den),
        )
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_i128(
            i128::from(self.num) * i128::from(rhs.num),
            i128::from(self.den) * i128::from(rhs.den),
        )
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.num != 0, "cannot divide by a zero rational");
        Self::from_i128(
            i128::from(self.num) * i128::from(rhs.den),
            i128::from(self.den) * i128::from(rhs.num),
        )
    }
}

const fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    a = if a < 0 { -a } else { a };
    b = if b < 0 { -b } else { b };

    if a == 0 {
        return if b == 0 { 1 } else { b };
    }
    if b == 0 {
        return a;
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::Rational;

    #[test]
    fn rationals_are_normalized() {
        let ratio = Rational::new(6, -8);
        assert_eq!(ratio.num(), -3);
        assert_eq!(ratio.den(), 4);
    }

    #[test]
    fn ordering_uses_exact_cross_multiplication() {
        assert!(Rational::new(17, 4) > Rational::new(4, 1));
        assert!(Rational::new(401, 100) > Rational::new(4, 1));
    }

    #[test]
    fn arithmetic_preserves_exactness() {
        let rho = Rational::new(17, 4);
        let bar = Rational::new(401, 100);

        assert_eq!(rho - bar, Rational::new(24, 100));
        assert_eq!(
            Rational::new(2, 3) + Rational::new(1, 6),
            Rational::new(5, 6)
        );
        assert_eq!(
            Rational::new(3, 5) * Rational::new(10, 9),
            Rational::new(2, 3)
        );
        assert_eq!(
            Rational::new(8, 9) / Rational::new(4, 3),
            Rational::new(2, 3)
        );
    }
}
