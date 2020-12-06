/// Allows for the conversion of `&[u8]` to `usize`s, with some limitations due to the current state
/// of `const fn`s in Rust. (Since traits cannot be used in `const fn`'s, we cannot use
/// `usize::FromStr` to do this simple task.)
///
/// Currently, you can't index into a container to get a slice (no `&"foobar"[0..4]`). Therefore,
/// instead of taking an `&[u8]`, we take an array of size `N`.
///
/// Say you are reading a sequence of characters from a string like "3-456-", and want extract the
/// usize values `3` and `456`. Knowing that the maximal of the length of either number is 3 digits,
/// we would declare an array `[None; 3]`. Then, iterate over the bytes, adding each to the array,
/// until you meet the `-` or `:` char. Then, call this method to get your number.
///
/// ```rust
/// # #![feature(min_const_generics)]
/// # use aoc2020::const_utils::stou;
/// # const fn is_const_fn() -> (usize, usize) {
/// let str = b"3-456:";
///
/// let mut number3 = 0;
/// let mut number456 = 0;
///
/// // input to `stou`
/// let mut array = [None; 3];
///
/// // maintain state
/// let mut i = 0;
/// let mut i0 = 0;
/// while i < str.len() {
///     let char = str[i];
///     match char {
///         b'-' => {
///             number3 = stou(array);
///             array = [None; 3];
///             i0 = i + 1;
///         }
///         b':' => {
///             number456 = stou(array);
///         }
///         _ => {
///             array[i - i0] = Some(char);
///         }
///     }
///     i += 1;
/// }
/// # (number3, number456)
/// # }
/// # const NUMS: (usize, usize) = is_const_fn();
/// # let number3: usize = NUMS.0;
/// # let number456: usize = NUMS.1;
/// assert_eq!(number3, 3);
/// assert_eq!(number456, 456);
/// ```
pub const fn stou<const N: usize>(str: [Option<u8>; N]) -> usize {
    let mut res = 0;
    let mut i = 0;
    let mut exp = 0;
    while i < N {
        if let Some(char) = str[N - 1 - i] {
            let digit = (char - b'0') as usize;
            res += digit * 10_u32.pow(exp as _) as usize;
            exp += 1;
        }

        i += 1;
    }
    res
}