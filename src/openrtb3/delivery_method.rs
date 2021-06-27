
use crate::rtb_type_strict;

rtb_type_strict! {
    DeliveryMethod,
    Either = 0;
    PartOfTransaction = 1;
    PreviouslyUploading = 2
}
impl Default for DeliveryMethod {
    fn default() -> Self {
        Self::Either
    }
}
