use std::fmt;

use crate::error::{Error, Result};

/// 1 MINA = 10^9 nanomina.
const NANOMINA_PER_MINA: u64 = 1_000_000_000;

/// Represents a Mina currency amount stored internally as nanomina (atomic units).
///
/// # Examples
/// ```
/// use mina_sdk::Currency;
///
/// let one_mina = Currency::from_mina("1.5").unwrap();
/// assert_eq!(one_mina.nanomina(), 1_500_000_000);
/// assert_eq!(one_mina.mina(), "1.500000000");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Currency(u64);

impl Currency {
    /// Create from a nanomina (atomic unit) value.
    pub fn from_nanomina(nanomina: u64) -> Self {
        Self(nanomina)
    }

    /// Create from a whole MINA decimal string (e.g. "1.5", "100", "0.000000001").
    pub fn from_mina(s: &str) -> Result<Self> {
        parse_decimal(s).map(Self)
    }

    /// Create from a GraphQL response value (nanomina as string).
    pub fn from_graphql(s: &str) -> Result<Self> {
        s.parse::<u64>()
            .map(Self)
            .map_err(|_| Error::InvalidCurrency(s.to_string()))
    }

    /// Get the value in nanomina (atomic units).
    pub fn nanomina(&self) -> u64 {
        self.0
    }

    /// Get the value as a MINA decimal string with 9 decimal places.
    pub fn mina(&self) -> String {
        let whole = self.0 / NANOMINA_PER_MINA;
        let frac = self.0 % NANOMINA_PER_MINA;
        format!("{whole}.{frac:09}")
    }

    /// Convert to nanomina string for GraphQL API submission.
    pub fn to_nanomina_str(&self) -> String {
        self.0.to_string()
    }

    /// Checked addition. Returns `None` on overflow.
    pub fn checked_add(self, rhs: Currency) -> Option<Currency> {
        self.0.checked_add(rhs.0).map(Currency)
    }

    /// Checked subtraction. Returns `Err(CurrencyUnderflow)` if result would be negative.
    pub fn checked_sub(self, rhs: Currency) -> Result<Currency> {
        self.0
            .checked_sub(rhs.0)
            .map(Currency)
            .ok_or(Error::CurrencyUnderflow(self.0, rhs.0))
    }

    /// Multiply by a scalar.
    pub fn checked_mul(self, rhs: u64) -> Option<Currency> {
        self.0.checked_mul(rhs).map(Currency)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mina())
    }
}

