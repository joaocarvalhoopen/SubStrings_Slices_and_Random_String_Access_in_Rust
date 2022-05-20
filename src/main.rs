/******************************************************************************
 * Project:     Random String Access in Rust - Vec<char>
 * Author:      João Nuno Carvalho
 * Date:        2022.05.19
 * Description: This small project was made to ease Text Processing in Rust for
 *              people coming from other Programming Languages like Python.
 *              With this traits, implemented on a program you can access
 *              Vec<char>  at O(1) performance for Random Access and with more
 *              ease then with normal Strings in Rust, at least for the
 *              beginner.
 *              I implemented in here many functions as traits over Vec<char>,
 *              &[char], String and &str. They are implemented with somewhat
 *              medium eye, regarding performance. They should not be the
 *              fastest in the FarWest, except for Random Access normal Rust
 *              String should be faster, but for the many cases they are
 *              alright.
 * Note:        See the README.md of the project page on my github for the 
 *              details of a couple of small function that came from example
 *              of other developer.
 * License:     MIT Open Source License.
 * 
 * Have fun! 
 *****************************************************************************/

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

/*
    Output:
       abcdè, abcdèfghij, dèfgh, dèfghij.
       abcdè, abcdèfghij, dèfgh, dèfghij.
       abcdè, abcdèfghij, dèfgh, dèfghij.
       abcdè, abcdèfghij, dèfgh, dèfghij.
       abcdè, abcdèfghij, dèfgh, dèfghij.
       abc bcd cdè dèf èfg fgh ghi hij 
       #abc#dèf#ghi#j
       #abc#dèf#ghi#j
*/


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
    fn insert_str(& mut self, p_str: &str, at_pos: usize) -> Result<(), String>;
    fn insert_vec(& mut self, other_vec: &Vec<char>, at_pos: usize) -> Result<(), String>;

    fn trim_start(& mut self);
    fn trim_end(& mut self);
    fn trim(& mut self);

    fn find_vec(& self, p_vec_chars: &Vec<char>, start_pos: usize, end_pos: Option<usize>) -> Option<usize>;
    fn find_str(& self, p_str: &str, start_pos: usize, end_pos: Option<usize>) -> Option<usize>;

    fn contains_vec(& self, p_vec_chars: &Vec<char>) -> bool;
    fn contains_str(& self, p_str: &str) -> bool;

    fn start_with_vec(& self, pattern_vec_chars: &[char]) -> bool;
    fn start_with_str(& self, pattern_str: &str) -> bool;
    fn ends_with_vec(& self, pattern_vec_chars: &[char]) -> bool;
    fn ends_with_str(& self, pattern_str: &str) -> bool;

    /// Returns a None or the index of the first replace.
    fn replace_vec(& mut self, match_pattern_vec: &Vec<char>, replace_pattern_vec: &Vec<char>, start_pos: usize, end_pos: Option<usize>) -> Option<usize>;
    /// Returns a None or the index of the first replace.
    fn replace_str(& mut self, match_pattern_str: &str, replace_pattern_str: &str, start_pos: usize, end_pos: Option<usize>) -> Option<usize>;
    
    /// Returns a None or the number of replaces.
    fn replace_vec_all(& mut self, match_pattern_vec: &Vec<char>, replace_pattern_vec: &Vec<char>) -> Option<usize>;
    /// Returns a None or the number of replaces.
    fn replace_str_all(& mut self, match_pattern_str: &str, replace_pattern_str: &str) -> Option<usize>;
    
    fn split_vec(& self, at_pattern: &Vec<char>) -> Vec<&[char]>;        
    fn split_str(& self, at_pattern_str: &str) -> Vec<&[char]>;

    fn map_str(& mut self, map: & HashMap<&str, &str>) -> HashMap<String, usize>;
}

impl StringUtilsVecCharsV2 for Vec<char> {
    
/*
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
*/

    fn join_vec(p_vec_vec_chars: &[&[char]]) -> Vec<char> {
        // Calculate the total length of the strings.
        let mut capacity = 0_usize;
        for src_vec_chars_tmp in p_vec_vec_chars {
            capacity += src_vec_chars_tmp.len();
        }
        // Allocate the memory on the heap.
        let mut vec_chars: Vec<char> = Vec::with_capacity(capacity);
        for src_vec_chars_tmp in p_vec_vec_chars {
            vec_chars.push_vec(src_vec_chars_tmp);
        }
        vec_chars
    }

