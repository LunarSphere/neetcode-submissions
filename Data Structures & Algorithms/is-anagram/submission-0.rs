use std::collections::HashMap;



impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut dict1 = HashMap::new();
        let mut dict2 = HashMap::new();
        for c in s.chars(){
            *dict1.entry(c).or_insert(0) +=1;
        }
        for c in t.chars(){
            *dict2.entry(c).or_insert(0) +=1;
        }
        dict1 == dict2
    }
}
