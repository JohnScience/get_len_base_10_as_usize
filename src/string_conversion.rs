/// Trait providing the method-namesake
/// 
/// Requires `std` feature.
#[cfg(any(doc, test, doctest, feature = "std"))]
pub trait GetLenBase10AsUsizeViaStringConversion {
    fn get_len_base_10_as_usize_via_string_conversion(&self) -> usize;
}

#[cfg(any(doc, test, doctest, feature = "std"))]
impl<T> GetLenBase10AsUsizeViaStringConversion for T
where
    T: dd_maths_traits::int::IntSubset
        + dd_maths_traits::num_sys::base_10::ToCanonicalRepresentationBase10AsString,
{
    fn get_len_base_10_as_usize_via_string_conversion(&self) -> usize {
        // Costly syscall for allocation
        let s: String = self.to_canonical_representation_base_10_as_string();
        s.len()
        // Costly syscall for deallocation
    }
}
