use num_traits::{PrimInt, ToPrimitive, Unsigned};

use dd_maths_traits::ranges::ClosedIntvl;

/// Trait providing the method-namesake
/// 
/// Requires `semver_exempt` feature. Incomplete.
pub trait TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm {
    fn try_estimate_len_base_10_as_closed_usize_intvl_via_fp_logarithm(
        &self,
    ) -> Option<ClosedIntvl<usize>>;
}

#[allow(unused)]
impl<T> TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm for T
where
    T: PrimInt + Unsigned + ToPrimitive,
{
    /// f64 can be very imprecise
    /// https://en.wikipedia.org/wiki/Floating-point_arithmetic
    ///
    fn try_estimate_len_base_10_as_closed_usize_intvl_via_fp_logarithm(
        &self,
    ) -> Option<ClosedIntvl<usize>> {
        use zero_based_index::AsZeroBasedIndex;

        // At the time of writing, f64 is the largest available floating point number supported by Rust out of the box
        let floating_point_representation: f64 = self.to_f64()?;
        let logarithm: f64 = floating_point_representation.log10();
        // In order to implement it properly, one should first create a crate for dealing with
        // IEEE 754 floating point numbers:
        //
        // https://en.wikipedia.org/wiki/IEEE_754
        unimplemented!()
    }
}