    fn join_str(p_vec_str: &[&str]) -> Vec<char> {
        // Calculate the total length of the strings.
        let mut capacity = 0_usize;
        for str_tmp in p_vec_str {
            for _ in str_tmp.chars() {
                capacity += 1;
            }
        }
        // Allocate the memory on the heap.
        let mut vec_chars: Vec<char> = Vec::with_capacity(capacity);
        for str_tmp in p_vec_str {
            vec_chars.push_str(str_tmp);
        }
        vec_chars
    } 

    #[inline]
    fn eq_vec(&self, other: &[char]) -> bool {
        self.len() == other.len() && iter::zip(self, other).all(|(a, b)| *a == *b)
    }

    #[inline]
    fn eq_str(&self, p_str: &str) -> bool {
        self.len() == p_str.chars().count() && iter::zip(self, p_str.chars()).all(|(a, b)| *a == b)
    }

    fn push_vec(& mut self, p_vec_chars: &[char]) {
        self.extend(p_vec_chars);
    }
  
    fn push_str(& mut self, p_str: &str) {
        let vec_chars = p_str.get_vec_chars();
        self.extend(vec_chars);
    }
    
    fn push_str_start(& mut self, p_str: &str) {              
        let mut vec_chars = p_str.get_vec_chars();
        vec_chars.extend(self.iter());
        let _ = mem::replace(self, vec_chars);
    }

    fn push_vec_start(& mut self, other_vec: &Vec<char>) {
        let mut vec_tmp = other_vec.clone();
        vec_tmp.extend(self.iter());
        let _ = mem::replace(self, vec_tmp);
    }

    fn insert_str(& mut self, p_str: &str, at_pos: usize) -> Result<(), String> {
        if at_pos >= self.len() {
            return Err("Error: In insert_str(), parameter at_pos is greater then sel.len() - 1".to_string());
        }
        let vec_t1: Vec<char> = p_str.get_vec_chars();
        let mut vec_tmp: Vec<char> = Vec::with_capacity(self.len() + vec_t1.len());
        vec_tmp.extend(self[..at_pos].iter());
        vec_tmp.extend(vec_t1.iter());
        vec_tmp.extend(self[at_pos..].iter());
        let _ = mem::replace(self, vec_tmp);
        Ok(())
    }

    fn insert_vec(& mut self, other_vec: &Vec<char>, at_pos: usize) -> Result<(), String> {
        
        if at_pos >= self.len() {
            return Err("Error: In insert_str(), parameter at_pos is greater then sel.len() - 1".to_string());
        }
        //let mut vec_t1: Vec<char> = p_str.chars().collect();
        let mut vec_tmp: Vec<char> = Vec::with_capacity(self.len() + other_vec.len());
        vec_tmp.extend(self[..at_pos].iter());
        vec_tmp.extend(other_vec.iter());
        vec_tmp.extend(self[at_pos..].iter());
        let _ = mem::replace(self, vec_tmp);
        Ok(())
    }

    fn trim_start(& mut self) {
        if self.is_empty() {
            return;
        }
        let mut start_index = 0;
        let mut i = 0;
        while i < self.len() && self[i].is_whitespace() {
            start_index = i;
            i += 1;
        }
        if i < self.len() - 1 {
            self.copy_within((start_index + 1).., 0);
            for _ in 0..i {
                self.pop();
            }
        } else {
            self.clear();
        }
    }

    fn trim_end(& mut self) {
        let mut i = self.len() as i32 - 1;
        while i >= 0 && self[i as usize].is_whitespace() {
            let _ = self.pop();
            i -= 1;
        }
    }

    fn trim(& mut self) {
        let mut vec_tmp: Vec<char> = Vec::with_capacity(self.len());
        let mut flag_starting_white_spaces = true;
        for c in self.iter() {
            if !c.is_whitespace() {
                flag_starting_white_spaces = false;
            }
            if !flag_starting_white_spaces {
                vec_tmp.push(*c);
            }
        }
        vec_tmp.trim_end();
        let _ = mem::replace(self, vec_tmp);
    }


    fn find_vec(& self, p_vec_chars: &Vec<char>, start_pos: usize, end_pos: Option<usize>) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        if start_pos >= self.len() {
            panic!("Error: In find_str() parameter start_pos must not be greater then Vec<char>.len() - 1 .");
        }
        let end_pos_val = if let Some(val) = end_pos {
                if val >= self.len() {
                    panic!("Error: In find_str() parameter end_pos must not be greater then Vec<char>.len() - 1 .");
                }
                if val < start_pos {
                    panic!("Error: In find_str() parameter end_pos cannot be lower then parameter start_pos.");
                } 
                val
            } else {
                self.len() - 1
            };
        if p_vec_chars.is_empty() {
            return None;
        }
        // let pattern_vec: Vec<char> = p_str.chars().collect(); 
        let pattern_vec = p_vec_chars; 
        if  pattern_vec.len() + start_pos > self.len() {
            return None;
        }

