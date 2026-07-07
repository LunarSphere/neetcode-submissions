use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dict = HashMap::new();
        for num in nums{
            *dict.entry(num).or_insert(0) +=1;
        }
        for val in dict.values(){
            if *val > 1 {return true;}
        }
        return false;
    }

}
