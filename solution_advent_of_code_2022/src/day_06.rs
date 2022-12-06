use std::collections::HashSet;
pub fn get_end_of_first_packet_start(input: &str, size_marker: usize) -> u32 {
    let chars: Vec<char> = input.chars().collect();
    let limit = chars.len() - (size_marker - 1);
    let mut symb_cache: HashSet<char> = Default::default();
    for (index, current_symb) in (&chars[0..limit]).iter().enumerate() {
        symb_cache.clear();
        symb_cache.insert(*current_symb);
        let sub_limit = index + size_marker;
        let mut found_no_dup = true;
        for next in (&chars[(index + 1)..(sub_limit)]).iter() {
            if !symb_cache.insert(*next) {
                found_no_dup = false;
                break;
            }
        }

        if found_no_dup {
            return sub_limit as u32;
        }
    }

    panic!("No start for start packet found");
}
