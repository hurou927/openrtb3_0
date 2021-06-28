use crate::rtb_type;

rtb_type! {
LossReasonCode,
500,
BidWon=0;
InternalError=1;
ImpressionOpportunityExpired=2;
InvalidBidResponse=3;
InvalidDealID=4;
InvalidAuctionID=5;
InvalidAdvertiserDomain=6;
MissingMarkup=7;
MissingCreativeID=8;
MissingBidPrice=9;
MissingMinimumCreativeApprovalData=10;
BidwasBelowAuctionFloor=100;
BidwasBelowDealFloor=101;
LosttoHigherBid=102;
LosttoaBidforaDeal=103;
BuyerSeatBlocked=104;
CreativeFilteredGeneralReasonUnknown=200;
CreativeFilteredPendingProcessingbyExchange=201;
CreativeFilteredDisapprovedbyExchange=202;
CreativeFilteredSizeNotAllowed=203;
CreativeFilteredIncorrectCreativeFormat=204;
CreativeFilteredAdvertiserExclusions=205;
CreativeFilteredAppBundleExclusions=206;
CreativeFilteredNotSecure=207;
CreativeFilteredLanguageExclusions=208;
CreativeFilteredCategoryExclusions=209;
CreativeFilteredCreativeAttributeExclusions=210;
CreativeFilteredAdTypeExclusions=211;
CreativeFilteredAnimationTooLong=212;
CreativeFilteredNotAllowedinDeal=213
}
