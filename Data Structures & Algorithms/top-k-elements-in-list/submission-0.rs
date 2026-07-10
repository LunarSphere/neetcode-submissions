use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let nums_len = nums.len();
        let mut freqs = HashMap::new(); 
        // create an array of vectors the length of nums
        let mut buckets = vec![Vec::new(); nums_len + 1];
        //figure out how frequnt the numbers are
        for num in nums{
            *freqs.entry(num).or_insert(0) += 1; 
        }

        for (num, freq) in freqs{
            buckets[freq].push(num)
        }
        //flatten buckets
        let mut flat_buckets = Vec::new();
        for i in (1..buckets.len()).rev(){
            flat_buckets.extend(buckets[i].iter().copied());
        }
        // cut ints that arent in top k
        flat_buckets.truncate(k as usize);
        flat_buckets
    }

}
