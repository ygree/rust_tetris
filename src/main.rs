

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

    fn rotate(&mut self) {
        fn p(x: usize, y: usize) -> usize {
            x + 4*y
        }

        self.0.swap(p(0,0), p(3,0));
        self.0.swap(p(0,0), p(0,3));
        self.0.swap(p(0,3), p(3,3));

        self.0.swap(p(0,1), p(2,0));
        self.0.swap(p(0,1), p(1,3));
        self.0.swap(p(1,3), p(3,2));

        self.0.swap(p(0,2), p(1,0));
        self.0.swap(p(0,2), p(2,3));
        self.0.swap(p(2,3), p(3,1));

        self.0.swap(p(1,1), p(2,1));
        self.0.swap(p(1,1), p(1,2));
        self.0.swap(p(1,2), p(2,2));
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
