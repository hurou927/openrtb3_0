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
pub enum $type_name {
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

//rtb_type! {Apt, 500, Hoge = 1; Boo = 3}

//rtb_type_loose! {Yum, Hoge = 1; Boo = 3}
// #[derive(Debug, PartialEq, Eq, Clone, Copy)]

#[cfg(test)]
mod test {
    #[macro_use]
    use super::*;
    rtb_type_with_custom! { T, A = 1; B = 2}
    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(T::A.value() == 1);
        assert!(T::B.value() == 2);
        Ok(())
    }
}
