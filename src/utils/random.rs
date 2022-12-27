use rand::Rng;

pub fn get_random_num(nums: Vec<u32>) -> (u32, u32, u32) {
    let limit = nums[1];
    let init = nums[0];

    let num = rand::thread_rng().gen_range(init..=limit);

    (num, init, limit)
}
