use super::*;
use proptest::prelude::*;

fn positive_dimension() -> impl Strategy<Value = u32> {
    prop_oneof![Just(1), Just(u32::MAX), 1..=u32::MAX]
}

fn invalid_literal() -> impl Strategy<Value = (String, ParsePixelSizeError)> {
    (positive_dimension(), 0u8..7).prop_map(|(dimension, kind)| {
        let value = match kind {
            0 => format!("0x{dimension}"),
            1 => format!("{dimension}x0"),
            2 => dimension.to_string(),
            3 => format!("wideX{dimension}"),
            4 => format!("{dimension}xhigh"),
            5 => format!("{}x{dimension}", u64::from(u32::MAX) + u64::from(dimension)),
            _ => format!("{dimension}x{dimension}x1"),
        };
        let error = match kind {
            0 => ParsePixelSizeError::ZeroWidth {
                value: value.clone(),
            },
            1 => ParsePixelSizeError::ZeroHeight {
                value: value.clone(),
            },
            2 => ParsePixelSizeError::MissingSeparator {
                value: value.clone(),
            },
            3 | 5 => ParsePixelSizeError::InvalidWidth {
                value: value.clone(),
            },
            _ => ParsePixelSizeError::InvalidHeight {
                value: value.clone(),
            },
        };
        (value, error)
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    #[test]
    fn scaling_preserves_long_edge_and_bounds_rounding_error(
        width in positive_dimension(),
        height in positive_dimension(),
        long_edge in positive_dimension(),
    ) {
        let original = PixelSize::new(width, height);
        let edge = NonZeroU32::new(long_edge).unwrap();
        let scaled = PixelSize::from_long_edge(original, edge);
        prop_assert_eq!(scaled.width().max(scaled.height()), long_edge);
        prop_assert!(scaled.width() > 0 && scaled.height() > 0);
        let transposed = PixelSize::from_long_edge(PixelSize::new(height, width), edge);
        prop_assert_eq!(transposed.dimensions(), (scaled.height(), scaled.width()));
        prop_assert_eq!(
            PixelSize::from_long_edge(original, NonZeroU32::new(width.max(height)).unwrap()),
            original,
        );

        // Bound the ideal rational size independently of the integer rounding
        // implementation. u128 keeps the oracle safe at every u32 boundary.
        let denominator = u128::from(width.max(height));
        let numerator = u128::from(long_edge) * u128::from(width.min(height));
        let shorter = u128::from(scaled.width().min(scaled.height()));
        if numerator < denominator {
            prop_assert_eq!(shorter, 1);
        } else {
            prop_assert!(2 * (shorter * denominator).abs_diff(numerator) <= denominator);
        }
    }

    #[test]
    fn scaling_is_monotone_in_requested_edge(
        width in positive_dimension(),
        height in positive_dimension(),
        first in positive_dimension(),
        second in positive_dimension(),
    ) {
        let original = PixelSize::new(width, height);
        let smaller = PixelSize::from_long_edge(original, NonZeroU32::new(first.min(second)).unwrap());
        let larger = PixelSize::from_long_edge(original, NonZeroU32::new(first.max(second)).unwrap());
        prop_assert!(smaller.width() <= larger.width());
        prop_assert!(smaller.height() <= larger.height());
    }

    #[test]
    fn positive_dimensions_parse_and_serialize_without_changing_values(
        width in positive_dimension(),
        height in positive_dimension(),
        uppercase in any::<bool>(),
    ) {
        let separator = if uppercase { 'X' } else { 'x' };
        let size: PixelSize = format!("{width}{separator}{height}").parse().unwrap();
        prop_assert_eq!(size.dimensions(), (width, height));
        prop_assert_eq!(size.to_string(), format!("{width}x{height}"));
        let json = serde_json::to_string(&size).unwrap();
        prop_assert_eq!(serde_json::from_str::<PixelSize>(&json).unwrap().dimensions(), (width, height));
    }

    #[test]
    fn malformed_size_literals_keep_their_error_category((value, error) in invalid_literal()) {
        prop_assert_eq!(value.parse::<PixelSize>(), Err(error));
    }
}
