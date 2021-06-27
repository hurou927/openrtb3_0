use crate::rtb_type_strict;

rtb_type_strict! {
    Bool,
    False = 0;
    True = 1
}

impl Bool {
    pub fn default_false() -> Bool {
        Bool::False
    }
    pub fn default_true() -> Bool {
        Bool::True
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json() {
        assert!(serde_json::from_str::<Bool>("0").unwrap() == Bool::False);
        assert!(serde_json::from_str::<Bool>("1").unwrap() == Bool::True);
        assert!(serde_json::from_str::<Bool>("500").is_err());
    }
}
