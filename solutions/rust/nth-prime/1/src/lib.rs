pub fn nth(n: u32) -> u32 
{
    let mut num_to_chk: u32 = 2;
    let mut iter = 1;
    
    while iter <= n+1 
    {
        if is_prime(num_to_chk)
        {
            iter += 1;
            
            if iter > n+1
            {
                break;
            }
            else
            {
                num_to_chk += 1;
            }
            
        }
        else
        {
            num_to_chk += 1;
        }
        
    }
    num_to_chk
    
}

fn is_prime(n: u32) -> bool 
{
    if n <= 1 { return false; }
    
    // Check for divisors from 2 up to the square root of n
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit 
    {
        if n % i == 0 
        {
            return false;
        }
    }
    true
}
