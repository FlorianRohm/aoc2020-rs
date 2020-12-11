use ndarray::{Array, Array2};

use crate::grid::Cell;

pub fn parse(input: &str) -> Array2<Cell> {
    let lines: Vec<Vec<char>> = input.lines().into_iter().filter(|line| !line.is_empty()).map(|line| line.chars().into_iter().collect()).collect();
    let l_x = lines.len();
    let l_y = lines[0].len();

    Array::from_shape_fn((l_x + 2, l_y + 2), |(x, y)| {
        match (x, y) {
            (x, y) if x == 0 || y == 0 || x == l_x + 1 || y == l_y + 1 => Cell::FLR,
            (x, y) => match lines[x - 1][y - 1] {
                '.' => Cell::FLR,
                'L' => Cell::FRE,
                '#' => Cell::OCC,
                _ => unreachable!()
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test() {
        let input = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

        let grid = ndarray::array![
            [Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FLR, Cell::FLR, Cell::FRE, Cell::FLR, Cell::FLR, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FLR, Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FRE, Cell::FLR, Cell::FRE, Cell::FRE, Cell::FLR],
            [Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR, Cell::FLR]
        ];
        let base = parse(input);


        assert_eq!(base, grid)
    }
}