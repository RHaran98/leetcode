use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn encode(strings: Vec<String>) -> String {
        let mut password = String::new();
        for s in strings.iter() {
            let length = (s.chars().count() as u32);
            password.push(char::from_u32(length).unwrap());
            password.push_str(s);

        }
        password
    }

    pub fn decode(str: String, ) -> Vec<String> {
        let mut strings: Vec<String> = Vec::new();
        
        let curr_char:u32 = 0;
        // let mut s = String::new();
        
        let mut s_iter = &mut str.chars().into_iter();
        let mut pass_size = match s_iter.next() {
            Some(c) => (c as usize),
            None => {return strings;0}
        };

        loop {
            let s = s_iter.take(pass_size).collect::<String>();
            strings.push(s);
            pass_size = match s_iter.next() {
                Some(num) => num as usize,
                None =>  {return strings;0}
            };


        }
    }
}

fn main()  {
    let strings = vec![
        "hello1".to_string(),
        "whatsup".to_string()

    ];

    let s = Solution::encode(strings);

    println!("bf {:?}",s);
    let comp_strings = Solution::decode(s);
    println!("af {:?}",comp_strings);

}