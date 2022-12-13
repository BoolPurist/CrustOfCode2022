use crate::parsing;
use std::collections::VecDeque;

type ListData = Vec<Box<Packet>>;

#[derive(Debug)]
enum Packet {
    Integer(u32),
    List(ListData),
}

#[derive(Debug)]
struct PairPacket {
    left: Box<Packet>,
    right: Box<Packet>,
}
#[allow(unused_macros)]
macro_rules! packet_int {
    [$n:expr] => {{
        Box::new(Packet::Integer($n))
    }};
    [] => {{
        Box::new(vec![])
    }};
}

#[allow(unused_macros)]
macro_rules! packet_lst {
    ($($n:expr),*) => {{
        let mut _vec: Vec<Box<Packet>> = Vec::new();
        $(
            _vec.push($n);
        )*

        Box::new(Packet::List(_vec))
    }};
}
pub fn get_sum_indicies(input: &str) -> usize {
    let packets = parse_input(input);

    packets
        .into_iter()
        .enumerate()
        .filter_map(|index_packet_to_inspect| {
            let (index, to_inspect) = index_packet_to_inspect;
            let order = index + 1;
            let is_in_right_order = compare_pair_queies(to_inspect);

            if is_in_right_order {
                dbg!(order);
                Some(order)
            } else {
                None
            }
        })
        .sum()
}

fn compare_pair_queies(pair: PairPacket) -> bool {
    let mut pending_values_left: VecDeque<Box<Packet>> = Default::default();
    let mut pending_values_right: VecDeque<Box<Packet>> = Default::default();

    pending_values_left.push_front(pair.left);
    pending_values_right.push_back(pair.right);

    loop {
        match (
            pending_values_left.pop_front(),
            pending_values_right.pop_front(),
        ) {
            (Some(next_left), Some(next_right)) => {
                match (*next_left, *next_right) {
                    (Packet::Integer(left_number), Packet::Integer(right_number)) => {
                        if left_number < right_number {
                            return true;
                        } else if left_number > right_number {
                            return false;
                        }
                    }
                    (Packet::List(list), Packet::Integer(number)) => {
                        pending_values_right.push_front(Box::new(Packet::Integer(number)));
                        for to_push_front in list.into_iter().rev() {
                            pending_values_left.push_front(to_push_front);
                        }
                    }
                    (Packet::Integer(number), Packet::List(list)) => {
                        pending_values_left.push_front(Box::new(Packet::Integer(number)));
                        for to_push_front in list.into_iter().rev() {
                            pending_values_right.push_front(to_push_front);
                        }
                    }
                    (Packet::List(left_list), Packet::List(right_list)) => {
                        for to_push_front in right_list.into_iter().rev() {
                            pending_values_right.push_front(to_push_front);
                        }
                        for to_push_front in left_list.into_iter().rev() {
                            pending_values_left.push_front(to_push_front);
                        }
                    }
                };

                continue;
            }
            (Some(_), None) => return false,
            (None, Some(_)) => return true,
            (None, None) => return true,
        };
    }
}

fn parse_input(input: &str) -> Vec<PairPacket> {
    let chunks = parsing::split_chunks_where(input, |line| line.is_empty());

    return chunks
        .into_iter()
        .enumerate()
        .map(
            |left_right| match (left_right.1.get(0), left_right.1.get(1)) {
                (Some(&left), Some(&right)) => {
                    let left_parsed = return_nested_packet(&left);
                    let right_parsed = return_nested_packet(&right);
                    PairPacket::new(left_parsed, right_parsed)
                }
                _ => panic!("No 2 lines for the left and right part of the pair packet."),
            },
        )
        .collect();

    fn return_nested_packet(line: &str) -> Box<Packet> {
        return traverse(&line.chars().collect(), 0).0;
        fn traverse(line: &Vec<char>, mut index: usize) -> (Box<Packet>, usize) {
            let mut current_char = line[index];

            match current_char {
                '[' => {
                    let mut list: ListData = Vec::new();
                    index += 1;
                    loop {
                        let (item, last_index) = traverse(line, index);
                        let new_index = last_index + 1;
                        list.push(item);
                        current_char = line[last_index];

                        match current_char {
                            ']' => return (Box::new(Packet::List(list)), new_index),
                            ',' => {
                                index = new_index;
                                continue;
                            }
                            not_expected_char => panic!(
                                "Char {} is not inspected at index {}",
                                not_expected_char, index
                            ),
                        }
                    }
                }
                ']' => return (Box::new(Packet::List(Vec::new())), index),
                _digit => {
                    let mut number_str = String::new();

                    while current_char.is_numeric() {
                        number_str.push(current_char);
                        index += 1;
                        current_char = line[index];
                    }
                    let number = number_str.parse().unwrap();

                    return (Box::new(Packet::Integer(number)), index);
                }
            }
        }
    }
}

impl PairPacket {
    fn new(left: Box<Packet>, right: Box<Packet>) -> Self {
        Self { left, right }
    }
}
