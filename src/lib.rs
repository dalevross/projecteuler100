
mod p001;

/// ```
/// let arg = 10;
/// let answer = projecteuler100::test_p001(arg);
///
/// assert_eq!(23, answer);
/// ```
pub fn test_p001(limit: i32 ) -> i32 {
   p001::sum_multiples_of3and5(limit)
}




