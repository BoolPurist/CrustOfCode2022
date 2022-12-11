use crate::advent_math;
use crate::parsing;
use core::str::FromStr;
use std::collections::VecDeque;

type AmountUnit = usize;

#[derive(Debug)]
enum OpAmount {
    Old,
    Amount(AmountUnit),
}

#[derive(Debug)]
enum Operation {
    Add(OpAmount),
    Mult(OpAmount),
}
#[derive(Debug)]
struct OperationParseErr {
    _error_message: &'static str,
}

#[derive(Debug)]
struct ItemThrown {
    item: AmountUnit,
    monkey_thrown_to: AmountUnit,
}

#[derive(Debug)]
struct Test {
    divider: AmountUnit,
    thrown_if_true: AmountUnit,
    thrown_if_false: AmountUnit,
}

#[derive(Debug)]
struct Monkey {
    _id: AmountUnit,
    items: VecDeque<AmountUnit>,
    operation: Operation,
    test_logic: Test,
    number_inspection: AmountUnit,
    modular: Option<usize>,
}

pub fn calc_top_2_inspecting_number(input: &str) -> AmountUnit {
    let mut monkeys = parse_input(input);

    get_top_inspection_num_form(&mut monkeys, 2, 20)
}

// == After round 1 ==
// Monkey 0 inspected items 2 times.
// Monkey 1 inspected items 4 times.
// Monkey 2 inspected items 3 times.
// Monkey 3 inspected items 6 times.
pub fn calc_top_2_inspecting_number_no_relief(input: &str) -> AmountUnit {
    let mut monkeys = parse_input(input);
    let lcm = get_lcm_from(&monkeys);
    for to_set_modular in monkeys.iter_mut() {
        to_set_modular.set_common_modular(lcm);
    }

    get_top_inspection_num_form(&mut monkeys, 2, 10_000)
}

fn get_top_inspection_num_form(
    monkeys: &mut VecDeque<Monkey>,
    top_number: usize,
    round: usize,
) -> AmountUnit {
    go_nth_rounds(monkeys, round);
    let mut number_inspections: Vec<AmountUnit> = monkeys
        .into_iter()
        .map(|monkey| monkey.number_inspection)
        .collect();

    number_inspections.sort();
    number_inspections.reverse();

    number_inspections.into_iter().take(top_number).product()
}

fn get_lcm_from(monkeys: &VecDeque<Monkey>) -> AmountUnit {
    let mut modulars = monkeys.iter().map(|monkey| monkey.test_logic.divider);

    let mut lcm = modulars.next().unwrap();
    for current_modular in modulars {
        lcm = advent_math::get_lcm(lcm, current_modular);
    }

    lcm
}

fn go_nth_rounds(monkeys: &mut VecDeque<Monkey>, round_number: AmountUnit) {
    for _ in 0..round_number {
        for current_monkey in 0..monkeys.len() {
            let thrown_items = monkeys[current_monkey].inpect_next_round();
            for next_throw in thrown_items {
                monkeys[next_throw.monkey_thrown_to].give_thrown_item(next_throw.item);
            }
        }
    }
}

fn parse_input(input: &str) -> VecDeque<Monkey> {
    let sections = section_by_empty_line(input);

    sections
        .into_iter()
        .map(|current_section| {
            let mut lines = current_section.into_iter();

            let line_monkey_id = lines.next().expect("No lines for monkey id");
            let monkey_id: AmountUnit =
                parsing::get_seq_from_regex(r"Monkey (\d+):", &line_monkey_id, 1)
                    .expect("Could not parse out monkey id")[0]
                    .parse()
                    .expect("Could not parse monkey id to number");

            let line_starting_item = lines.next().expect("No line for starting items");
            let starting_items_comma_sep =
                parsing::strip_away_left_part(&line_starting_item, "Starting items: ");
            let items_to_start_with: Vec<AmountUnit> =
                parsing::get_parsed_sep_by(&starting_items_comma_sep, ", ");

            let raw_opration = lines.next().expect("No line for operation");
            let operation_stripped =
                parsing::strip_away_left_part(&raw_opration, "Operation: new = old ");

            let operations: Operation = operation_stripped
                .parse()
                .expect("Could not parse operation");

            let test_raw_divide = lines.next().expect("no line for test divide");
            let test_divide_stripped =
                parsing::strip_away_left_part(test_raw_divide, "Test: divisible by ");

            let test_amount_divide: AmountUnit = test_divide_stripped
                .parse()
                .expect("Test divide amount not parsable into number");

            let (thrown_true, thrown_false): (AmountUnit, AmountUnit) =
                match (lines.next(), lines.next()) {
                    (Some(true_line), Some(false_line)) => {
                        let line_true_stripped =
                            parsing::strip_away_left_part(&true_line, "If true: throw to monkey ");
                        let line_false_stripped = parsing::strip_away_left_part(
                            &false_line,
                            "If false: throw to monkey ",
                        );

                        (
                            line_true_stripped.parse().expect("Could parse into number"),
                            line_false_stripped
                                .parse()
                                .expect("Could parse into number"),
                        )
                    }
                    _ => panic!("No true and false line logic line"),
                };

            let monkey_test_logic = Test::new(test_amount_divide, thrown_true, thrown_false);

            Monkey::new(
                monkey_id,
                items_to_start_with.into_iter().collect(),
                operations,
                monkey_test_logic,
            )
        })
        .collect()
}

