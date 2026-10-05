pub fn collatz(n: u64) -> Option<u64> {
    let mut a = n;
    let mut b: Option<u64> = Some(0);
    if a <= 0 {return None;}
    
    while a != 1 {
        if a % 2 == 0 {
            for val in &mut b {
                *val += 1;
            }
            a = a/2;
        } else  {
            for val in &mut b {
                *val += 1;
            }
            a = (a*3)+1;
        }
    } 
    return b;
}
