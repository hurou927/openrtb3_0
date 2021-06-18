use crate::models::api_model::*;

pub struct SspZebra {}

impl SspApi for SspZebra {
    type HttpReq = String;
    type HttpRes = String;

    fn parse(body: String) -> BidRequest {
        BidRequest { body }
    }
    fn format(res: BidResponse) -> String {
        res.body
    }
}
