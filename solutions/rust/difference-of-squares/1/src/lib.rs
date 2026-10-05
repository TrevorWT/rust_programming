pub fn square_of_sum(n: u32) -> u32 {
    let mut j = 0;
    for i in 1..=n {
        j += i;
    }
    j.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut j = 0;
    for i in 1..=n {
        j += i.pow(2);
    }
    j
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
