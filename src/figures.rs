
#[cfg(test)]
extern crate quickcheck;

#[derive(Debug, Clone, Copy)]
pub enum Figure {
    Cube,
    Line,
    Base,
    LeftZig,
    RightZig,
    RightL,
    LeftL
}

impl Figure {

    pub fn draw(&self) -> FigureMap {
        use self::Figure::*;
        const O: bool = false;
        const X: bool = true;
        match *self {
            Cube => FigureMap([
                O, O, O, O,
                O, X, X, O,
                O, X, X, O,
                O, O, O, O,
            ]),
            Line => FigureMap([
                O, O, O, O,
                O, O, O, O,
                X, X, X, X,
                O, O, O, O,
            ]),
            Base => FigureMap([
                O, O, O, O,
                O, O, X, O,
                O, X, X, X,
                O, O, O, O,
            ]),
            LeftZig => FigureMap([
                O, O, O, O,
                O, X, X, O,
                O, O, X, X,
                O, O, O, O,
            ]),
            RightZig => FigureMap([
                O, O, O, O,
                O, X, X, O,
                X, X, O, O,
                O, O, O, O,
            ]),
            RightL => FigureMap([
                O, X, O, O,
                O, X, O, O,
                O, X, X, O,
                O, O, O, O,
            ]),
            LeftL => FigureMap([
                O, O, X, O,
                O, O, X, O,
                O, X, X, O,
                O, O, O, O,
            ]),
        }
    }

}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FigureMap([bool; 16]);

impl FigureMap {
    pub fn height(&self) -> usize {
        4
    }

    pub fn width(&self) -> usize {
        4
    }
}

impl ::std::ops::Index<usize> for FigureMap {
    type Output = [bool];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * 4;
        &self.0[start .. start + 4]
    }
}

impl ::std::ops::IndexMut<usize> for FigureMap {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * 4;
        &mut self.0[start .. start + 4]
    }
}

impl FigureMap {

    fn swap(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize) ) {
        fn p(row: usize, column: usize) -> usize {
            row + 4*column
        }
        self.0.swap(p(r1,c1), p(r2,c2));
    }

    fn center_of_mass(&self) -> (f32, f32) {
        let mut sum_row = 0;
        let mut sum_col = 0;
        let mut count = 0;
        for row in 0 .. 4 {
            for col in 0 .. 4 {
                if self[row][col] {
                    sum_row += row;
                    sum_col += col;
                    count += 1;
                }
            }
        }
        (sum_row as f32 / count as f32, sum_col as f32 / count as f32)
    }

    pub fn rotate(&mut self) {
        self.swap((0,0), (3,0));
        self.swap((0,0), (0,3));
        self.swap((0,3), (3,3));

        self.swap((0,1), (2,0));
        self.swap((0,1), (1,3));
        self.swap((1,3), (3,2));

        self.swap((0,2), (1,0));
        self.swap((0,2), (2,3));
        self.swap((2,3), (3,1));

        self.swap((1,1), (2,1));
        self.swap((1,1), (1,2));
        self.swap((1,2), (2,2));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

/// FigureRepr is a replacement candidate for Figure and FigureMap all together
pub struct FigureRepr {
    /// block coordinates relatively to rotation center
    blocks: [Point;4]
}

impl FigureRepr {

    /// create figure out of visual map and normalize its coordinates relatively to its center of mass
    ///
    /// TODO: (it can be a macros)
    fn new(figure_map: &FigureMap) -> FigureRepr {
        let mut blocks = [Point { x: 0, y: 0 }; 4];
        let mut i = 0;

        let (row, col) = figure_map.center_of_mass();
        let (x0, y0) = (col.round() as i32, row.round() as i32);

        'main:
        for row in 0 .. 4 {
            for col in 0 .. 4 {
                if figure_map[row][col] {
                    blocks[i] = Point { x: col as i32 - x0, y: row as i32 - y0 };
                    i += 1;
                    if i == 4 {
                        break 'main;
                    }
                }
            }
        }

        if i != 4 {
            panic!("Not enough points to construct FigureRepr. It need to be exactly 4, but provided: {}", i);
        }

        FigureRepr {
            blocks
        }
    }

    //TODO: rotate figure repr relatively to its center
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::Arbitrary;
    use quickcheck::Gen;

    impl Arbitrary for Figure {

        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            use self::Figure::*;
            g.choose(&[Cube, Line, Base, LeftZig, RightZig, LeftL, RightL]).unwrap().clone()
        }
    }

    quickcheck! {

        fn four_rotations(f: Figure) -> bool {
            let drawn = f.draw();
            let mut rotated = drawn.clone();

            rotated.rotate();
            rotated.rotate();
            rotated.rotate();
            rotated.rotate();

            drawn == rotated
        }

        fn center_of_mass_exist(f: Figure) -> bool {
            let figure_map = f.draw();

            let (row, col) = figure_map.center_of_mass();

            0.0 <= row && row < 4.0 && 0.0 <= col && col < 4.0
        }

        fn figure_repr_has_only_one_center(f: Figure) -> bool {
            let figure_map = f.draw();
            let figure_repr = FigureRepr::new(&figure_map);

            figure_repr.blocks.iter().filter(|b| { b.x == 0 && b.y == 0 }).count() == 1
        }

        // check that FigureRepr center of mass is (0,0)
//        fn figure_repr_normalized(f: Figure) -> bool {
//            let figure_map = f.draw();
//            let figure_repr = FigureRepr::new(&figure_map);
//
//            let mut x = 0;
//            let mut y = 0;
//
//            for p in figure_repr.blocks.iter() {
//                x += p.x;
//                y += p.y;
//            }
//
//            println!("{:?} center: {} {} : {:?}", f, x, y, &figure_repr.blocks);
//            x == 0 && y == 0
//        }
    }
}
