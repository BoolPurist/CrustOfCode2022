use core::fmt::Debug;
use core::iter::zip;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type NumberUnit = usize;
type OpenList = VecDeque<(Coord, NumberUnit)>;
type PathMap = HashMap<Coord, CoordAndCost>;
type HeightMap = Vec<Vec<NumberUnit>>;

#[derive(Default, Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coord(NumberUnit, NumberUnit);

const MAX_HEIGHT: NumberUnit = 26;
const MIN_HEIGHT: NumberUnit = 1;

impl Coord {
    fn get_cell_from(&self, grid: &HeightMap) -> NumberUnit {
        grid[self.1][self.0]
    }
}

enum GridCell {
    S,
    E,
    Cell(NumberUnit),
}

#[derive(Debug)]
struct CoordAndCost {
    origins: Vec<Coord>,
    cost: NumberUnit,
}

struct GridStartEnd {
    grid: HeightMap,
    start: Coord,
    end: Coord,
}

impl Debug for GridStartEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(GridStartEnd))
            .field("S", &self.start)
            .field("E", &self.end)
            .field("Grid: {}", &self.grid)
            .finish()
    }
}

pub fn calc_path_with_fewest_steps(input: &str) -> usize {
    let mountains = parse_input(input);
    let paths = build_path_map(&mountains, |adjacant, current| adjacant > (current + 1));

    paths
        .get(&mountains.end)
        .expect("No path to end point")
        .cost
}

pub fn calc_path_from_any_a_fewest_steps(input: &str) -> ((usize, usize), usize) {
    let mut mountains = parse_input(input);

    let new_start = mountains.end;
    mountains.end = mountains.start;
    mountains.start = new_start;

    let paths = build_path_map(&mountains, |adjacant, current| {
        let too_low = current - 1;

        adjacant < too_low
    });

    let optimal_a = zip(paths.keys(), paths.values())
        .filter(|&cell| cell.0.get_cell_from(&mountains.grid) == 1)
        .min_by_key(|cell| cell.1.cost)
        .unwrap();

    ((optimal_a.0 .0, optimal_a.0 .1), optimal_a.1.cost)
}

fn build_path_map<P>(system: &GridStartEnd, skip_predicate: P) -> PathMap
where
    P: Fn(NumberUnit, NumberUnit) -> bool,
{
    let mut open: OpenList = Default::default();
    let grid = &system.grid;
    let mut closed: HashSet<Coord> = Default::default();
    open.push_back((system.start, 0));
    closed.insert(system.start);

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;

    let mut path_map: PathMap = Default::default();
    _ = path_map.insert(
        system.start,
        CoordAndCost {
            cost: 0,
            origins: Vec::new(),
        },
    );

    while let Some(next_coord) = open.pop_front() {
        let adjacant = get_adjacant_coord(next_coord.0, max_y, max_x);

        let currend_coord = next_coord.0;
        let current_height = currend_coord.get_cell_from(&grid);

        let current_node = next_coord.0;
        let current_cost = next_coord.1 + 1;

        for current_adjacant in adjacant.iter() {
            let adjacant_height = current_adjacant.get_cell_from(&grid);

            if skip_predicate(adjacant_height, current_height) {
                continue;
            }

            if closed.insert(*current_adjacant) {
                open.push_back((*current_adjacant, current_cost));
            }

            match path_map.get_mut(&current_adjacant) {
                Some(found) => {
                    if found.cost > current_cost {
                        found.cost = current_cost;
                        found.origins = vec![current_node];
                    } else if found.cost == current_cost {
                        found.origins.push(current_node);
                    }
                }
                None => {
                    _ = path_map.insert(
                        *current_adjacant,
                        CoordAndCost {
                            cost: current_cost,
                            origins: vec![current_node],
                        },
                    )
                }
            };
        }
    }

    path_map
}

fn get_adjacant_coord(coord: Coord, max_y: NumberUnit, max_x: NumberUnit) -> VecDeque<Coord> {
    let mut adjacant: VecDeque<Coord> = Default::default();

    let y = coord.1;

    let x = coord.0;

    if y != 0 {
        adjacant.push_back(Coord(x, y - 1));
    }
    if y < max_y {
        adjacant.push_back(Coord(x, y + 1));
    }
    if x != 0 {
        adjacant.push_back(Coord(x - 1, y));
    }
    if x < max_x {
        adjacant.push_back(Coord(x + 1, y));
    }

    adjacant
}

fn parse_input(input: &str) -> GridStartEnd {
    let mut target = Coord::default();
    let mut start = Coord::default();
    let mut lowest_elevation: Vec<Coord> = Default::default();

    let grid = input
        .lines()
        .enumerate()
        .map(|y_line| {
            let (y, line) = y_line;

            line.chars()
                .enumerate()
                .map(|x_char| {
                    let (x, char) = x_char;

                    let coord = Coord(x, y);

                    match map_char_to_num(char) {
                        GridCell::S => {
                            lowest_elevation.push(coord);
                            start = coord;

                            MIN_HEIGHT
                        }
                        GridCell::E => {
                            target = coord;

                            MAX_HEIGHT
                        }
                        GridCell::Cell(cell_v) => {
                            if cell_v == MIN_HEIGHT {
                                lowest_elevation.push(coord)
                            }
                            cell_v
                        }
                    }
                })
                .collect()
        })
        .collect();

    GridStartEnd {
        grid,
        start,
        end: target,
    }
}

fn map_char_to_num(to_map: char) -> GridCell {
    match to_map {
        'S' => GridCell::S,
        'E' => GridCell::E,
        lower_case => {
            let num_offset = (lower_case as u32) - ('a' as u32) + 1;

            GridCell::Cell(num_offset as usize)
        }
    }
}
