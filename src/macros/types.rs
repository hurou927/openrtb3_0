#[macro_export]
macro_rules! rtb_type_with_custom {
    ($type_name: ident, $($name: ident = $v: expr);*) => {
        rtb_type! {
            $type_name, 500, $($name = $v);*
        }
    };
}

#[macro_export]
macro_rules! rtb_type_loose {
    ($type_name: ident, $($name: ident = $v: expr);*) => {
        rtb_type! {
            $type_name, 0, $($name = $v);*
        }
    };
}

#[macro_export]
macro_rules! rtb_type {
    ($type_name: ident, $other_th_value:expr, $($name: ident = $v:expr);*) => {
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum $type_name {
$(
    $name,
)*
    VendorSpecificCode(i32),
}

impl $type_name {
    pub fn value(&self) -> i32 {
        match self {
        $(
            Self::$name => $v,
        )*
            Self::VendorSpecificCode(n) => *n,
        }
    }

    pub fn from_value_opt(n: i32) -> Option<Self> {
        match n {
        $(
            $v => Some(Self::$name),
        )*
            n if n >= ($other_th_value) => Some(Self::VendorSpecificCode(n)),
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
                let s = format!("invalid value: {}. type_name: {}, buyer_specific_value: {}+", parsed, stringify!($type_name), $other_th_value);
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
    rtb_type_with_custom! { TypeWithCustom, One = 1; Eight = 8}

    #[test]
    fn base() {
        assert!(TypeWithCustom::One.value() == 1);
        assert!(TypeWithCustom::Eight.value() == 8);
        assert!(TypeWithCustom::VendorSpecificCode(560).value() == 560);

        assert!(TypeWithCustom::from_value_opt(1) == Some(TypeWithCustom::One));
        assert!(TypeWithCustom::from_value_opt(8) == Some(TypeWithCustom::Eight));
        assert!(TypeWithCustom::from_value_opt(499) == None);
        assert!(
            TypeWithCustom::from_value_opt(500) == Some(TypeWithCustom::VendorSpecificCode(500))
        );
    }

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<TypeWithCustom>("3").is_err());
        assert!(serde_json::from_str::<TypeWithCustom>("499").is_err());
        assert!(serde_json::from_str::<TypeWithCustom>("1").unwrap() == TypeWithCustom::One);
        assert!(serde_json::from_str::<TypeWithCustom>("8").unwrap() == TypeWithCustom::Eight);
        assert!(
            serde_json::from_str::<TypeWithCustom>("500").unwrap()
                == TypeWithCustom::VendorSpecificCode(500)
        );
        Ok(())
    }
}
