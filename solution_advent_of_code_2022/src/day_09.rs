use std::collections::HashSet;
use std::iter::repeat;
type HeadSteps = Vec<HeadMovement>;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
struct Coord(i32, i32);
#[derive(Default, Debug, Clone)]
struct Tail(Coord);
#[derive(Default, Debug)]
struct Head(Coord);

#[derive(Debug)]
enum HeadMovement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Default, Debug)]
struct SimulationResult {
    tail: Tail,
    head: Head,
}

pub fn get_tail_vists_number(input: &str) -> usize {
    let head_tails = parse_input(input);
    let (_, tail_moved_number) = simulation_head_tail(&head_tails);

    tail_moved_number
}
pub fn get_visted_number_of_last_tail(input: &str, number: usize) -> usize {
    let parsed = parse_input(input);

    get_nth_number_visited(&parsed, number)
}

fn get_nth_number_visited(input: &HeadSteps, number: usize) -> usize {
    let mut tails: Vec<Tail> = repeat(Tail::default()).take(number).collect();
    let mut head: Head = Default::default();

    let mut found: HashSet<Coord> = Default::default();
    found.insert(Coord(0, 0));

    let mut visted_by_last_tail = 1;
    for next_direction in input {
        match next_direction {
            HeadMovement::Up(steps) => {
                for _ in 0..*steps {
                    head.0.go_top();
                    visted_by_last_tail += return_if_last_resolved(&head, &mut tails, &mut found);
                }
            }
            HeadMovement::Down(steps) => {
                for _ in 0..*steps {
                    head.0.go_bottom();
                    visted_by_last_tail += return_if_last_resolved(&head, &mut tails, &mut found);
                }
            }
            HeadMovement::Left(steps) => {
                for _ in 0..*steps {
                    head.0.go_left();
                    visted_by_last_tail += return_if_last_resolved(&head, &mut tails, &mut found);
                }
            }
            HeadMovement::Right(steps) => {
                for _ in 0..*steps {
                    head.0.go_right();
                    visted_by_last_tail += return_if_last_resolved(&head, &mut tails, &mut found);
                }
            }
        }
    }

    return visted_by_last_tail;

    fn return_if_last_resolved(
        head: &Head,
        tails: &mut Vec<Tail>,
        found: &mut HashSet<Coord>,
    ) -> usize {
        if let Some(crood) = resolve_movement_head(head, tails) {
            if found.insert(crood.clone()) {
                return 1;
            } else {
                return 0;
            }
        }

        0
    }

    fn resolve_movement_head(head: &Head, tails: &mut Vec<Tail>) -> Option<Coord> {
        let mut current_element = head.0.clone();
        let mut last_resolved = None;

        for next_tail in tails.iter_mut() {
            last_resolved = resolve_if_needed(&mut next_tail.0, &current_element);

            current_element = next_tail.0.clone();
        }

        // current_element should be last element as tail now.

        last_resolved
    }
}

fn simulation_head_tail(input: &HeadSteps) -> (SimulationResult, usize) {
    let mut simulation: SimulationResult = Default::default();
    let mut moved_fiels: usize = 0;

    let mut already_visited: HashSet<Coord> = Default::default();
    already_visited.insert(simulation.tail.0.clone());
    moved_fiels += 1;

    for next_direction in input {
        match next_direction {
            HeadMovement::Up(steps) => {
                for _ in 0..*steps {
                    simulation.head.0.go_top();
                    moved_fiels += return_inc_if_resolved(&mut simulation, &mut already_visited);
                }
            }
            HeadMovement::Down(steps) => {
                for _ in 0..*steps {
                    simulation.head.0.go_bottom();
                    moved_fiels += return_inc_if_resolved(&mut simulation, &mut already_visited);
                }
            }
            HeadMovement::Left(steps) => {
                for _ in 0..*steps {
                    simulation.head.0.go_left();
                    moved_fiels += return_inc_if_resolved(&mut simulation, &mut already_visited);
                }
            }
            HeadMovement::Right(steps) => {
                for _ in 0..*steps {
                    simulation.head.0.go_right();
                    moved_fiels += return_inc_if_resolved(&mut simulation, &mut already_visited);
                }
            }
        }

        fn return_inc_if_resolved(sim: &mut SimulationResult, found: &mut HashSet<Coord>) -> usize {
            match resolve_if_needed(&mut sim.tail.0, &mut sim.head.0) {
                Some(coord) => {
                    if found.insert(coord) {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            }
        }
    }

    return (simulation, moved_fiels);
}

fn resolve_if_needed(tail: &mut Coord, head: &Coord) -> Option<Coord> {
    if let Some(to_resolve) = tail.get_distances_if_needed(&head) {
        let (x_dist, y_dist) = to_resolve;
        let x_dist_abs = x_dist.abs();
        let y_dist_abs = y_dist.abs();

        if x_dist_abs == y_dist_abs {
            let x_signum = x_dist.signum();
            let y_signum = y_dist.signum();
            tail.1 = head.1 + y_signum;
            tail.0 = head.0 + x_signum;
        } else if x_dist_abs < y_dist_abs {
            tail.0 = head.0;
            let signum = y_dist.signum();
            tail.1 = head.1 + signum;
        } else {
            tail.1 = head.1;
            let signum = x_dist.signum();
            tail.0 = head.0 + signum;
        }

        return Some(tail.clone());
    }

    None
}

fn parse_input(input: &str) -> HeadSteps {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(" ");

            match (splits.next(), splits.next()) {
                (Some(direction), Some(steps)) => {
                    let steps_parsed: u32 = steps
                        .parse()
                        .expect("Right part from whitespace is not valid unsigned number");

                    match direction {
                        "U" => HeadMovement::Up(steps_parsed),
                        "D" => HeadMovement::Down(steps_parsed),
                        "L" => HeadMovement::Left(steps_parsed),
                        "R" => HeadMovement::Right(steps_parsed),
                        _ => panic!("{direction} is not parseable for a direction"),
                    }
                }
                _ => panic!("Line being parsed has no left and right part between space"),
            }
        })
        .collect()
}
impl Coord {
    fn go_top(&mut self) {
        self.1 += 1
    }
    fn go_bottom(&mut self) {
        self.1 -= 1
    }
    fn go_left(&mut self) {
        self.0 -= 1
    }
    fn go_right(&mut self) {
        self.0 += 1
    }
    fn get_distances_if_needed(&self, other: &Self) -> Option<(i32, i32)> {
        let x_dist = self.0 - other.0;
        let y_dist = self.1 - other.1;
        let x_total_dist = x_dist.abs();
        let y_total_dist = y_dist.abs();

        if x_total_dist > 1 || y_total_dist > 1 {
            Some((x_dist, y_dist))
        } else {
            None
        }
    }
}
