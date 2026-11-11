use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Decimal {
    units: i128,
    scale: u32,
}

impl Decimal {
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let scale = self.scale.max(other.scale);
        let lift = |value: Self| {
            let factor = 10_i128.checked_pow(scale - value.scale)?;
            value.units.checked_mul(factor)
        };
        let units = lift(self)?.checked_sub(lift(other)?)?;
        Some(Self { units, scale })
    }

    pub fn is_zero(self) -> bool {
        self.units == 0
    }

    pub fn is_negative(self) -> bool {
        self.units < 0
    }
}

impl TryFrom<f64> for Decimal {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let text = value.to_string();
        let scale = text
            .split_once('.')
            .map_or(0, |(_, fraction)| fraction.len());
        text.replace('.', "")
            .parse::<i128>()
            .ok()
            .zip(u32::try_from(scale).ok())
            .map(|(units, scale)| Self { units, scale })
            .ok_or_else(|| format!("number {value} is too long for exact arithmetic"))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = self.scale as usize;
        let digits = format!("{:0>width$}", self.units.unsigned_abs(), width = scale + 1);
        let (whole, fraction) = digits.split_at(digits.len() - scale);
        let fraction = fraction.trim_end_matches('0');
        let sign = if self.units < 0 { "-" } else { "" };
        if fraction.is_empty() {
            write!(formatter, "{sign}{whole}")
        } else {
            write!(formatter, "{sign}{whole}.{fraction}")
        }
    }
}

impl Serialize for Decimal {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match i64::try_from(self.units) {
            Ok(units) if self.scale == 0 => serializer.serialize_i64(units),
            _ => {
                let value = self.to_string().parse::<f64>();
                serializer.serialize_f64(value.map_err(ser::Error::custom)?)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_from(f64::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtraction_is_exact_in_decimal() {
        let difference = |left: f64, right: f64| {
            let (left, right) = (Decimal::try_from(left), Decimal::try_from(right));
            left.unwrap()
                .checked_sub(right.unwrap())
                .map(|gap| gap.to_string())
        };
        assert_eq!(difference(0.3, 0.1), Some("0.2".to_owned()));
        assert_eq!(difference(-2.0, -2.5), Some("0.5".to_owned()));
        assert_eq!(difference(2.5, 0.5), Some("2".to_owned()));
        assert_eq!(difference(-0.0, 0.0), Some("0".to_owned()));
        assert_eq!(
            difference(1e15, 0.001),
            Some("999999999999999.999".to_owned())
        );
        assert_eq!(difference(1e-7, 1e-7), Some("0".to_owned()));
        assert_eq!(difference(1e30, 1e-30), None);
        assert_eq!(Decimal::try_from(1e39).ok(), None);
    }
}
