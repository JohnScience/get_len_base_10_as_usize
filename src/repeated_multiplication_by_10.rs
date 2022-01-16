/// Trait providing the method-namesake
pub trait GetLenBase10AsUsizeViaRepeatedMultiplicationBy10 {
    fn get_len_base_10_as_usize_via_multiplication_by_10(&self) -> usize;
}

impl<T> GetLenBase10AsUsizeViaRepeatedMultiplicationBy10 for T
where
    T: num_traits::PrimInt + num_traits::FromPrimitive + is_signed_trait::IsSigned,
{
    fn get_len_base_10_as_usize_via_multiplication_by_10(&self) -> usize {
        use num_traits::pow::checked_pow;
        use take_until::TakeUntilExt;

        debug_assert!([['-'].len(), "-".len(),].into_iter().all(|n| n == 1));
        const LEN_OF_MINUS: usize = 1;

        let zero = T::zero();
        let ten: T = T::from_u8(10)
            // unwrap will always succeed because 10 will fit into any primitive integer
            .unwrap();

        let iter_of_opt_powers_of_10 = (1usize..).map(|exponent| checked_pow(ten, exponent));

        if Self::IS_SIGNED && self < &zero {
            // https://en.wikipedia.org/wiki/Sign_function
            let signum: T = T::from_isize(-1)
                // unwrap will always succeed because -1 will fit into any primitive integer that
                // is capable of holding a negative number (which means that T is signed)
                .unwrap();
            let len_of_digits: usize = iter_of_opt_powers_of_10
                .map(|opt_power_of_10: Option<T>| {
                    // If a positive value fits into primitive integer, its inverse fits too
                    opt_power_of_10.map(|power_of_10| signum * power_of_10)
                })
                .take_until(|opt_power_of_10: &Option<T>| {
                    if let Some(power_of_10) = opt_power_of_10 {
                        power_of_10 < self
                    } else {
                        true
                    }
                })
                .count();

            LEN_OF_MINUS + len_of_digits
        } else {
            iter_of_opt_powers_of_10
                .take_until(|opt_power_of_10: &Option<T>| {
                    if let Some(power_of_10) = opt_power_of_10 {
                        self < power_of_10
                    } else {
                        true
                    }
                })
                .count()
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_for_val {
        ($test_name:ident, $val:expr) => {
            #[test]
            fn $test_name() {
                use crate::repeated_multiplication_by_10::GetLenBase10AsUsizeViaRepeatedMultiplicationBy10;
                use crate::string_conversion::GetLenBase10AsUsizeViaStringConversion;

                assert_eq!(
                    $val.get_len_base_10_as_usize_via_string_conversion(),
                    $val.get_len_base_10_as_usize_via_multiplication_by_10()
                );
            }
        };
    }

    test_for_val!(
        get_len_base_10_as_usize_via_multiplication_by_10_works_for_u64_max,
        u64::MAX
    );
    test_for_val!(
        get_len_base_10_as_usize_via_multiplication_by_10_works_for_9u8,
        9u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_multiplication_by_10_works_for_10u8,
        10u8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_multiplication_by_10_works_for_minus_1i8,
        -1i8
    );
    test_for_val!(
        get_len_base_10_as_usize_via_multiplication_by_10_works_for_minus_10i8,
        -10i8
    );
}
