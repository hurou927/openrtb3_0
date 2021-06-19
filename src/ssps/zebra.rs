use crate::models::api_model::*;
use crate::models::openrtb::*;
use serde_json;

pub struct SspZebra {}

impl SspApi for SspZebra {
    type HttpReq = String;
    type HttpRes = String;

    fn parse(body: String) -> BidRequest {
        let rtb_req: OpenRtbRequest = serde_json::from_str(&body).unwrap();
        BidRequest { id: rtb_req.id }
    }
    fn format(res: BidResponse) -> String {
        res.body
    }
}
