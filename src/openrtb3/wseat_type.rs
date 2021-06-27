use crate::rtb_type_strict;

rtb_type_strict! {
    WSeatType,
    BlockList = 0;
    AllowList = 1
}
impl Default for WSeatType {
    fn default() -> Self {
        Self::AllowList
    }
}
