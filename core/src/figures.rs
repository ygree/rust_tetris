
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
        let mut blocks = [(0, 0); 4];
        let mut i = 0;

        'main:
        for row in 0 .. 4 {
            for col in 0 .. 4 {
                if figure_map[row * 4 + col] {
                    blocks[i] = (col as i32, row as i32);
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

        let center = (
            blocks.iter().fold(0.0, |sum, &(x, _) | { sum + x as f32 }) / 4.0,
            blocks.iter().fold(0.0, |sum, &(_, y) | { sum + y as f32 }) / 4.0
        );

        FigureRepr {
            blocks,
            center
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FigureRepr {
    /// block coordinates
    pub blocks: [(i32, i32);4],
    /// rotation center
    center: (f32, f32),
}

impl FigureRepr {

    /// create figure out of visual map and normalize its coordinates relatively to its center of mass
    ///
    pub fn new<T: Into<Self>>(another_repr: T) -> Self {
        another_repr.into()
    }

    pub fn rotate(&mut self) {
        let (dx, dy) = self.center;

        for (x, y) in self.blocks.iter_mut() {
            let fy = *y as f32;
            let fx = *x as f32;
            *x = (-(fy - dy) + dx).ceil() as i32;
            *y = (fx - dx + dy).ceil() as i32;
        }
    }

    pub fn center_x(&self) -> isize {
        let (x, _) = self.center;
        x.ceil() as isize
    }

    pub fn min_y(&self) -> isize {
        self.blocks.iter().map(|(_, y)| { *y }).min().unwrap() as isize
    }
}

use rand::distributions::Standard;
use rand::prelude::Distribution;

use rand::Rng;


impl Distribution<Figure> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Figure {
        use self::Figure::*;
        let values = [Cube, Line, Base, LeftZig, RightZig, LeftL, RightL];
        let i = rng.gen_range(0..=6);
        values[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{Arbitrary, quickcheck};
    use quickcheck::Gen;

    impl Arbitrary for Figure {
        fn arbitrary<G: Gen>(_g: &mut G) -> Self {
            rand::thread_rng().gen::<Figure>()
        }
    }

    impl Arbitrary for FigureRepr {

        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let f: Figure = Arbitrary::arbitrary(g);
            f.into()
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct OneToThree(u32);

    impl Arbitrary for OneToThree {

        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            OneToThree(g.gen_range(1, 4))
        }
    }

    quickcheck! {

        /// four consecutive rotations bring figure to its initial shape and position
        fn four_repr_rotations(orig: FigureRepr) -> bool {
            let mut repr = orig.clone();

            repr.rotate();
            repr.rotate();
            repr.rotate();
            repr.rotate();

            orig.blocks == repr.blocks
        }

        /// 1 to 3 consecutive rotations result in distinct figure representation
        fn one_to_three_rotations(orig: FigureRepr, one_to_three: OneToThree) -> bool {
            let mut repr = orig.clone();

            for _ in 0 .. one_to_three.0 {
                repr.rotate();
            }

            orig.blocks != repr.blocks
        }
    }

}