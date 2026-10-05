pub fn collatz(n: u64) -> Option<u64> {
    let mut a = n;
    let mut b: Option<u64> = Some(0);
    if a == 0 {return None;}
    
    while a != 1 {
        if a.is_multiple_of(2) {
            if let Some(val) = &mut b {
                *val += 1;
            }
            a/=2;
        } else  {
            if let Some(val) = &mut b {
                *val += 1;
            }
            a = (a*3)+1;
        }
    } 
    b
}
