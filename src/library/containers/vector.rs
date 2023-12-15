// Contains all vector-related functions for the library, so slicing, concatenating, etc.
#[allow(unused_imports)]
use num_traits::{one, zero, FromPrimitive, Unsigned};

// Function to go from a generic vector of numbers to a long integer maximum value of that type else throw an error.
// So for example if you pass it a [4, 21, 4, 2] it gives back a 42142 as the value. Do so without any string conversions.
#[allow(unused)]
pub fn concat_slice_to_num<T: Unsigned + FromPrimitive + PartialOrd + Clone>(nums: &[T]) -> T {
    let mut together = zero();
    let mut power: T = one();

    for num in nums.iter().rev() {
        together = together + num.clone() * power.clone();
        power = power.clone() * calculate_next_power_ten(num);
    }

    together
}

fn calculate_next_power_ten<T: Unsigned + FromPrimitive + PartialOrd>(num: &T) -> T {
    let mut power = T::from_u8(10).unwrap();
    while &power <= num {
        power = power * T::from_u8(10).unwrap();
    }
    power
}