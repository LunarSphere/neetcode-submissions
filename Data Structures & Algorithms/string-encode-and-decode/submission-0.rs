impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_string = String::new();
        for s in strs{
            encoded_string.push_str(&format!("{}#{}",s.len(), s))
        }
        encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded_string: Vec<String> = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes(); // rust string cannot be indexed using s[j]
        while i < s.len(){
            let mut j = i;
            while bytes[j] != b'#'{  // needs to be a byte
                j+=1
            }
            let str_len: usize = s[i..j].parse().unwrap();
            let start = j+1;
            let end = j+1 + str_len;
            decoded_string.push(s[start..end].to_string()); // without to string we push &str
            i = end;
        }
        decoded_string
    }
}
