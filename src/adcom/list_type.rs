use crate::rtb_type_strict;

rtb_type_strict! {
ListType,
Block=1;
Allowed=2
}

impl ListType {
    pub fn default_allowed() -> Self {
        Self::Allowed
    }
}
