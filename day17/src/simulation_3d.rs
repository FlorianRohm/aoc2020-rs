use ndarray::{Array3, Zip};
use ndarray::prelude::s;

use crate::cell::Cell;

#[derive(PartialEq, Debug)]
pub struct Simulation3d {
    pub l_x: usize,
    pub l_y: usize,
    pub l_z: usize,
    pub lattice: Array3<Cell>,
    pub tmp_lattice: Array3<Cell>,
}


macro_rules! inner_slice {
    ( $x:expr ) => {
        s!(1..=$x.l_x, 1..=$x.l_y, 1..=$x.l_z)
    };
}
impl Simulation3d {
    pub fn propagate(mut self) -> Self {
        let source = self.lattice.windows((3, 3, 3));
        let mut target = self.tmp_lattice.slice_mut(inner_slice!(self));
        Zip::from(&mut target).and(source).apply(|tar, src| {
            let occupations = src.iter().filter(|c| c == &&Cell::Active).count();

            let new_cell = match (&src[[1, 1, 1]], occupations) {
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

    pub(crate) fn new(grid: Array3<Cell>) -> Self {
        use ndarray::Array;
        use ndarray::Ix3;

        let (x, y, z) = grid.dim();

        let mut tmp_lattice = unsafe { Array::<Cell, Ix3>::uninitialized((x, y, z)) };
        tmp_lattice.assign(&grid);
        Simulation3d { lattice: grid, tmp_lattice, l_x: x - 2, l_y: y - 2, l_z: z - 2 }
    }
}
