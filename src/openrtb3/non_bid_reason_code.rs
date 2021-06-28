
use crate::rtb_type;

rtb_type! {
NonBidReasonCode,
500,
UnknownError = 0;
TechnicalError = 1;
InvalidRequest = 2;
KnownWebCrawler = 3;
SuspectedNonHumanTraffic = 4;
CloudDataCenterOrProxyIP = 5;
UnsupportedDevice = 6;
BlockedPublisherOrSite = 7;
UnmatchedUser = 8;
DailyUserCapMet = 9;
DailyDomainCapMet = 10;
AdsTxtAuthorizationUnavailable = 11;
AdsTxtAuthorizationViolation = 12;
AdsCertAuthenticationUnavailable = 13;
AdsCertAuthenticationViolation = 14;
InsufficientAuctionTime = 15;
IncompleteSupplyChain = 16;
BlockedSupplyChainNode = 17
}
