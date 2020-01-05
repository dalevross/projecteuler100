
// Largest prime factor
   
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

pub fn run(number: i64 ) -> i64 {
    let mut max_prime_factor: i64 = 0;
    let largest_possible_factor = number / 2;

    let mut range: Vec<i64>  = (2..=largest_possible_factor).filter(|n| not_multiple_of_low_primes(n)).collect();
    let mut num : i64;
    let mut max : i64;
    match range.iter().next(){       
        Some(val) => {
            num = *val;
        }
        None => { num = 0 ;}
    }

    match range.iter().max(){       
        Some(val) => {
            max = *val;
        }
        None => { max = 0 ;}
    }
    //println!("{:?}",range);
    while num <= max && num != 0 {
        //move all multiples from array for current number
        range = range.into_iter().filter(|n| *n % num != 0 && num != *n).collect();
        //println!("{:?}",range);
        //replace the max if this number is greater and is a factor. 
        //It should have no factors and would therefore be prime as
        //a previous element would have removed it as a multiple;
        
        if num > max_prime_factor && (number % num) == 0 
        {
            //Remove all found primes that have been surpassed
            match range.binary_search(&max_prime_factor) {
                Ok(removal_index) => range.remove(removal_index),
                Err(_) => {max_prime_factor} 
            };

            //Check if the other divisor which should be greater is a prime
            let other_factor = number/num;
            if is_prime(&other_factor)
            {
                max_prime_factor =  other_factor;
                match range.binary_search(&max_prime_factor) {
                    //Move to where other factor is and continue
                    Ok(iter_index) => range.iter().skip(iter_index),
                    Err(_iter_index) => {range.iter().skip(0)} 
                };
            }
            else
            {
                max_prime_factor =  num;
            }
        } 
        
        

        //Progress

        match range.iter().next(){       
            Some(val) => {
                num = *val;
            }
            None => { num = 0 ;}
        }
    
        match range.iter().max(){       
            Some(val) => {
                max = *val;
            }
            None => { max = 0 ;}
        }
    }
    max_prime_factor
    
}

fn not_multiple_of_low_primes(num: &i64) -> bool
{
    let low_primes = [2,3,5,7,11,13,17];

    //println!("{} {} {}",*num,low_primes.contains(num),low_primes.iter().all(|&lp| *num % lp != 0));

    !low_primes.iter().any(|&lp| *num % lp == 0) || low_primes.contains(num)

}

fn is_prime(num: &i64) -> bool
{
    let range: Vec<i64> = (2..=*num/2).collect();
    //println!("Is prime: {} {:?}",num,range);
    
    !range.iter().any(|&f| *num % f == 0)

}