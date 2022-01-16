use is_signed_trait::IsSigned;
use min_max_traits::{Max, Min};

/// Trait providing the namesake associated constant
pub trait MaxLenBase10AsUsize {
    const MAX_LEN_BASE_10_AS_USIZE: usize;
}

impl MaxLenBase10AsUsize for u8 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 3;
}

impl MaxLenBase10AsUsize for u16 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 5;
}

impl MaxLenBase10AsUsize for u32 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 10;
}

impl MaxLenBase10AsUsize for u64 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 20;
}

impl MaxLenBase10AsUsize for u128 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 39;
}

impl MaxLenBase10AsUsize for usize {
    const MAX_LEN_BASE_10_AS_USIZE: usize = match core::mem::size_of::<usize>() {
        1 => u8::MAX_LEN_BASE_10_AS_USIZE,
        2 => u16::MAX_LEN_BASE_10_AS_USIZE,
        4 => u32::MAX_LEN_BASE_10_AS_USIZE,
        8 => u64::MAX_LEN_BASE_10_AS_USIZE,
        16 => u128::MAX_LEN_BASE_10_AS_USIZE,
        _ => unimplemented!(),
    };
}

impl MaxLenBase10AsUsize for i8 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 4;
}

impl MaxLenBase10AsUsize for i16 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 6;
}

impl MaxLenBase10AsUsize for i32 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 11;
}

impl MaxLenBase10AsUsize for i64 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 20;
}

impl MaxLenBase10AsUsize for i128 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 40;
}

impl MaxLenBase10AsUsize for isize {
    const MAX_LEN_BASE_10_AS_USIZE: usize = match core::mem::size_of::<usize>() {
        1 => i8::MAX_LEN_BASE_10_AS_USIZE,
        2 => i16::MAX_LEN_BASE_10_AS_USIZE,
        4 => i32::MAX_LEN_BASE_10_AS_USIZE,
        8 => i64::MAX_LEN_BASE_10_AS_USIZE,
        16 => i128::MAX_LEN_BASE_10_AS_USIZE,
        _ => unimplemented!(),
    };
}

// https://en.wikipedia.org/wiki/Equivalence_class#Definition_and_notation
trait CanonicalRepresentativeWithGreatestLenBase10 {
    const CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10: Self;
}

impl<T> CanonicalRepresentativeWithGreatestLenBase10 for T
where
    T: num_traits::PrimInt + IsSigned + Min + Max,
{
    const CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10: T =
        if T::IS_SIGNED { T::MIN } else { T::MAX };
}

#[cfg(test)]
mod tests {
    use super::CanonicalRepresentativeWithGreatestLenBase10;
    use crate::{GetLenBase10AsUsizeViaRepeatedMultiplicationBy10, MaxLenBase10AsUsize};

    macro_rules! test_for_t {
        ($test_name:ident, $t:ty) => {
            #[test]
            fn $test_name() {
                assert_eq!(
                    <$t>::MAX_LEN_BASE_10_AS_USIZE,
                    <$t>::CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10
                        .get_len_base_10_as_usize_via_multiplication_by_10()
                );
            }
        };
    }

    test_for_t!(max_len_base_10_for_u8_is_correct, u8);
    test_for_t!(max_len_base_10_for_u16_is_correct, u16);
    test_for_t!(max_len_base_10_for_u32_is_correct, u32);
    test_for_t!(max_len_base_10_for_u64_is_correct, u64);
    test_for_t!(max_len_base_10_for_u128_is_correct, u128);
    test_for_t!(max_len_base_10_for_usize_is_correct, usize);
    test_for_t!(max_len_base_10_for_i8_is_correct, i8);
    test_for_t!(max_len_base_10_for_i16_is_correct, i16);
    test_for_t!(max_len_base_10_for_i32_is_correct, i32);
    test_for_t!(max_len_base_10_for_i64_is_correct, i64);
    test_for_t!(max_len_base_10_for_i128_is_correct, i128);
    test_for_t!(max_len_base_10_for_isize_is_correct, isize);
}
