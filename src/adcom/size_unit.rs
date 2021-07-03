use crate::rtb_type_strict;

rtb_type_strict! {
SizeUnit,
Dips=1;
Inches = 2;
Centimeters = 3
}

impl Default for SizeUnit {
    fn default() -> Self {
        Self::Dips
    }
}
