use crate::models::openrtb;

pub struct BidRequest {
    pub id: String,
}

pub struct BidResponse {
    pub body: String,
}

pub trait SspApi {
    type HttpReq;
    type HttpRes;
    fn parse(body: Self::HttpReq) -> BidRequest;
    fn format(res: BidResponse) -> Self::HttpRes;
}
