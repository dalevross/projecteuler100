
// Largest prime factor
   
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

pub fn run(number: i64 ) -> i64 {
    let mut max_prime_factor: i64 = 0;
    let largest_possible_factor = number / 2;

   
    for num in 2..=largest_possible_factor {
        max_prime_factor = if is_prime_factor(number,num) && num > max_prime_factor {num} else {max_prime_factor};
    }
    max_prime_factor
    
}

fn is_prime_factor(number: i64, fac : i64) -> bool
{
    //println!("In is_prime_factor {} {} {}",number,fac,number % fac);

    is_prime(fac) && (number % fac == 0)
    

}

fn is_prime(number: i64) -> bool
{
    for num in 2..number/2
    {
        if number % num == 0
        {
            return false;
        }

    }
    true
}
