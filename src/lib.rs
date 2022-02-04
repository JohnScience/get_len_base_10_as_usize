#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
#![cfg_attr(
    any(doc, test, doctest, feature = "const_trait_impl"),
    feature(const_trait_impl)
)]

#![doc = include_str!("../README.md")]

#[cfg(any(doc, test, doctest, feature = "semver_exempt"))]
#[doc(hidden)]
mod fp_logarithm;
#[cfg(any(doc, test, doctest, feature = "semver_exempt"))]
pub use fp_logarithm::TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm;

#[cfg(any(doc, test, doctest, feature = "std"))]
#[doc(hidden)]
mod string_conversion;
#[cfg(any(doc, test, doctest, feature = "std"))]
pub use string_conversion::GetLenBase10AsUsizeViaStringConversion;

#[doc(hidden)]
mod repeated_multiplication_by_10;
pub use repeated_multiplication_by_10::GetLenBase10AsUsizeViaRepeatedMultiplicationBy10;

#[doc(hidden)]
mod divide_and_conquer;
pub use divide_and_conquer::GetLenBase10AsUsizeViaDivideAndConquerApproach;

pub use ::max_len_base_10_as_usize::MaxLenBase10AsUsize;

#[doc(hidden)]
mod dividing_with_pows;
pub use dividing_with_pows::GetLenBase10AsUsizeViaDivigingWithPowsOf2;

pub trait GetLenBase10AsUsize {
    fn get_len_base_10_as_usize(&self) -> usize;
}
