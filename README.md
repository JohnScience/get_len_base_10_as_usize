# Traits for getting length base 10 as usize

This crate offers such traits as [`MaxLenBase10AsUsize`], [`GetLenBase10AsUsizeViaRepeatedMultiplicationBy10`], [`GetLenBase10AsUsize`], [`GetLenBase10AsUsizeViaDivigingWithPowsOf2`]
[`GetLenBase10AsUsizeViaStringConversion`], and [`TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm`].

# Example

```rust
use get_len_base_10_as_usize::MaxLenBase10AsUsize;

assert_eq!(u64::MAX_LEN_BASE_10_AS_USIZE, 20);
```

# Sources of insipration

* <https://www.baeldung.com/java-number-of-digits-in-int>

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[`GetLenBase10AsUsize`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.GetLenBase10AsUsize.html
[`GetLenBase10AsUsizeViaRepeatedMultiplicationBy10`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.GetLenBase10AsUsizeViaRepeatedMultiplicationBy10.html
[`GetLenBase10AsUsizeViaStringConversion`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.GetLenBase10AsUsizeViaStringConversion.html
[`MaxLenBase10AsUsize`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.MaxLenBase10AsUsize.html
[`TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.TryEstimateLenBase10AsClosedUsizeIntvlViaFPLogarithm.html
[`GetLenBase10AsUsizeViaDivigingWithPowsOf2`]: https://docs.rs/get_len_base_10_as_usize/latest/get_len_base_10_as_usize/trait.GetLenBase10AsUsizeViaDivigingWithPowsOf2.html