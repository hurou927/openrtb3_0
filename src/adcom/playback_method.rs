use crate::rtb_type_strict;

rtb_type_strict! {
PlaybackMethod,
InitiatesOnPageLoadWithSoundOn=1;
InitiatesOnPageLoadWithSoundOffByDefault=2;
InitiatesOnClickWithSoundOn =3;
InitiatesOnMouseOverWithSoundOn = 4;
InitiatesOnEnteringViewportWithSoundOn = 5;
InitiatesOnenteringViewportWithSoundOffByDefault=6
}
