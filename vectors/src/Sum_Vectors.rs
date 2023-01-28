// fn sum_vectors(nums: Vec<i32>) -> Vec<i32> {
//     let mut sum = 0;
//     nums.iter().map(|i| {
//         sum += 1; sum;
//     }).collect()
// }

// fn main() {
//     let numbers: Vec<i32> = vec![1, 2, 3, 4];

//     sum_vectors(numbers);
// }
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|i| {
                sum += i;
                sum
            })
            .collect()
    }
}
