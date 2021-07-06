use crate::rtb_type_strict;

rtb_type_strict! {
VolumeNormalizationMode,
None=0;
AdvolumeAverageNormalizedtoContent=1;
AdVolumePeakNormalizedtoContent=2;
AdLoudnessNormalizedToContent=3;
CustomVolumeNormalization=4
}
