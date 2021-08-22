use openrtb3::openrtb::OpenRtb3_0;

mod adcom;
mod openrtb3;

mod rtb_type_macros;
mod rtb_type_strict_macros;

use serde_json;

fn main() {
    let v: OpenRtb3_0 = serde_json::from_str(
        r#"{"openrtb":{
            "ver":"1.0",
            "domainspec": "adcom",
            "domainver": "1.0"
        }}"#,
    )
    .unwrap();
    println!("{:?}", v);
}