fn section_by_empty_line(to_section: &str) -> Vec<Vec<&str>> {
    let mut current_monkey = 0;
    let mut sections: Vec<Vec<&str>> = vec![vec![]];

    for line in to_section.lines() {
        if line.is_empty() {
            sections.push(vec![]);
            current_monkey += 1;
        } else {
            sections[current_monkey].push(line);
        }
    }

    sections
}

impl Test {
    pub fn new(
        divider: AmountUnit,
        thrown_if_true: AmountUnit,
        thrown_if_false: AmountUnit,
    ) -> Self {
        Self {
            divider,
            thrown_if_true,
            thrown_if_false,
        }
    }

    pub fn return_right_value(&self, amount: AmountUnit) -> AmountUnit {
        if (amount % self.divider) == 0 {
            self.thrown_if_true
        } else {
            self.thrown_if_false
        }
    }
}

impl FromStr for Operation {
    type Err = OperationParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");

        match (splits.next(), splits.next()) {
            (Some(operator), Some(amount)) => {
                let mut amount_value = OpAmount::Old;
                if amount != "old" {
                    let amount_parsed: AmountUnit =
                        amount.parse().map_err(|_| OperationParseErr {
                            _error_message: "Amount for operation is parseable to number",
                        })?;

                    amount_value = OpAmount::Amount(amount_parsed);
                }

                match operator {
                    "+" => Ok(Operation::Add(amount_value)),
                    "*" => Ok(Operation::Mult(amount_value)),
                    _ => Err(OperationParseErr {
                        _error_message: "Unkown operator",
                    }),
                }
            }
            _ => Err(OperationParseErr {
                _error_message: "No two parst after splitting by whitespace",
            }),
        }
    }
}

impl Operation {
    fn apply_to(&self, item: AmountUnit) -> AmountUnit {
        return match &self {
            Operation::Add(amount) => item + Self::get_amount(item, amount),
            Operation::Mult(amount) => item * Self::get_amount(item, amount),
        };
    }

    fn apply_with_mod(&self, item: AmountUnit, modular: usize) -> AmountUnit {
        return match &self {
            Operation::Add(amount) => {
                let (mod_item, mod_amount) = get_modular_left_right(item, &amount, modular);
                (mod_item + mod_amount) % modular
            }
            Operation::Mult(amount) => {
                let (mod_item, mod_amount) = get_modular_left_right(item, &amount, modular);
                (mod_item * mod_amount) % modular
            }
        };

        fn get_modular_left_right(
            item: usize,
            amount: &OpAmount,
            modular: usize,
        ) -> (usize, usize) {
            let mod_item = item % modular;
            let amount = Operation::get_amount(item, amount);
            let mod_amount = amount % modular;

            (mod_item, mod_amount)
        }
    }

    fn get_amount(item: AmountUnit, amount: &OpAmount) -> AmountUnit {
        match amount {
            OpAmount::Old => item,
            OpAmount::Amount(fixed_amount) => *fixed_amount,
        }
    }
}

impl Monkey {
    fn new(
        id: AmountUnit,
        items: VecDeque<AmountUnit>,
        operation: Operation,
        test_logic: Test,
    ) -> Self {
        Self {
            _id: id,
            items,
            operation,
            test_logic,
            number_inspection: 0,
            modular: None,
        }
    }

    fn set_common_modular(&mut self, modular: usize) {
        self.modular = Some(modular)
    }

    fn give_thrown_item(&mut self, new_item: AmountUnit) {
        self.items.push_back(new_item);
    }

    fn inpect_next_round(&mut self) -> VecDeque<ItemThrown> {
        let to_return = self
            .items
            .iter()
            .map(|next_item| {
                let worried_level = match self.modular {
                    None => {
                        let from_operation = self.operation.apply_to(*next_item);

                        from_operation / 3
                    }
                    Some(m) => {
                        let from_operation = self.operation.apply_with_mod(*next_item, m);

                        from_operation
                    }
                };

                let to_throw_to = self.test_logic.return_right_value(worried_level);

                ItemThrown {
                    item: worried_level,
                    monkey_thrown_to: to_throw_to,
                }
            })
            .collect();

        self.number_inspection += self.items.len();
        self.items.clear();

        to_return
    }
}
