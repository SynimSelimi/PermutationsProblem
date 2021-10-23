// Test in Rust Playground
// https://play.rust-lang.org/
pub fn permute(perms: &mut Vec<Vec<i32>>, nums: Vec<i32>, start: usize) {
    if start >= nums.len() {
        println!("{:?} {:?}", nums, start);
        perms.push(nums);
        return
    }
    for i in start..nums.len() {
        println!("{:?} loop {:?}", nums, start);
        let mut nums = nums.clone();
        nums.swap(start, i);
        permute(perms, nums, start + 1);
    }
}

fn main() {
    let numbers:Vec<i32> = vec![1,2];
    let mut permutations: Vec<Vec<i32>> = vec![];
    permute(&mut permutations, numbers, 0);
    println!("{:?}", permutations);
}
