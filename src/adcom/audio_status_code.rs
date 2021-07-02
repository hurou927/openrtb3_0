use crate::rtb_type;

rtb_type! {
AudioStatusCode,
500,
PendingAudio=1;
PreApproved = 2;
Approved = 3;
Denied = 4;
Changed = 5;
Expired = 6
}
