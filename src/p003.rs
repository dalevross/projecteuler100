
// Largest prime factor
   
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

pub fn run(number: i64) -> i64 {

    get_largest_prime_factor(number,2)
    
}

fn get_largest_prime_factor(mut number: i64, start: i64) -> i64
{
    println!("Number: {} Start: {}",number,start);
    
    let largest_possible_factor = (number as f64).sqrt() as i64;

    let mut range: Vec<i64>  = (start..=largest_possible_factor).collect();
    let mut numoption: Option<&i64> = range.iter().next();
    
    while number != 1 && numoption != None {
        
        let num:i64 = *numoption.unwrap();
        //move all multiples from array for current number
        range = range.into_iter().filter(|mul| *mul % num != 0 && num != *mul).collect();
        
        
        //replace the max if this number is greater and is a factor. 
        //It should have no factors and would therefore be prime as
        //a previous element would have removed it as a multiple;
        
        if (number % num) == 0 
        {  
            //wipe out all the current factor leaving only other factor 
            //Eg 24 = 2 x 2 x 2 x 3. Here we'll divide until only 3 is left
            while number % num == 0
            {
                number = number/ num;
            }         
          
            let lpf_remainder = get_largest_prime_factor(number,num);
            return if lpf_remainder > num {lpf_remainder} else {num};

            
        } 

        numoption = range.iter().next();
        
    }
    number
   
    
}

