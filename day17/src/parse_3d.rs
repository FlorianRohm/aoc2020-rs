use ndarray::{arr3, Array, Array3, s};

use crate::cell::Cell;

pub fn parse(input: &str, border: usize) -> Array3<Cell> {
    enlarge(smallest_possible(input), border)
}

pub fn enlarge(input: Array3<Cell>, border: usize) -> Array3<Cell> {
    let (x, y, z) = input.dim();

    let mut raw = Array3::from_elem((x + 2 * border, y + 2 * border, z + 2 * border), Cell::Inactive);
    let mut slice = raw.slice_mut(s![border..border+x,border..border+y,border..border+z]);

    slice.assign(&input);

    raw
}

pub fn smallest_possible(input: &str) -> Array3<Cell> {
    let lines: Vec<Vec<Cell>> = input.lines().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().into_iter()
            .map(|c| c.to_string().parse::<Cell>().unwrap())
            .collect())
        .collect();
    let l_y = lines.len();
    let l_z = lines[0].len();


    Array::from_shape_fn((1, l_y, l_z), move |(_, y, z)| { lines[y][z] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test_3() {
        use Cell::*;
        let input = "
..#
###
";

        let grid = ndarray::array![[
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        ],[
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        ],[
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Active,  Inactive,Inactive ],
        [Inactive, Inactive, Active,   Active,   Active,  Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ]
       ],[
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        ],[
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive, Inactive,Inactive,Inactive ],
        ]];

        let base = parse(input, 2);


        assert_eq!(base, grid)
    }

    #[test]
    fn should_solve_test() {
        use Cell::*;
        let input = "
..#
###
";

        let grid = ndarray::array![[
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        ],[
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Active,  Inactive ],
        [Inactive, Active,   Active,   Active,  Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ]
       ],[
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        [Inactive, Inactive, Inactive, Inactive,Inactive ],
        ]];

        let base = parse(input, 1);


        assert_eq!(base, grid)
    }

    #[test]
    fn should_solve_test_2() {
        use Cell::*;
        let input = "
.#.
..#
###
";

        let grid = ndarray::array![[
        [Inactive, Active,   Inactive ],
        [Inactive, Inactive, Active ],
        [Active,   Active,   Active ]
       ]];

        let base = smallest_possible(input);


        assert_eq!(base, grid)
    }
}

