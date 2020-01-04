
mod p001;
mod p002;

/// ```
/// let arg = 10;
/// let answer = projecteuler100::test_p001(arg);
///
/// assert_eq!(23, answer);
/// ```
pub fn test_p001(limit: i32 ) -> i32 {
   p001::sum_multiples_of3and5(limit)
}


/// ```
/// let arg = 90;
/// let answer = projecteuler100::test_p002(arg);
///
/// assert_eq!(44, answer);
/// ```
pub fn test_p002(limit: i64 ) -> i64 {
   p002::run(limit)
}





