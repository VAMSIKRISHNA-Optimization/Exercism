pub fn factors(n: u64) -> Vec<u64> 
{
    let mut ans: Vec<u64> = Vec::new();
    
    let mut num = n;
    let mut div = 2;
    while num != 1
    {
        
        if is_prime(div) && num % div == 0
        {
            num = num/div;
            ans.push(div);
        }
        else
        {
            div += 1;
        }
    }
    
    ans
    
}

fn is_prime(n: u64) -> bool 
{
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut i = 5;
    // Check up to the square root of n
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}