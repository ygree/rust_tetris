
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T
}

/// FigureRepr is a replacement candidate for Figure and FigureMap all together
#[derive(Clone, Copy, Debug)]
pub struct FigureRepr {
    /// block coordinates relatively to rotation center
    blocks: [Point<i32>;4]
}

impl FigureRepr {

    /// create figure out of visual map and normalize its coordinates relatively to its center of mass
    ///
    /// TODO: (it can be a macro)
    fn new(figure_map: &FigureMap) -> Self {
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

    fn rotate(&mut self) {
        // find center of mass for rotation
        let dx = self.blocks.iter().fold(0.0, |sum, &Point { x, y }| { sum + x as f32 });
        let dy = self.blocks.iter().fold(0.0, |sum, &Point { x, y }| { sum + y as f32 });

        let mut f_blocks = [Point { x: 0.0, y: 0.0 }; 4];

        // TODO use pattern matching
        // normalize relatively of center of mass
        for (org, mut norm) in self.blocks.iter().zip(f_blocks.iter_mut()) {
            *norm = Point {
                x: org.x as f32 - dx,
                y: org.y as f32 - dy,
            }
        }

        // rotate
        for &mut Point { mut x, mut y } in f_blocks.iter_mut() {
            let new_x = -y;
            let new_y = x;
            x = new_x;
            y = new_y;
        }

        // de-normalize move back from center of mass to origin position
        for &mut Point { mut x, mut y } in f_blocks.iter_mut() {
            x += dx;
            y += dy;
        }

        // modify origin coordinates by rounding float point result
        for (&mut Point {mut x, mut y}, &Point {x: fx, y: fy}) in self.blocks.iter_mut().zip(f_blocks.iter()) {
            x = fx.round() as i32;
            y = fy.round() as i32;
        }
    }
}

extern crate rand;

use self::rand::Rand;
use self::rand::Rng;

impl Rand for Figure {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        use self::Figure::*;
        let values = [Cube, Line, Base, LeftZig, RightZig, LeftL, RightL];
        *rng.choose(&values).unwrap()
    }
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

//    impl Arbitrary for FigureRepr {
//
//        fn arbitrary<G: Gen>(g: &mut G) -> Self {
//            //TODO generate figure and render it
//        }
//    }

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

        fn four_repr_rotations(f: Figure) -> bool {
            let orig = FigureRepr::new(&f.draw());

            let mut repr = orig.clone();

            repr.rotate();
            repr.rotate();
            repr.rotate();
            repr.rotate();

            orig.blocks == repr.blocks
        }
    }

}
