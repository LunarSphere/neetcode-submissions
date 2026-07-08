use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut dict = HashMap::new();
        for s in strs{
            let mut alpha_freq = [0u8; 26];
            for c in s.bytes(){
                alpha_freq[(c - b'a') as usize] += 1
            } 
            dict.entry(alpha_freq).or_insert(Vec::new()).push(s);
        }
        dict.into_values().collect()

    }
}
