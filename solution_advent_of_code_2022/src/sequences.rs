use std::iter::repeat;

pub fn create_grid_with_value<T>(height: usize, width: usize, cell_value: &T) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut output: Vec<Vec<T>> = Default::default();

    for _ in 0..height {
        let row = repeat(cell_value.clone()).take(width).collect::<Vec<T>>();
        output.push(row);
    }

    output
}

pub fn create_grid_with_default<T>(height: usize, width: usize) -> Vec<Vec<T>>
where
    T: Clone + Default,
{
    create_grid_with_value(height, width, &Default::default())
}

pub fn grid_traverse_up(y: usize) -> impl Iterator<Item = usize> {
    (0..y).rev()
}
pub fn grid_traverse_down(y: usize, height: usize) -> impl Iterator<Item = usize> {
    (y + 1)..height
}
pub fn grid_traverse_left(x: usize) -> impl Iterator<Item = usize> {
    (0..x).rev()
}
pub fn grid_traverse_right(x: usize, width: usize) -> impl Iterator<Item = usize> {
    (x + 1)..width
}
