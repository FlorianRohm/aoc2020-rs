use ndarray::{arr3, Array, Array4, s};

use crate::cell::Cell;

pub fn parse(input: &str, border: usize) -> Array4<Cell> {
    enlarge(smallest_possible(input), border)
}

pub fn enlarge(input: Array4<Cell>, border: usize) -> Array4<Cell> {
    let (d1, d2, d3, d4) = input.dim();

    let mut raw = Array4::from_elem((d1 + 2 * border, d2 + 2 * border, d3 + 2 * border, d4 + 2 * border), Cell::Inactive);
    let mut slice = raw.slice_mut(s![border..border+d1,border..border+d2,border..border+d3, border..border+d4]);

    slice.assign(&input);

    raw
}

pub fn smallest_possible(input: &str) -> Array4<Cell> {
    let lines: Vec<Vec<Cell>> = input.lines().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().into_iter()
            .map(|c| c.to_string().parse::<Cell>().unwrap())
            .collect())
        .collect();
    let l_y = lines.len();
    let l_z = lines[0].len();


    Array::from_shape_fn((1, 1, l_y, l_z), move |(_, _, y, z)| { lines[y][z] })
}
