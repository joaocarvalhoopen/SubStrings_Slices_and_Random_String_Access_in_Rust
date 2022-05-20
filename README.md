# SubStrings, Slices and Random String Access in Rust
This is a simple way to do it.

## Description

* Rust string processing is kind of hard, because text in a UTF-8 world has many complex details, and Rust exposes all that power and all that complexity to you, the programmer. Sometimes it can be over whelming. Sometimes you only want to have a simple ```Vec<char>``` but with many String functions, a simple substring or a slice and you don’t mind to pay the performance cost for that, because you really need this feature and the Standard Library doesn’t help you a lot there.

* Fortunately **carlomilanesi** made this code available to all <br>
[https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11](https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11)

* But if you have to do many text operations based on the positions of chars inside a strings this isn’t a really good option, because you have to scan all the strings to the correct position, from the start, to have the string divided it into the correct boundaries of the chars. In this context, you would happily pay a up front cost of transforming the string into a Vec<char>, Vec of chars with individual chars separated, and process it as positional chars with access cost of 1 and then, slice them, range them, append to them at the end (or if you need to append in the start or the middle paying the cost of copy to a new buffer, but you can do it if you need to). **The following code is my expansion** to the code of **carlomilanesi**. It will allow you to do it. <br>

* In a second phase I extended greatly the number of methods and functions supported by ```Vec<char>```, as a internal (to the program) implemented trait on the external struct type Vector of chars. It has random access with O(1) performance. <br>

* The only dependency of this code is on the Std and on the Crate unic_normal - UNIC — Unicode Normalization Forms . <br>


