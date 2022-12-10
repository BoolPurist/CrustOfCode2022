use core::fmt::Display;
#[macro_export]
macro_rules! print_banner {
    ($c:tt, $n:tt) => {
        println!("{}", $c.repeat($n));
    };
    ($c:tt) => {
        println!("{}", $c.repeat(80));
    };
}
#[macro_export]
macro_rules! print_var {
    ($c:tt) => {
        println!("{}", $c);
    };
    ($l:tt, $c:tt) => {
        println!("{}: {}", $l, $c);
    };
}
pub fn print_grid<T>(grid: &Vec<Vec<T>>, sep: &str)
where
    T: Display,
{
    let mut max_len = 1;

    let buffer: Vec<Vec<String>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|column| {
                    let column_as_str = column.to_string();
                    max_len = max_len.max(column_as_str.len());
                    column_as_str
                })
                .collect()
        })
        .collect();

    let height = buffer.len();
    let width = buffer[0].len();
    let columns_indices: Vec<String> = (0..width)
        .map(|column| {
            let column_as_str = column.to_string();
            max_len = max_len.max(column_as_str.len());
            column_as_str
        })
        .collect();

    let mut write_to = String::with_capacity(max_len * height * width + (width * sep.len()));

    write_one_line(&mut write_to, max_len, &columns_indices, &sep);
    write_to.push('\n');

    let banner = "=".repeat(write_to.len() - 1);
    write_to.push_str(&banner);
    write_to.push('\n');

    for (index, line) in buffer.into_iter().enumerate() {
        write_one_line(&mut write_to, max_len, &line, &sep);

        let end_of_line = format!("+{}\n", index);
        write_to.push_str(&end_of_line);
    }

    println!("{write_to}");

    fn write_one_line(write_to: &mut String, max_len: usize, line: &Vec<String>, sep: &str) {
        write_to.push_str(sep);
        for column in line {
            let to_pad = max_len - column.len();
            write_to.push_str(column);
            if to_pad != 0 {
                let padding = " ".repeat(to_pad);
                write_to.push_str(&padding);
            }
            write_to.push_str(sep);
        }
    }
}
