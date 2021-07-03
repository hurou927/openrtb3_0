use crate::rtb_type;

rtb_type! {
ClickType,
500,
NonClickable=0;
DetailsUnknown = 1;
EmbeddedBrowser = 2;
NativeBrowser = 3
}

impl Default for ClickType {
    fn default() -> Self {
        Self::DetailsUnknown
    }
}
