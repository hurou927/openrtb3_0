use crate::models::api_model::*;

pub struct SspController {}

impl SspController {
    pub fn process(req: BidRequest) -> BidResponse {
        BidResponse { body: req.id }
    }
}
