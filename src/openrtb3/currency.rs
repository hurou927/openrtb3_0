use iso_4217::*;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Currency {
    pub code: CurrencyCode,
}

impl Default for Currency {
    fn default() -> Self {
        Currency {
            code: CurrencyCode::USD,
        }
    }
}

impl Currency {
    pub fn default_currencies() -> Vec<Self> {
        vec![Currency {
            code: CurrencyCode::USD,
        }]
    }
}

impl serde::Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.code.alpha())
    }
}

impl<'de> serde::Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parsed = String::deserialize(deserializer)?;
        let cur_result: Result<CurrencyCode, _> = TryFrom::try_from(parsed.as_ref());
        match cur_result {
            Err(_) => {
                let s = format!("undefined currency: {}", parsed);
                Err(serde::de::Error::custom(s))
            }
            Ok(cur) => Ok(Currency { code: cur }),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json() {
        assert!(
            serde_json::from_str::<Currency>("\"JPY\"")
                .unwrap()
                .code
                .alpha()
                == "JPY"
        );
        assert!(
            serde_json::from_str::<Currency>("\"USD\"")
                .unwrap()
                .code
                .alpha()
                == "USD"
        );
        assert!(serde_json::from_str::<Currency>("\"JPYY\"").is_err());
    }
}
