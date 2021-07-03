use crate::rtb_type_strict;

rtb_type_strict! {
    ServerSideAdInsertionType,
    Unknown = 0;
    AllClientSide = 1;
    TrackingPixeldFiredClientSide = 2;
    AllServerSide = 3
}

impl Default for ServerSideAdInsertionType {
    fn default() -> Self {
        Self::Unknown
    }
}