        // Find pattern inside string.
        let match_pos: usize;
        // let mut flag_match = false;
        for i in start_pos..=end_pos_val {
            let mut counter = pattern_vec.len();
            let mut offset = 0_usize;
            for c in pattern_vec.iter() {
                if self[i + offset] != *c {
                    break;
                }
                offset += 1;
                counter -= 1
            }
            if counter == 0 {
                // flag_match = true;
                match_pos = i;
                return Some(match_pos);
            }
        }
        None
    }

    fn find_str(& self, p_str: &str, start_pos: usize, end_pos: Option<usize>) -> Option<usize> {
        let pattern_vec_chars: Vec<char> = p_str.get_vec_chars(); 
        self.find_vec(&pattern_vec_chars, start_pos, end_pos)
    }

    fn contains_vec(& self, p_vec_chars: &Vec<char>) -> bool {
        if self.find_vec(p_vec_chars, 0, None).is_some() {
            return true;
        }
        false
    }

    fn contains_str(& self, p_str: &str) -> bool {
        let vec_chars = p_str.get_vec_chars();
        self.contains_vec(&vec_chars)
    }



    fn start_with_vec(& self, pattern_vec_chars: &[char]) -> bool {
        if pattern_vec_chars.len() > self.len() {
            return false;
        }
        self.starts_with(pattern_vec_chars)
    }


    fn start_with_str(& self, pattern_str: &str) -> bool {
        let pattern_vec_chars = pattern_str.get_vec_chars();
        self.start_with_vec(&pattern_vec_chars)
    }

    fn ends_with_vec(& self, pattern_vec_chars: &[char]) -> bool {
        if pattern_vec_chars.len() > self.len() {
            return false;
        }
        self.ends_with(pattern_vec_chars)
    }

    fn ends_with_str(& self, pattern_str: &str) -> bool {
        let pattern_vec_chars = pattern_str.get_vec_chars();
        self.ends_with_vec(&pattern_vec_chars)
    }



    /// Returns a None or the index of the first replace.
    fn replace_vec(& mut self, match_pattern_vec: &Vec<char>, replace_pattern_vec: &Vec<char>, start_pos: usize, end_pos: Option<usize>) -> Option<usize> {
        let res = self.find_vec(match_pattern_vec, start_pos, end_pos);
        if let Some(index) = res {
            for _ in 0..(match_pattern_vec.len()) {
                self.remove(index);
            }
            let _ = self.insert_vec(replace_pattern_vec, index);
            return Some(index);
        }
        None
    }

    /// Returns a None or the index of the first replace.
    fn replace_str(& mut self, match_pattern_str: &str, replace_pattern_str: &str, start_pos: usize, end_pos: Option<usize>) -> Option<usize> {
        let match_pattern_vec = match_pattern_str.get_vec_chars();
        let replace_pattern_vec = replace_pattern_str.get_vec_chars();
        self.replace_vec(&match_pattern_vec, &replace_pattern_vec, start_pos, end_pos)
    }

    /// Returns a None or the number of replaces.
    fn replace_vec_all(& mut self, match_pattern_vec: &Vec<char>, replace_pattern_vec: &Vec<char>) -> Option<usize> {
        let mut flag_ended_find = false;
        let mut next_start_pos = 0_usize;
        let mut indexes_vec: Vec<usize> = Vec::new(); 
        // Find, from start to end, the indexes of the machs. Put's them on a Vec.
        while !flag_ended_find {
            if next_start_pos >= self.len() {
                // flag_ended_find = true;
                break;
            }
            let res = self.find_vec(match_pattern_vec, next_start_pos, None);
            if let Some(index) = res {
                indexes_vec.push(index);
                next_start_pos = index + match_pattern_vec.len();                
            } else {
                flag_ended_find = true;
            }
        }
        // Case where it didn't found any match, it will exit earlier.
        if indexes_vec.is_empty() {
            return None;
        }

        // We will copy to a new Vec<char> the data and do the replacement when coping.
        let num_matches = indexes_vec.len();
        let capacity = self.len() - num_matches * match_pattern_vec.len() + num_matches * replace_pattern_vec.len(); 
        let mut target_vec_chars: Vec<char> = Vec::with_capacity(capacity);
        let mut last_index = 0_usize;
        for (counter,index) in indexes_vec.iter().enumerate() {
            // Copy the first chars before the first match.
            if last_index < self.len() {
                target_vec_chars.push_vec(&self[last_index..*index]);
                target_vec_chars.push_vec(replace_pattern_vec);
                last_index = index + match_pattern_vec.len();
                if counter == indexes_vec.len() - 1 && last_index < self.len() {
                    target_vec_chars.push_vec(&self[last_index..]);
                }
            }
        }

        // Copia a zona de memoria do src para o target self.
        let _ = mem::replace(self, target_vec_chars);

        Some(num_matches)
    }

    /// Returns a None or the number of replaces.
    fn replace_str_all(& mut self, match_pattern_str: &str, replace_pattern_str: &str) -> Option<usize> {
        self.replace_vec_all(&match_pattern_str.get_vec_chars(),
                           &replace_pattern_str.get_vec_chars())
    }



    fn split_vec(& self, at_pattern_vec: &Vec<char>) -> Vec<&[char]> {
        let match_pattern_vec = at_pattern_vec;
        let mut flag_ended_find = false;
        let mut next_start_pos = 0_usize;
        let mut indexes_vec: Vec<usize> = Vec::new(); 
        // Find, from start to end, the indexes of the machs. Put's them on a Vec.
        while !flag_ended_find {
            if next_start_pos >= self.len() {
                // flag_ended_find = true;
                break;
            }
            let res = self.find_vec(match_pattern_vec, next_start_pos, None);
            if let Some(index) = res {
                indexes_vec.push(index);
                next_start_pos = index + match_pattern_vec.len();                
            } else {
                flag_ended_find = true;
            }
        }

        let mut res_vec: Vec<&[char]> = Vec::new();
        // Case where it didn't found any match, it will exit earlier.
        if indexes_vec.is_empty() {
            return res_vec;
        }

        // Join the intervals between splits that have chars, that are not the split chars.
        let mut last_index = 0_usize;
        for (counter, index) in indexes_vec.iter().enumerate() {
            // Copy the first chars before the first match.
            if last_index < self.len() {
                let slice_tmp = &self[last_index..*index];
                if !slice_tmp.is_empty() {
                    res_vec.push(slice_tmp);
                }
                last_index = index + match_pattern_vec.len();
                if counter == indexes_vec.len() - 1 && last_index < self.len() {
                    let slice_tmp = &self[last_index..];
                    if !slice_tmp.is_empty() {
                        res_vec.push(slice_tmp);
                    }   
                }
            }
        }

        res_vec
    }

    fn split_str(& self, at_pattern_str: &str) -> Vec<&[char]> {
        self.split_vec(&at_pattern_str.get_vec_chars())
    }

    fn map_str(& mut self, map: & HashMap<&str, &str>) -> HashMap<String, usize> {
        let mut res_hashmap: HashMap<String, usize> = HashMap::new();
        for (src_str, target_str) in map.iter() {
            let res = self.replace_str_all(src_str, target_str);
            match res {
                Some(num_replaces_for_seg_string) => res_hashmap.insert(src_str.to_string(), num_replaces_for_seg_string),
                None => res_hashmap.insert(src_str.to_string(), 0_usize),
            };
        }
        res_hashmap
    }

}