``` Rust
use core::panic;
use std::ops::{Bound, RangeBounds};
use std::iter;
use std::mem;
use std::collections::HashMap;

extern crate unic_normal;
use unic_normal::StrNormalForm;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
    fn get_vec_chars(&self) -> Vec<char>;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }

    fn get_vec_chars(&self) -> Vec<char> {
        // return self.chars().collect();
        let tmp_str = self.nfc().collect::<String>();
        tmp_str.chars().collect()
    }
}

trait StringUtilsVecChars {
    fn to_string(&self) -> String;
    fn to_string_buf<'a>(&self, buf: & 'a mut String) -> & 'a String;
}

impl StringUtilsVecChars for Vec<char> {
    fn to_string(&self) -> String { 
        self.iter().collect()
    }

    fn to_string_buf<'a>(&self, buf: & 'a mut String) -> & 'a String {
        buf.clear();
        for c in self.iter() {
            buf.push(*c);
        }
        buf
    }
}

trait StringUtilsSlices {
    fn to_string(&self) -> String;
    fn to_string_buf<'a>(&self, buf: & 'a mut String) -> & 'a String;
    fn to_vec_chars(&self) -> Vec<char>;
}

impl StringUtilsSlices for [char] {
    fn to_string(&self) -> String {
        self.iter().collect()
    }

    fn to_string_buf<'a>(&self, buf: & 'a mut String) -> & 'a String {
        buf.clear();
        for c in self.iter() {
            buf.push(*c);
        }
        buf
    }

    fn to_vec_chars(&self) -> Vec<char> {
        let vec_chars = self.iter().copied().collect();
        vec_chars
    }
}

trait StringUtilsVecCharsV2 {
    // fn to_string(&self) -> String;
    // fn to_string_buf<'a>(&self, buf: & 'a mut String) -> & 'a String;
    
    fn join_vec(p_vec_vec_chars: &[&[char]]) -> Vec<char>;
    fn join_str(p_vec_str: &[&str]) -> Vec<char>;
 
    fn eq_vec(&self, other: &[char]) -> bool;
    fn eq_str(&self, p_str: &str) -> bool;

    fn push_vec(& mut self, p_vec_chars: &[char]);
    fn push_str(& mut self, p_str: &str);
    fn push_str_start(& mut self, p_str: &str);
    fn push_vec_start(& mut self, other_vec: &Vec<char>);
    fn insert_str(& mut self, p_str: &str, at_pos: usize)
        -> Result<(), String>;
    fn insert_vec(& mut self, other_vec: &Vec<char>, at_pos: usize) 
        -> Result<(), String>;

    fn trim_start(& mut self);
    fn trim_end(& mut self);
    fn trim(& mut self);

    fn find_vec(& self, p_vec_chars: &Vec<char>,
        start_pos: usize, end_pos: Option<usize>)
        -> Option<usize>;
    fn find_str(& self, p_str: &str, start_pos: usize,
                end_pos: Option<usize>) -> Option<usize>;

    fn contains_vec(& self, p_vec_chars: &Vec<char>) -> bool;
    fn contains_str(& self, p_str: &str) -> bool;

    fn start_with_vec(& self, pattern_vec_chars: &[char]) -> bool;
    fn start_with_str(& self, pattern_str: &str) -> bool;
    fn ends_with_vec(& self, pattern_vec_chars: &[char]) -> bool;
    fn ends_with_str(& self, pattern_str: &str) -> bool;

    /// Returns a None or the index of the first replace.
    fn replace_vec(& mut self, match_pattern_vec: &Vec<char>,
                    replace_pattern_vec: &Vec<char>, 
                    start_pos: usize, 
                    end_pos: Option<usize>) -> Option<usize>;
    /// Returns a None or the index of the first replace.
    fn replace_str(& mut self, match_pattern_str: &str,
                    replace_pattern_str: &str, start_pos: usize,
                    end_pos: Option<usize>) -> Option<usize>;
    
    /// Returns a None or the number of replaces.
    fn replace_vec_all(& mut self, match_pattern_vec: &Vec<char>,
                       replace_pattern_vec: &Vec<char>) -> Option<usize>;
    /// Returns a None or the number of replaces.
    fn replace_str_all(& mut self, match_pattern_str: &str,
                       replace_pattern_str: &str) -> Option<usize>;
    
    fn split_vec(& self, at_pattern: &Vec<char>) -> Vec<&[char]>;        
    fn split_str(& self, at_pattern_str: &str) -> Vec<&[char]>;

    fn map_str(& mut self, map: & HashMap<&str, &str>) 
        -> HashMap<String, usize>;
}


fn main() {
    let s = "abcdèfghij";
    // All three statements should print:
    // "abcdè, abcdèfghij, dèfgh, dèfghij."
    println!("{}, {}, {}, {}.",
        s.substring(0, 5),
        s.substring(0, 50),
        s.substring(3, 5),
        s.substring(3, 50));
    println!("{}, {}, {}, {}.",
        s.slice(..5),
        s.slice(..50),
        s.slice(3..8),
        s.slice(3..));
    println!("{}, {}, {}, {}.",
        s.slice(..=4),
        s.slice(..=49),
        s.slice(3..=7),
        s.slice(3..));

    // Allocating a string from Vec<char>.
    let mut vc = s.get_vec_chars(); 
    println!("{}, {}, {}, {}.",
        vc[..5].to_string(),
        vc.to_string(),
        vc[3..8].to_string(),
        vc[3..].to_string());

    // Reusing a String buffer from a Vec<char>.
    let mut buf = String::new();
    print!("{}, ",  vc[..5].to_string_buf(& mut buf));
    print!("{}, ",  vc[..].to_string_buf(& mut buf));
    print!("{}, ",  vc[3..8].to_string_buf(& mut buf));
    print!("{}.\n", vc[3..].to_string_buf(& mut buf));
    
    // Random access to the Vec<char>. 
    for i in 0..(vc.len() - 2) {
        print!("{} ", vc[i..i+3].to_string_buf(& mut buf));
    }
    println!("");
    
    // Random modifications to the Vec<char>.
    for i in (0..(vc.len() / 3) + 1).rev() {
        vc.insert(i*3, '#');
    }
    println!("{} ", vc.to_string());
    println!("{} ", vc.to_string_buf(& mut buf));

    // Test trait StringUtilsVecCharsV2 for Vec<char> .
    test_vec_char_methods();
}


// Output:
//    abcdè, abcdèfghij, dèfgh, dèfghij.
//    abcdè, abcdèfghij, dèfgh, dèfghij.
//    abcdè, abcdèfghij, dèfgh, dèfghij.
//
//    abcdè, abcdèfghij, dèfgh, dèfghij.
//    abcdè, abcdèfghij, dèfgh, dèfghij.
//    abc bcd cdè dèf èfg fgh ghi hij 
//    #abc#dèf#ghi#j
//    #abc#dèf#ghi#j
```


## License

* MIT Open Source License.


## My Rust guide

* **How to learn modern Rust** <br>
  [https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust](https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust)


## All my other guides

* The links to all my guides are in: <br>
  **Guides on Linux - Programming - Embedded - Electronics - Aeronautics** <br>
  [https://github.com/joaocarvalhoopen/Guides_Linux-Programming-Electronics-Aeronautics](https://github.com/joaocarvalhoopen/Guides_Linux-Programming-Electronics-Aeronautics)


## Have fun!
Best regards, <br>
Joao Nuno Carvalho
