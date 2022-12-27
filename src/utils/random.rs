use rand::Rng;

pub fn get_random_num(nums: Vec<u8>) -> (u8, u8, u8) {
    let limit = nums[1];
    let init = nums[0];

    let num = rand::thread_rng().gen_range(init..=limit);

    (num, init, limit)
}
