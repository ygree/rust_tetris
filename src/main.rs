

#[cfg(test)]
#[macro_use]
extern crate quickcheck;


fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
enum Figure {
    Cube,
    Line,
    Base,
    LeftZig,
    RightZig
}

impl Figure {

    fn draw(&self) -> FigureMap {
        use Figure::*;
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
        }
    }

}

#[derive(Clone, Copy, Eq, PartialEq)]
struct FigureMap([bool; 16]);

impl FigureMap {

    fn swap(&mut self, (r1, c1): (usize, usize), (r2, c2): (usize, usize) ) {
        fn p(x: usize, y: usize) -> usize {
            x + 4*y
        }
        self.0.swap(p(r1,c1), p(r2,c2));
    }

    fn rotate(&mut self) {
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

use quickcheck::Arbitrary;
use quickcheck::Gen;

impl Arbitrary for Figure {

    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        use Figure::*;
        g.choose(&[Cube, Line, Base, LeftZig, RightZig]).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
