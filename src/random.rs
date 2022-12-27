use core::fmt;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid array init cannot be greater than limit - Array invalido init no puede ser mas grande que el limit ")
    }
}

pub fn get_random_num(nums: Vec<u32>) -> Result<(u32, u32, u32), EmptyError> {
    let limit = nums[1];
    let init = nums[0];

    // manage case 3..2
    if init > limit {
        return Err(EmptyError);
    }

    let num = rand::thread_rng().gen_range(init..=limit);

    let result = (num, init, limit);

    Ok(result)
}