// *******************************
// *******************************
//              Tests
// *******************************
// *******************************

fn test_vec_char_methods() {


    // @@ Test 1 - eq_vec() and eq_str() .

    let vc_a = "bla".get_vec_chars();
    let vc_b = "bla".get_vec_chars();
    let vc_c = "tu".get_vec_chars();
    
    assert!(vc_a.eq_vec(&vc_b));
    assert!(!vc_a.eq_vec(&vc_c));

    assert!(vc_a.eq_str("bla"));
    assert!(!vc_a.eq_str("tu"));

    assert!(vc_a.eq_str(&"bla".to_string()));
    assert!(!vc_a.eq_str(&"tu".to_string()));

    drop(vc_a);
    drop(vc_b);
    drop(vc_c);


    // @@ Test 2 - Tests on Vec<char> with the reverse of the order of the Vec
    //             with some problematic characters.

    fn fnc(s: &str) -> String {
        let mut str_tmp:Vec<char> = s.get_vec_chars();
        str_tmp.reverse();
        str_tmp.to_string() 
    }

    // With  Crate unic_normal - UNIC — Unicode Normalization Forms

    println!("\nReverse Vec<char> with crate unic_normal - UNIC — Unicode Normalization Forms");

    println!("{} vs {}", fnc("🇸🇪"), "🇸🇪");

    println!("{} vs {}", fnc("noçl"), "lçon");
    println!("{} vs {}", fnc("noãl"), "lãon");
    println!("{} vs {}", fnc("noál"), "láon");
    println!("{} vs {}", fnc("noél"), "léon");
    println!("{} vs {}", fnc("noíl"), "líon");
    println!("{} vs {}", fnc("noàl"), "làon");
    println!("{} vs {}", fnc("noâl"), "lâon");
    println!("{} vs {}", fnc("noêl"), "lêon");

    // Print Out reverse Vec<char> .
    //
    // 🇪🇸 vs 🇸🇪
    // lçon vs lçon
    // lãon vs lãon
    // láon vs láon
    // léon vs léon
    // líon vs líon
    // làon vs làon
    // lâon vs lâon
    // lêon vs lêon
    


    // Without  Crate unic_normal - UNIC — Unicode Normalization Forms

    println!("\nReverse String without crate unic_normal - UNIC — Unicode Normalization Forms");

    fn naive_reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }

    println!("{} vs {}", naive_reverse_string("🇸🇪"), "🇸🇪");

    println!("{} vs {}", naive_reverse_string("noçl"), "lçon");
    println!("{} vs {}", naive_reverse_string("noãl"), "lãon");
    println!("{} vs {}", naive_reverse_string("noál"), "láon");
    println!("{} vs {}", naive_reverse_string("noél"), "léon");
    println!("{} vs {}", naive_reverse_string("noíl"), "líon");
    println!("{} vs {}", naive_reverse_string("noàl"), "làon");
    println!("{} vs {}", naive_reverse_string("noâl"), "lâon");
    println!("{} vs {}", naive_reverse_string("noêl"), "lêon");

    // Print Out reverse String .
    //
    // 🇪🇸 vs 🇸🇪
    // ļcon vs lçon
    // l̃aon vs lãon
    // ĺaon vs láon
    // ĺeon vs léon
    // ĺion vs líon
    // l̀aon vs làon
    // l̂aon vs lâon
    // l̂eon vs lêon
    

    // @@ Test 3 - join_vec() .
    let vc_a: Vec<char> = Vec::join_vec(&[&"bla".get_vec_chars(),
                                                          &"_bli".get_vec_chars(),
                                                          &['_', 'a', 'b', 'c'],
                                                          &"_blu".get_vec_chars()]);
    assert!(vc_a.eq_str("bla_bli_abc_blu"));                                                              
    drop(vc_a);


    // @@ Test 4 - join_str() .
    let vc_a: Vec<char> = Vec::join_str(&["bla",
                                                    "_bli",
                                                    &"_abc".to_string(),
                                                    "_blu"]);
    assert!(vc_a.eq_str("bla_bli_abc_blu"));                                                              
    drop(vc_a);


    // @@ Test 5 - push_str() .
    let mut vc_a = "bla".get_vec_chars();
    vc_a.push_str("bli");
    assert!(vc_a.eq_str("blabli"));
    assert!( !(vc_a.eq_str("bla")) );
    drop(vc_a);
     

    // @@ Test 6 - push_str_start() .
    let mut vc_a = "bla".get_vec_chars();
    vc_a.push_str_start("bli");
    assert!(vc_a.eq_str("blibla"));
    assert!( !(vc_a.eq_str("bla")) );
    drop(vc_a);
    

    // @@ Test 7 - push_vec_start() .
    let mut vc_a = "bla".get_vec_chars();
    let vc_b = "bli".get_vec_chars();
    vc_a.push_vec_start(&vc_b);
    assert!(vc_a.eq_str("blibla"));
    assert!( !(vc_a.eq_str("bla")) );
    drop(vc_a);
    drop(vc_b);


    // @@ Test 8 - insert_str()
    let mut vc_a = "bla".get_vec_chars();
    let res = vc_a.insert_str("bli", 0_usize);
    assert!(res == Ok(()));
    assert!(vc_a.eq_str("blibla"));
    assert!( !(vc_a.eq_str("bla")) );
    let res = vc_a.insert_str("#", 1_usize);
    assert!(res == Ok(()));
    assert!(vc_a.eq_str("b#libla"));
    let res = vc_a.insert_str("#", 1000_usize);
    assert!(res.is_err());
    drop(vc_a);


    // @@ Test 9 - insert_vec()
    let mut vc_a = "bla".get_vec_chars();
    let vc_b = "bli".get_vec_chars();
    let res = vc_a.insert_vec(&vc_b, 0_usize);
    assert!(res == Ok(()));
    assert!(vc_a.eq_str("blibla"));
    assert!( !(vc_a.eq_str("bla")) );
    let res = vc_a.insert_vec(&"#".get_vec_chars(), 1_usize);
    assert!(res == Ok(()));
    assert!(vc_a.eq_str("b#libla"));
    let res = vc_a.insert_vec(&"#".get_vec_chars(), 1000_usize);
    assert!(res.is_err());
    drop(vc_a);
    drop(vc_b);


    // @@ Test 10 - trim_start()
    let mut vc_a = "  \tbla".get_vec_chars();
    vc_a.trim_start();
    // println!("{:?}", vc_a);
    // println!("{}", '\t'.is_whitespace());
    assert!(vc_a.eq_str("bla"));
    assert!( !(vc_a.eq_str("  \tbla")) );
    
    let mut vc_b = "".get_vec_chars();
    vc_b.trim_start();
    assert!( vc_b.eq_str("") );
    
    let mut vc_c = "  ".get_vec_chars();
    vc_c.trim_start();
    // println!("{:?}", vc_c);
    assert!( vc_c.eq_str("") );
    drop(vc_a);
    drop(vc_b);
    drop(vc_c);


    // @@ Test 11 - trim_end()
    let mut vc_a = "bla  \t".get_vec_chars();
    vc_a.trim_end();
    assert!(vc_a.eq_str("bla"));
    assert!( !(vc_a.eq_str("bla  \t")) );
    
    let mut vc_b = "".get_vec_chars();
    vc_b.trim_end();
    assert!( vc_b.eq_str("") );
    
    let mut vc_c = "  ".get_vec_chars();
    vc_c.trim_end();
    // println!("{:?}", vc_c);
    assert!( vc_c.eq_str("") );
    drop(vc_a);
    drop(vc_b);
    drop(vc_c);


    // @@ Test 12 - trim()
    let mut vc_a = "  \tb  lll  aa a  \t".get_vec_chars();
    vc_a.trim();
    assert!(vc_a.eq_str("b  lll  aa a"));
    assert!( !(vc_a.eq_str("  \tb  lll  aa a  \t")) );

    let mut vc_b = "".get_vec_chars();
    vc_b.trim();
    assert!( vc_b.eq_str("") );
    
    let mut vc_c = "  ".get_vec_chars();
    vc_c.trim();
    // println!("{:?}", vc_c);
    assert!( vc_c.eq_str("") );
    drop(vc_a);
    drop(vc_b);
    drop(vc_c);


    // @@ Test 13 - find_vec()
    let vc_a = "blabliblu".get_vec_chars();
    let start_pos: usize = 0;
    let end_pos: Option<usize> = None;
    let res = vc_a.find_vec(&"bla".get_vec_chars(), start_pos, end_pos);
    assert!(res.is_some());
    if let Some(match_pos) = res {
        assert_eq!(match_pos, 0_usize);
    }
    drop(vc_a);


    // @@ Test 14 - find_str()
    // Note: This is where all the tests are made to the underlining find_vec() .
    let vc_a = "blabliblu".get_vec_chars();
    let start_pos: usize = 0;
    let end_pos: Option<usize> = None;
    let res = vc_a.find_str("bla", start_pos, end_pos);
    assert!(res.is_some());
    if let Some(match_pos) = res {
        assert_eq!(match_pos, 0_usize);
    }
    drop(vc_a);

    let vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.find_str("bli", 0, None);
    assert!(res.is_some());
    if let Some(match_pos) = res {
        assert_eq!(match_pos, 3_usize);
    }
    drop(vc_a);

    let vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.find_str("blu", 0, None);
    assert!(res.is_some());
    if let Some(match_pos) = res {
        assert_eq!(match_pos, 6_usize);
    }
    drop(vc_a);

    let vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.find_str("bli", 0, Some(5));
    assert!(res.is_some());
    if let Some(match_pos) = res {
        assert_eq!(match_pos, 3_usize);
    }
    drop(vc_a);

    let vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.find_str("blu", 0, Some(5));
    assert!(res.is_none());
    drop(vc_a);

    
    let vc_a = "blablu".get_vec_chars();
    let res = vc_a.find_str("bli", 0, None);
    assert!(res.is_none());
    drop(vc_a);


    let vc_a = "".get_vec_chars();
    let res = vc_a.find_str("bli", 0, None);
    assert!(res.is_none());
    drop(vc_a);


    // @@ Test 15 - contains_vec()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.contains_vec(&"bla".get_vec_chars());
    assert!(res_bool);
    let res_bool = vc_a.contains_vec(&"tu".get_vec_chars());
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 16 - contains_str()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.contains_str("bla");
    assert!(res_bool);
    let res_bool = vc_a.contains_str("tu");
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 17 - start_with_vec()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.start_with_vec(&"bla".get_vec_chars());
    assert!(res_bool);
    let res_bool = vc_a.start_with_vec(&"tu".get_vec_chars());
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 18 - start_with_str()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.start_with_str("bla");
    assert!(res_bool);
    let res_bool = vc_a.start_with_str("tu");
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 19 - ends_with_vec()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.ends_with_vec(&"blu".get_vec_chars());
    assert!(res_bool);
    let res_bool = vc_a.ends_with_vec(&"tu".get_vec_chars());
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 20 - ends_with_str()
    let vc_a = "blabliblu".get_vec_chars();
    let res_bool = vc_a.ends_with_str("blu");
    assert!(res_bool);
    let res_bool = vc_a.ends_with_str("tu");
    assert!(!res_bool);
    drop(vc_a);


    // @@ Test 21 - replace_vec()
    // Pattern that doesn't exist.
    let mut vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.replace_vec(&"BBB".get_vec_chars(),
                                            &"YYY".get_vec_chars(),
                                            0,
                                            None);
    assert!(res.is_none());
    assert!(vc_a.eq_str("blabliblu"));
    // Pattern that exists.
    let res = vc_a.replace_vec(&"bla".get_vec_chars(),
                                            &"BBBHHH".get_vec_chars(),
                                            0,
                                            None);
    if let Some(index) = res {
        assert_eq!(index, 0);
    }
    assert!(vc_a.eq_str("BBBHHHbliblu"));
    // Pattern that exists that substitutes by the empty string.
    let res = vc_a.replace_vec(&"HHH".get_vec_chars(),
                                            &"".get_vec_chars(),
                                            0,
                                            None);
    if let Some(index) = res {
        assert_eq!(index, 3);
    }
    assert!(vc_a.eq_str("BBBbliblu"));
    // Pattern that exists at the end that substitutes.
    let res = vc_a.replace_vec(&"blu".get_vec_chars(),
                                            &"".get_vec_chars(),
                                            0,
                                            None);
    if let Some(index) = res {
        assert_eq!(index, 6);
    }
    assert!(vc_a.eq_str("BBBbli"));
    // Pattern that exists for the all string and erases the string by substitution with nothing.
    let res = vc_a.replace_vec(&"BBBbli".get_vec_chars(),
                                            &"".get_vec_chars(),
                                            0,
                                            None);
    if let Some(index) = res {
        assert_eq!(index, 0);
    }
    assert!(vc_a.eq_str(""));
    drop(vc_a);


    // @@ Test 22 - replace_str()
    // Pattern that doesn't exist.
    let mut vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.replace_str("BBB",
                                            "YYY",
                                            0,
                                            None);
    assert!(res.is_none());
    assert!(vc_a.eq_str("blabliblu"));
    // Pattern that exists.
    let res = vc_a.replace_str("bla",
                                            "BBBHHH",
                                            0,
                                            None);
    if let Some(index) = res {
        assert_eq!(index, 0);
    }
    assert!(vc_a.eq_str("BBBHHHbliblu"));
    drop(vc_a);


    // @@ Test 23 - replace_vec_all()
    // Pattern that doesn't exist.
    let mut vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.replace_vec_all(&"BBB".get_vec_chars(),
                                                &"YYY".get_vec_chars());
    assert!(res.is_none());
    assert!(vc_a.eq_str("blabliblu")); 
    // Pattern that exists.
    let res = vc_a.replace_vec_all(&"bla".get_vec_chars(),
                                                &"BBBHHH".get_vec_chars());
    // println!("{:?}", vc_a);
    if let Some(index) = res {
        // println!("{:?}", index);
        // There were 1 replacements.
        assert_eq!(index, 1_usize);
    }
    assert!(vc_a.eq_str("BBBHHHbliblu"));
    drop(vc_a);
    // Pattern that exists in several places in the middle at the beginning and at the end, but not at the extremities.
    let mut vc_a = "abbbabbbabbba".get_vec_chars();
    let res = vc_a.replace_vec_all(&"bbb".get_vec_chars(),
                                                &"BBBB".get_vec_chars());
    // println!("{:?}", vc_a);
    if let Some(index) = res {
        // println!("{:?}", index);
        // There were 3 replacements.
        assert_eq!(index, 3);
    }
    assert!(vc_a.eq_str("aBBBBaBBBBaBBBBa"));
    drop(vc_a);
    // Pattern that exists in several places in the middle at the beginning and at the end, but not at the extremities.
    let mut vc_a = "AA_AA_AA_AA".get_vec_chars();
    let res = vc_a.replace_vec_all(&"AA".get_vec_chars(),
                                                &"BBB".get_vec_chars());
    // println!("{:?}", vc_a);
    if let Some(index) = res {
        // println!("{:?}", index);
        // There were 4 replacements.
        assert_eq!(index, 4);
    }
    assert!(vc_a.eq_str("BBB_BBB_BBB_BBB"));
    drop(vc_a);


    // @@ Test 24 - replace_str_all()
    // Pattern that doesn't exist.
    let mut vc_a = "blabliblu".get_vec_chars();
    let res = vc_a.replace_str_all("BBB",
                                   "YYY");
    assert!(res.is_none());
    assert!(vc_a.eq_str("blabliblu")); 
    // Pattern that exists.
    let res = vc_a.replace_str_all("bla",
                                                "BBBHHH");
    // println!("{:?}", vc_a);
    if let Some(index) = res {
        // println!("{:?}", index);
        // There were 1 replacements.
        assert_eq!(index, 1_usize);
    }
    assert!(vc_a.eq_str("BBBHHHbliblu"));
    drop(vc_a);


    // @@ Test 25 - split_vec()
    // At pattern that doesn't exist.
    let vc_a = "aBBlaBBliBBlu".get_vec_chars();
    let res_vec = vc_a.split_vec(&"CCC".get_vec_chars());
    assert_eq!(res_vec.len(), 0);
    
    // Pattern that exists, several in the midlle.
    let res_vec = vc_a.split_vec(&"BB".get_vec_chars());
    // println!("{:?}", res_vec);
    assert_eq!(res_vec.len(), 4);
    assert!(res_vec[0].to_vec_chars().eq_vec(&"a".get_vec_chars()));
    assert!(res_vec[1].to_vec_chars().eq_vec(&"la".get_vec_chars()));
    assert!(res_vec[2].to_vec_chars().eq_vec(&"li".get_vec_chars()));
    assert!(res_vec[3].to_vec_chars().eq_vec(&"lu".get_vec_chars()));
    drop(vc_a);

    // Pattern that exists several in the external points and in the middle.
    let vc_a = "aaaBBaaaCCaaaDDaaa".get_vec_chars();
    let res_vec = vc_a.split_vec(&"aaa".get_vec_chars());
    // println!("{:?}", res_vec);
    assert_eq!(res_vec.len(), 3);
    assert!(res_vec[0].to_vec_chars().eq_vec(&"BB".get_vec_chars()));
    assert!(res_vec[1].to_vec_chars().eq_vec(&"CC".get_vec_chars()));
    assert!(res_vec[2].to_vec_chars().eq_vec(&"DD".get_vec_chars()));
    drop(vc_a);


    // @@ Test 26 - split_str()
    // At pattern that doesn't exist.
    let vc_a = "aBBlaBBliBBlu".get_vec_chars();
    let res_vec = vc_a.split_str("CCC");
    assert_eq!(res_vec.len(), 0);
    
    // Pattern that exists, several in the midlle.
    let res_vec = vc_a.split_str("BB");
    // println!("{:?}", res_vec);
    assert_eq!(res_vec.len(), 4);
    assert!(res_vec[0].to_vec_chars().eq_vec(&"a".get_vec_chars()));
    assert!(res_vec[1].to_vec_chars().eq_vec(&"la".get_vec_chars()));
    assert!(res_vec[2].to_vec_chars().eq_vec(&"li".get_vec_chars()));
    assert!(res_vec[3].to_vec_chars().eq_vec(&"lu".get_vec_chars()));
    drop(vc_a);


    // @@ Test 27 - map_str()
    // At pattern that doesn't exist.
    let mut vc_a = "a1 a1 : a2 : a3 a3 a3 : a4 : a5".get_vec_chars();
    let replace_hashmap = HashMap::from([
        ("a1", "Cube"),
        ("a2", "Foo"),
        ("a3", "Bar"),
        ("a4", "33"),
        ("a5", "JJ"),
        ("BB", "This one doesn't exist!"),
    ]);
    let res_hashmap = vc_a.map_str(&replace_hashmap);
    
    // println!("{:?}", vc_a);
    assert_eq!(res_hashmap.len(), 6);
    assert_eq!(*res_hashmap.get("a1").unwrap(), 2);
    assert_eq!(*res_hashmap.get("a2").unwrap(), 1);
    assert_eq!(*res_hashmap.get("a3").unwrap(), 3);
    assert_eq!(*res_hashmap.get("a4").unwrap(), 1);
    assert_eq!(*res_hashmap.get("a5").unwrap(), 1);
    assert_eq!(*res_hashmap.get("BB").unwrap(), 0);
    assert!(vc_a.eq_str("Cube Cube : Foo : Bar Bar Bar : 33 : JJ"));
    drop(vc_a);

} // End of function test_vec_char_methods()