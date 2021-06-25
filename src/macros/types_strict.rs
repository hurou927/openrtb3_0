#[macro_export]
macro_rules! rtb_type_strict {
    ($type_name: ident, $($name: ident = $v:expr);*) => {
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum $type_name {
$(
    $name,
)*
}

impl $type_name {
    pub fn value(&self) -> i32 {
        match self {
        $(
            Self::$name => $v,
        )*
        }
    }

    pub fn from_value_opt(n: i32) -> Option<Self> {
        match n {
        $(
            $v => Some(Self::$name),
        )*
            _n => None
        }
    }
}


impl serde::Serialize for $type_name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = self.value();
        serializer.serialize_i32(v)
    }
}

impl<'de> serde::Deserialize<'de> for $type_name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parsed = i32::deserialize(deserializer)?;
        let var_opt = $type_name::from_value_opt(parsed);

        match var_opt {
            Some(v) => Ok(v),
            None => {
                let s = format!("invalid value: {}. type_name: {}", parsed, stringify!($type_name));
                Err(serde::de::Error::custom(s))
            }
        }
        // Ok(v)
    }
}

    };
}

#[cfg(test)]
mod test {
    #[macro_use]
    use super::*;
    rtb_type_strict! { TypeStrict, One = 1; Two = 2}

    #[test]
    fn base() {
        assert!(TypeStrict::One.value() == 1);
        assert!(TypeStrict::Two.value() == 2);
        assert!(TypeStrict::from_value_opt(1) == Some(TypeStrict::One));
        assert!(TypeStrict::from_value_opt(2) == Some(TypeStrict::Two));
        assert!(TypeStrict::from_value_opt(500) == None);
    }

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<TypeStrict>("3").is_err());
        assert!(serde_json::from_str::<TypeStrict>("499").is_err());
        assert!(serde_json::from_str::<TypeStrict>("500").is_err());
        assert!(serde_json::from_str::<TypeStrict>("1").unwrap() == TypeStrict::One);
        assert!(serde_json::from_str::<TypeStrict>("2").unwrap() == TypeStrict::Two);
        Ok(())
    }
}
