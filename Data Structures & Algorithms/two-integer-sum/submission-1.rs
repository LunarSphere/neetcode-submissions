impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len(){
            for j in i+1..nums.len(){
                if nums[i] == target-nums[j]{return vec![i.try_into().unwrap(),j.try_into().unwrap()];}
            }
        }
        return vec![0,0]
    }
}
