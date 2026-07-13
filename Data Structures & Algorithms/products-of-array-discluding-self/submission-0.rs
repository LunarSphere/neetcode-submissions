impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut prefix = vec![0;nums_len]; 
        let mut suffix = vec![0;nums_len];
        let mut result = vec![0;nums_len]; 

        //build prefix array
        for i in 0..nums_len{
            if i == 0{
                prefix[i] = 1;
            }else{prefix[i] = nums[i-1] * prefix[i-1];}
            
        }

        // build suffix array
        for i in (0..nums_len).rev(){
            if i == nums_len - 1{
                suffix[i] = 1;
            }else{suffix[i] = nums[i+1] * suffix[i+1];}
            
        }

        // compute result
        for i in 0..nums_len{
            // multiplying prefix[i] by suffix[i] should give us result[i]
            result[i] = prefix[i] * suffix[i];
        }
        result 
    }
}
