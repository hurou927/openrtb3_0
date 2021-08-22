use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum DomainSpec {
    Adcom,
}

impl DomainSpec {
    pub fn value(&self) -> &str {
        match self {
            &Self::Adcom => "adcom",
        }
    }
}

#[derive(Debug)]
pub struct ParseError(String);
impl TryFrom<&str> for DomainSpec {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_ref() {
            "adcom" => Ok(DomainSpec::Adcom),
            els => Err(ParseError(els.into())),
        }
    }
}

impl serde::Serialize for DomainSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value().into())
    }
}

impl<'de> serde::Deserialize<'de> for DomainSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parsed = String::deserialize(deserializer)?;
        let result: Result<DomainSpec, _> = TryFrom::try_from(parsed.as_ref());
        match result {
            Err(_) => {
                let s = format!("undefined domainspec: {}", parsed);
                Err(serde::de::Error::custom(s))
            }
            Ok(cur) => Ok(cur),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json() {
        assert!(serde_json::from_str::<DomainSpec>("\"adcom\"").unwrap() == DomainSpec::Adcom);
        assert!(serde_json::from_str::<DomainSpec>("\"JPYY\"").is_err());
    }
}
