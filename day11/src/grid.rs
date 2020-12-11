use std::fmt::{Debug, Formatter};

use ndarray::{Array2, ArrayBase, ArrayView, ArrayView2, OwnedRepr, Zip};
use ndarray::prelude::s;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    FLR,
    OCC,
    FRE,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::FLR => write!(f, "."),
            Cell::OCC => write!(f, "#"),
            Cell::FRE => write!(f, "L")
        }
    }
}

impl Cell {
    pub fn to_occupation_nr(&self) -> usize {
        if self == &Self::OCC { 1 } else { 0 }
    }
}

#[derive(PartialEq, Debug)]
pub struct Simulation {
    pub l_x: usize,
    pub l_y: usize,
    pub lattice: Array2<Cell>,
    pub tmp_lattice: Array2<Cell>,
}

macro_rules! inner_slice {
    ( $x:expr ) => {
        s!(1..=$x.l_x, 1..=$x.l_y)
    };
}
impl Simulation {
    fn propagate_pull_inner(mut self) -> Self {
        let source = self.lattice.windows((3, 3));
        let mut target = self.tmp_lattice.slice_mut(inner_slice!(self));
        Zip::from(&mut target).and(source).apply(|tar, src| {
            let occupations = src[[0, 0]].to_occupation_nr() +
                src[[1, 0]].to_occupation_nr() +
                src[[2, 0]].to_occupation_nr() +
                src[[0, 1]].to_occupation_nr() +
                src[[2, 1]].to_occupation_nr() +
                src[[0, 2]].to_occupation_nr() +
                src[[1, 2]].to_occupation_nr() +
                src[[2, 2]].to_occupation_nr();

            let new_cell = match (&src[[1, 1]], occupations) {
                (Cell::OCC, o) if o >= 4 => Cell::FRE,
                (Cell::FRE, o) if o == 0 => Cell::OCC,
                (c, _) => c.clone()
            };
            *tar = new_cell;
        });

        std::mem::swap(&mut self.tmp_lattice, &mut self.lattice);

        self
    }

    fn propagate_pull_wide(mut self) -> Self {
        let source = self.lattice.slice(inner_slice!(self));
        let mut target = self.tmp_lattice.slice_mut(inner_slice!(self));

        target.indexed_iter_mut().for_each(|((x, y), tar)| {
            let occupations =
                get_seat_in_direction(source, x, y, (-1, 1)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (0, 1)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (1, 1)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (-1, 0)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (1, 0)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (-1, -1)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (0, -1)).map(|c| c.to_occupation_nr()).unwrap_or_default() +
                    get_seat_in_direction(source, x, y, (1, -1)).map(|c| c.to_occupation_nr()).unwrap_or_default();

            let new_cell = match (&source[[x, y]], occupations) {
                (Cell::OCC, o) if o >= 5 => Cell::FRE,
                (Cell::FRE, o) if o == 0 => Cell::OCC,
                (c, _) => c.clone()
            };
            *tar = new_cell;
        });

        std::mem::swap(&mut self.tmp_lattice, &mut self.lattice);

        self
    }

    pub fn propagate_until_hold(self) -> Self {
        let new = self.propagate_pull_inner();
        if new.lattice != new.tmp_lattice {
            new.propagate_pull_inner().propagate_until_hold()
        } else { new }
    }

    pub fn propagate_wide_hold(self) -> Self {
        let new = self.propagate_pull_wide();
        if new.lattice != new.tmp_lattice {
            new.propagate_pull_wide().propagate_wide_hold()
        } else { new }
    }

    pub fn count_seats(&self) -> usize {
        self.lattice.iter().map(|cell| cell.to_occupation_nr()).sum()
    }

    pub(crate) fn new(grid: Array2<Cell>) -> Self {
        use ndarray::Array;
        use ndarray::Ix2;

        let (x, y) = grid.dim();

        let mut tmp_lattice = unsafe { Array::<Cell, Ix2>::uninitialized((x, y)) };
        tmp_lattice.assign(&grid);
        Simulation { lattice: grid, tmp_lattice, l_x: x - 2, l_y: y - 2 }
    }
}

fn get_seat_in_direction(source: ArrayView2<Cell>, x: usize, y: usize, direction: (i32, i32)) -> Option<Cell> {
    if direction.0 > x as i32 || direction.1 > y as i32 { return None; }
    let new_x = (x as i32 - direction.0) as usize;
    let new_y = (y as i32 - direction.1) as usize;
    match source.get((new_x, new_y)) {
        None => None,
        Some(c) => match c {
            Cell::FLR => get_seat_in_direction(source, new_x, new_y, direction),
            Cell::OCC | Cell::FRE => Some(c.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test() {
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
        let simulation = Simulation::new(grid);

        let simulation = simulation.propagate_until_hold();

        assert_eq!(simulation.count_seats(), 37)
    }


    #[test]
    fn should_solve_test_2() {
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
        let simulation = Simulation::new(grid);

        let simulation = simulation.propagate_wide_hold();

        assert_eq!(simulation.count_seats(), 26)
    }
}