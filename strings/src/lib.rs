use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[cfg(test)]
mod tests;
mod utils;

// This excersises only work for 1 byte UTF-8 encoded characters

// Check if string has unique characters
pub fn has_unique_chars(input: &str) -> bool {
    let mut map = HashMap::new();

    for byte in input.bytes() {
        if !utils::is_latin(byte) {
            panic!("Not a latin word!");
        }

        let not_unique = match map.entry(byte) {
            Entry::Occupied(_o) => true,
            Entry::Vacant(_v) => {
                map.insert(byte, byte);
                false
            }
        };

        if not_unique {
            return false;
        }
    }
    true
}

// Check if one string is a permutation of another string
pub fn is_permutation(first: &str, second: &str) -> bool {
    let mut first_map: HashMap<u8, i32> = HashMap::new();
    let mut second_map: HashMap<u8, i32> = HashMap::new();
    let mut curr_map = &mut first_map;

    for (i, byte) in first.bytes().chain(second.bytes()).enumerate() {
        if !utils::is_latin(byte) {
            panic!("Not a latin word!");
        }
        if i == first.len() {
            curr_map = &mut second_map;
        }

        let counter = curr_map.entry(byte).or_insert(0);
        *counter += 1;
    }

    if first_map.len() != second_map.len() {
        return false;
    }
    first_map.iter().all(|(key, value)| {
        match second_map.entry(*key) {
            Entry::Occupied(v) => v.get() == value,
            Entry::Vacant(_o) => false
        }
    })
}

pub fn urlify(source: &str) -> String {
    let mut out = String::new();
    for word in source.split(' ') {
        out.push_str(&word);
        out.push_str("%20")
    }
    out.pop();
    out.pop();
    out.pop();
    out
}

pub fn encode_base64(source: &str) -> String {
    let mut out = String::new();
    let base64 = utils::get_base64_table();
    let byte_buffer : Vec<u8> = source.bytes().collect();
    let mut iter = byte_buffer.iter();

    loop {
        let mut code_buffer : i32 = 0;
        let mut mask : i32 = 0;
        for i in (0..3).rev() {
            match iter.next() {
                Some(byte) => code_buffer |= (*byte as i32).to_le() << 8*i,
                None => mask |= (0xFF) << 8*i
            }
        }

        let mut exit = false;
        for i in (0..4).rev() {
            if (mask >> 6*i) & 0x3F == 0x3F {
                out.push('=');
                exit = true;
            }
            else {
                let code = (code_buffer >> 6*i) & 0x3F;
                out.push(base64[code as usize]);
            }
        }
        if exit {break;}
    }

    out
}