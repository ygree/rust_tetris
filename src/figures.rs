
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

impl From<Figure> for FigureRepr {
    fn from(figure: Figure) -> Self {
        use self::Figure::*;
        const O: bool = false;
        const X: bool = true;
        let array_repr = match figure {
            Cube => [
                O, O, O, O,
                O, X, X, O,
                O, X, X, O,
                O, O, O, O,
            ],
            Line => [
                O, O, O, O,
                O, O, O, O,
                X, X, X, X,
                O, O, O, O,
            ],
            Base => [
                O, O, O, O,
                O, O, X, O,
                O, X, X, X,
                O, O, O, O,
            ],
            LeftZig => [
                O, O, O, O,
                O, X, X, O,
                O, O, X, X,
                O, O, O, O,
            ],
            RightZig => [
                O, O, O, O,
                O, X, X, O,
                X, X, O, O,
                O, O, O, O,
            ],
            RightL => [
                O, X, O, O,
                O, X, O, O,
                O, X, X, O,
                O, O, O, O,
            ],
            LeftL => [
                O, O, X, O,
                O, O, X, O,
                O, X, X, O,
                O, O, O, O,
            ],
        };
        array_repr.into()
    }
}

impl From<[bool;16]> for FigureRepr {

    fn from(figure_map: [bool;16]) -> Self {
        let mut blocks = [Point { x: 0, y: 0 }; 4];
        let mut i = 0;

        'main:
            for row in 0 .. 4 {
            for col in 0 .. 4 {
                if figure_map[row * 4 + col] {
                    blocks[i] = Point { x: col as i32, y: row as i32 };
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

        let center = Point {
            x: blocks.iter().fold(0.0, |sum, &Point { x, .. }| { sum + x as f32 }) / 4.0,
            y: blocks.iter().fold(0.0, |sum, &Point { y, .. }| { sum + y as f32 }) / 4.0
        };

        FigureRepr {
            blocks,
            center
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

#[derive(Clone, Copy, Debug)]
pub struct FigureRepr {
    /// block coordinates
    pub blocks: [Point<i32>;4],
    /// rotation center
    center: Point<f32>,
}

impl FigureRepr {

    /// create figure out of visual map and normalize its coordinates relatively to its center of mass
    ///
    pub fn new<T: Into<Self>>(another_repr: T) -> Self {
        another_repr.into()
    }

    pub fn rotate(&mut self) {
        let dx = self.center.x;
        let dy = self.center.y;

        let mut f_blocks = [Point { x: 0.0, y: 0.0 }; 4];

        for (org, mut norm) in self.blocks.iter().zip(f_blocks.iter_mut()) {
            *norm = Point {
                x: org.x as f32 - dx,
                y: org.y as f32 - dy,
            }
        }

        // rotate
        for &mut Point { ref mut x, ref mut y } in f_blocks.iter_mut() {
            let new_x = -*y;
            let new_y = *x;
            *x = new_x;
            *y = new_y;
        }

        // de-normalize move back from center of mass to origin position by applying rotated shift
        for &mut Point { ref mut x, ref mut y } in f_blocks.iter_mut() {
            *x += dx;
            *y += dy;
        }

        // modify origin coordinates by rounding float point result
        for (&mut Point {ref mut x, ref mut y}, &Point {x: fx, y: fy}) in self.blocks.iter_mut().zip(f_blocks.iter()) {
            *x = fx.ceil() as i32;
            *y = fy.ceil() as i32;
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

        /// four consecutive rotations bring figure to initial shape and its position
        fn four_repr_rotations(f: Figure) -> bool {
            let orig = FigureRepr::new(f);

            let mut repr = orig.clone();

            repr.rotate();
            repr.rotate();
            repr.rotate();
            repr.rotate();

            orig.blocks == repr.blocks
        }
    }

}
