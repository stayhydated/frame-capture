use super::super::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    #[test]
    fn frame_gate_matches_an_independent_latched_sequence_model(
        initial in prop_oneof![Just(0), Just(u32::MAX - 1), Just(u32::MAX), any::<u32>()],
        target in 1..=u32::MAX,
        operations in prop::collection::vec(any::<bool>(), 0..65),
    ) {
        let mut gate = CaptureFrameGate { frame: initial, requested: false };
        let target = CaptureFrame::new(target);
        let mut advances_before_request = 0u64;
        let mut requested = false;
        prop_assert_eq!(gate.frame(), initial);
        prop_assert!(!gate.requested());
        prop_assert_eq!(gate.ready(target), initial >= target.get());
        for request in operations {
            if request {
                gate.mark_requested();
                requested = true;
            } else {
                gate.advance();
                if !requested {
                    advances_before_request += 1;
                }
            }
            let expected_frame = (u64::from(initial) + advances_before_request)
                .min(u64::from(u32::MAX)) as u32;
            prop_assert_eq!(gate.frame(), expected_frame);
            prop_assert_eq!(gate.requested(), requested);
            prop_assert_eq!(gate.ready(target), !requested && expected_frame >= target.get());
        }
    }
}
