use get_len_base_10_as_usize_macros::impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2;

pub trait GetLenBase10AsUsizeViaDivigingWithPowsOf2 {
    fn get_len_base_10_as_usize_via_dividing_with_pows_of_2(&self) -> usize;
}

impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(u8);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(u16);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(u32);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(u64);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(u128);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(usize);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(i8);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(i16);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(i32);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(i64);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(i128);
impl_get_len_base_10_as_usize_via_dividing_with_pows_of_2!(isize);

#[cfg(test)]
mod tests {
    macro_rules! test_for_val {
        ($test_name:ident, $val:expr) => {
            #[test]
            fn $test_name() {
                use crate::repeated_multiplication_by_10::GetLenBase10AsUsizeViaRepeatedMultiplicationBy10;
                use crate::dividing_with_pows::GetLenBase10AsUsizeViaDivigingWithPowsOf2;

                assert_eq!(
                    $val.get_len_base_10_as_usize_via_dividing_with_pows_of_2(),
                    $val.get_len_base_10_as_usize_via_multiplication_by_10()
                );
            }
        };
    }

    test_for_val!(
        get_len_base_10_as_usize_via_dividing_with_pows_of_2_works_for_u64_max,
        u64::MAX
    );
    test_for_val!(
        get_len_base_10_as_usize_via_dividing_with_pows_of_2_works_for_9u8,
        9u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_dividing_with_pows_of_2_works_for_10u8,
        10u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_dividing_with_pows_of_2_works_for_minus_1i8,
        -1i8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_dividing_with_pows_of_2_works_for_minus_10i8,
        -10i8
    );
}