impl std::ops::Add for Currency {
    type Output = Currency;
    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Currency {
    type Output = Currency;
    /// Panics on underflow. Use `checked_sub` for fallible version.
    fn sub(self, rhs: Self) -> Self::Output {
        Currency(self.0.checked_sub(rhs.0).expect("currency underflow"))
    }
}

impl std::ops::Mul<u64> for Currency {
    type Output = Currency;
    fn mul(self, rhs: u64) -> Self::Output {
        Currency(self.0 * rhs)
    }
}

impl std::ops::Mul<Currency> for u64 {
    type Output = Currency;
    fn mul(self, rhs: Currency) -> Self::Output {
        Currency(self * rhs.0)
    }
}

/// Parse a decimal string like "1.5" or "100" into nanomina.
fn parse_decimal(s: &str) -> Result<u64> {
    let s = s.trim();
    if s.is_empty() {
        return Err(Error::InvalidCurrency(s.to_string()));
    }

    let (whole_str, frac_str) = match s.split_once('.') {
        Some((w, f)) => (w, f),
        None => (s, ""),
    };

    let whole: u64 = if whole_str.is_empty() {
        0
    } else {
        whole_str
            .parse()
            .map_err(|_| Error::InvalidCurrency(s.to_string()))?
    };

    if frac_str.len() > 9 {
        return Err(Error::InvalidCurrency(format!(
            "too many decimal places (max 9): {s}"
        )));
    }

    let frac: u64 = if frac_str.is_empty() {
        0
    } else {
        let padded = format!("{frac_str:0<9}");
        padded
            .parse()
            .map_err(|_| Error::InvalidCurrency(s.to_string()))?
    };

    whole
        .checked_mul(NANOMINA_PER_MINA)
        .and_then(|w| w.checked_add(frac))
        .ok_or_else(|| Error::InvalidCurrency(format!("overflow: {s}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_mina_integer() {
        let c = Currency::from_mina("5").unwrap();
        assert_eq!(c.nanomina(), 5_000_000_000);
    }

    #[test]
    fn from_mina_decimal() {
        let c = Currency::from_mina("1.5").unwrap();
        assert_eq!(c.nanomina(), 1_500_000_000);
    }

    #[test]
    fn from_mina_small() {
        let c = Currency::from_mina("0.000000001").unwrap();
        assert_eq!(c.nanomina(), 1);
    }

    #[test]
    fn from_mina_no_whole() {
        let c = Currency::from_mina(".5").unwrap();
        assert_eq!(c.nanomina(), 500_000_000);
    }

    #[test]
    fn from_graphql() {
        let c = Currency::from_graphql("1500000000").unwrap();
        assert_eq!(c.nanomina(), 1_500_000_000);
        assert_eq!(c.mina(), "1.500000000");
    }

    #[test]
    fn to_nanomina_str() {
        let c = Currency::from_mina("3").unwrap();
        assert_eq!(c.to_nanomina_str(), "3000000000");
    }

    #[test]
    fn display() {
        let c = Currency::from_nanomina(1);
        assert_eq!(c.to_string(), "0.000000001");
        assert_eq!(format!("{c}"), "0.000000001");
    }

    #[test]
    fn addition() {
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_mina("2").unwrap();
        assert_eq!((a + b).nanomina(), 3_000_000_000);
    }

    #[test]
    fn subtraction() {
        let a = Currency::from_mina("3").unwrap();
        let b = Currency::from_mina("1").unwrap();
        assert_eq!((a - b).nanomina(), 2_000_000_000);
    }

    #[test]
    fn checked_sub_underflow() {
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_mina("2").unwrap();
        assert!(a.checked_sub(b).is_err());
    }

    #[test]
    fn multiplication() {
        let c = Currency::from_mina("2").unwrap();
        assert_eq!((c * 3).nanomina(), 6_000_000_000);
    }

    #[test]
    fn reverse_multiplication() {
        let c = Currency::from_mina("2").unwrap();
        assert_eq!((3_u64 * c).nanomina(), 6_000_000_000);
    }

    #[test]
    fn ordering() {
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_mina("2").unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= a);
        assert!(a >= a);
    }

    #[test]
    fn hash_consistency() {
        use std::collections::HashSet;
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_nanomina(1_000_000_000);
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn from_mina_no_decimal() {
        let c = Currency::from_mina("100").unwrap();
        assert_eq!(c.nanomina(), 100_000_000_000);
    }

    #[test]
    fn from_nanomina_explicit() {
        let c = Currency::from_nanomina(500_000_000);
        assert_eq!(c.mina(), "0.500000000");
        assert_eq!(c.nanomina(), 500_000_000);
    }

    #[test]
    fn small_nanomina_display() {
        let c = Currency::from_nanomina(1);
        assert_eq!(c.mina(), "0.000000001");
    }

    #[test]
    fn zero_currency() {
        let c = Currency::from_nanomina(0);
        assert_eq!(c.mina(), "0.000000000");
        assert_eq!(c.to_nanomina_str(), "0");
    }

    #[test]
    fn checked_add_basic() {
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_mina("2").unwrap();
        assert_eq!(a.checked_add(b).unwrap().nanomina(), 3_000_000_000);
    }

    #[test]
    fn checked_mul_basic() {
        let c = Currency::from_mina("2").unwrap();
        assert_eq!(c.checked_mul(3).unwrap().nanomina(), 6_000_000_000);
    }

    #[test]
    fn equality_across_constructors() {
        let a = Currency::from_mina("1").unwrap();
        let b = Currency::from_nanomina(1_000_000_000);
        let c = Currency::from_graphql("1000000000").unwrap();
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn too_many_decimals() {
        assert!(Currency::from_mina("1.0000000001").is_err());
    }

    #[test]
    fn invalid_format() {
        assert!(Currency::from_mina("abc").is_err());
        assert!(Currency::from_mina("").is_err());
        assert!(Currency::from_graphql("not_a_number").is_err());
    }

    #[test]
    fn negative_input_rejected() {
        assert!(Currency::from_mina("-1").is_err());
        assert!(Currency::from_graphql("-500").is_err());
    }
}
