use crate::rtb_type_strict;

rtb_type_strict! {
    Package,
    IndividualWinsAccepted= 0;
    PacakgeWinOrLossOnly = 1
}
impl Default for Package {
    fn default() -> Self {
        Self::IndividualWinsAccepted
    }
}
