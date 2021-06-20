use crate::errors::error::*;
use crate::models::api_model::*;
use crate::models::openrtb::*;

pub struct SspZebra {}

fn parse(raw: &str) -> Result<OpenRtbRequest, ServiceError> {
    match serde_json::from_str(raw) {
        Ok(s) => Ok(s),
        Err(e) => Err(ServiceError {
            code: ErrorCode::RequestError,
            detail: "hoge".into(),
        }),
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
