#![cfg_attr(not(any(doc, test, doctest, feature = "std")), no_std)]
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
mod max_len_base_10_as_usize;
pub use max_len_base_10_as_usize::MaxLenBase10AsUsize;

pub trait GetLenBase10AsUsize {
    fn get_len_base_10_as_usize(&self) -> usize;
}
