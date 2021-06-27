use crate::rtb_type;

rtb_type! {
    AuctionType,
    500,
    FirstPrice = 1;
    SecondPricePlus = 2
}

impl Default for AuctionType {
    fn default() -> Self {
        Self::SecondPricePlus
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn json() {
        assert!(serde_json::from_str::<AuctionType>("1").unwrap() == AuctionType::FirstPrice);
        assert!(
            serde_json::from_str::<AuctionType>("500").unwrap()
                == AuctionType::VendorSpecificCode(500)
        );
    }
}
