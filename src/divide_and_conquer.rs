use get_len_base_10_as_usize_macros::impl_get_len_base_10_as_usize_via_divide_and_conquer_approach;

/// Trait providing the method-namesake
pub trait GetLenBase10AsUsizeViaDivideAndConquerApproach {
    fn get_len_base_10_as_usize_via_divide_and_conquer_approach(&self) -> usize;
}

macro_rules! impl_trait_for {
    (@PRIM_INTS) => {
        impl_trait_for!(u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize);
    };
    ($($t:ty),+) => {
        $(
            impl_get_len_base_10_as_usize_via_divide_and_conquer_approach!($t);
        )+
    }
}

impl_trait_for!(@PRIM_INTS);

#[cfg(test)]
mod tests {
    macro_rules! test_for_val {
        ($test_name:ident, $val:expr) => {
            #[test]
            fn $test_name() {
                use crate::GetLenBase10AsUsizeViaDivideAndConquerApproach;
                use crate::GetLenBase10AsUsizeViaDivigingWithPowsOf2;

                assert_eq!(
                    $val.get_len_base_10_as_usize_via_dividing_with_pows_of_2(),
                    $val.get_len_base_10_as_usize_via_divide_and_conquer_approach()
                );
            }
        };
    }

    test_for_val!(
        get_len_base_10_as_usize_via_divide_and_conquer_approach_works_for_u64_max,
        u64::MAX
    );
    test_for_val!(
        get_len_base_10_as_usize_via_divide_and_conquer_approach_works_for_9u8,
        9u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_divide_and_conquer_approach_works_for_10u8,
        10u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_divide_and_conquer_approach_works_for_minus_1i8,
        -1i8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_divide_and_conquer_approach_works_for_minus_10i8,
        -10i8
    );
}