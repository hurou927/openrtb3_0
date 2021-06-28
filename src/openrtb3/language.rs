use isolang::Language as IsoLang;

#[derive(Debug, PartialEq)]
pub struct Language {
    pub code: IsoLang,
}

impl serde::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.code.to_639_1() {
            Some(st) => serializer.serialize_str(st),
            None => {
                let s = format!("undefined language: {}", self.code);
                Err(serde::ser::Error::custom(s))
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parsed = String::deserialize(deserializer)?;
        match IsoLang::from_639_1(&parsed) {
            None => {
                let s = format!("undefined language: {}", parsed);
                Err(serde::de::Error::custom(s))
            }
            Some(cur) => Ok(Language { code: cur }),
        }
    }
}
