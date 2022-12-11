use std::ops;
pub fn get_hcf<T>(left: T, right: T) -> T
where
    T: ops::Div<Output = T>
        + ops::Mul<Output = T>
        + ops::Sub<Output = T>
        + Eq
        + Default
        + Copy
        + PartialOrd,
{
    let mut current_left;
    let mut current_right;

    if left >= right {
        current_left = left;
        current_right = right;
    } else {
        current_left = right;
        current_right = left;
    }

    let mut prev_reamainder = current_left;
    let mut remainder: T = current_left;
    while remainder != Default::default() {
        let mutiple = current_left / current_right;
        prev_reamainder = remainder;
        remainder = current_left - (mutiple * current_right);

        current_left = current_right;
        current_right = remainder;
    }

    prev_reamainder
}

pub fn get_lcm<T>(left: T, right: T) -> T
where
    T: ops::Div<Output = T>
        + ops::Mul<Output = T>
        + ops::Sub<Output = T>
        + Eq
        + Default
        + Copy
        + PartialOrd,
{
    let product = left * right;

    product / get_hcf(left, right)
}
