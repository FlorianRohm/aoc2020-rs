use ndarray::{Array3, Array4, Zip};
use ndarray::prelude::s;

use crate::cell::Cell;

#[derive(PartialEq, Debug)]
pub struct Simulation4d {
    pub d1: usize,
    pub d2: usize,
    pub d3: usize,
    pub d4: usize,
    pub lattice: Array4<Cell>,
    pub tmp_lattice: Array4<Cell>,
}


macro_rules! inner_slice {
    ( $x:expr ) => {
        s!(1..=$x.d1, 1..=$x.d2, 1..=$x.d3, 1..=$x.d4)
    };
}
impl Simulation4d {
    pub fn propagate(mut self) -> Self {
        let source = self.lattice.windows((3, 3, 3, 3));
        let mut target = self.tmp_lattice.slice_mut(inner_slice!(self));
        Zip::from(&mut target).and(source).apply(|tar, src| {
            let occupations = src.iter().filter(|c| c == &&Cell::Active).count();

            let new_cell = match (&src[[1, 1, 1, 1]], occupations) {
                (Cell::Active, o) if o == 3 || o == 4 => Cell::Active,
                (Cell::Inactive, o) if o == 3 => Cell::Active,
                _ => Cell::Inactive
            };

            *tar = new_cell;
        });
        std::mem::swap(&mut self.tmp_lattice, &mut self.lattice);

        self
    }

    pub fn count_active(&self) -> usize {
        self.lattice.iter().filter(|c| c == &&Cell::Active).count()
    }

    pub(crate) fn new(grid: Array4<Cell>) -> Self {
        use ndarray::Array;
        use ndarray::Ix4;

        let (d1, d2, d3, d4) = grid.dim();

        let mut tmp_lattice = unsafe { Array::<Cell, Ix4>::uninitialized((d1, d2, d3, d4)) };
        tmp_lattice.assign(&grid);
        Simulation4d { lattice: grid, tmp_lattice, d1: d1 - 2, d2: d2 - 2, d3: d3 - 2, d4: d4 - 2 }
    }
}
