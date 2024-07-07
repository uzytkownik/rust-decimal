use crate::Decimal;

use proptest::arbitrary::{Arbitrary, StrategyFor};
use proptest::prelude::*;
use proptest::strategy::Map;
use std::ops::RangeInclusive;

impl Arbitrary for Decimal {
    type Parameters = ();
    type Strategy =
        Map<(StrategyFor<(u32, u32, u32, bool)>, RangeInclusive<u8>), fn(((u32, u32, u32, bool), u8)) -> Self>;

    fn arbitrary_with(_parameters: Self::Parameters) -> Self::Strategy {
        // generate 3 arbitrary u32, a bool and an u32 between 0 to 28
        (any::<(u32, u32, u32, bool)>(), 0..=28)
            .prop_map(|((lo, mid, hi, negative), scale)| Decimal::from_parts(lo, mid, hi, negative, scale as u32))
    }
}
