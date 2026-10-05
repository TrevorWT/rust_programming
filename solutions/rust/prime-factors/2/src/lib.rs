pub fn factors(n: u64) -> Vec<u64> {
    let mut l: Vec<u64> = Vec::new();
        if n < 2 {
        return l;
    }

    let mut m = n;
    let mut i = 2;
    while i <= m {
        if m.is_multiple_of(i) {
            m /= i;
            l.push(i);
        } else {
        i += 1;
        }
    }
    l
    
}

