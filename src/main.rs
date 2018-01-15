

#[cfg(test)]
#[macro_use]
extern crate quickcheck;


fn main() {
    println!("Hello, world!");
}

enum Figure {
    Cube,
    Line,
    Base,
    LeftZig,
    RightZig
}

impl Figure {

    fn draw(&self) -> [bool; 16] {
        const O: bool = false;
        const X: bool = true;
        match self {
            &Figure::Cube => [
                O, O, O, O,
                O, X, X, O,
                O, X, X, O,
                O, O, O, O,
            ],
            &Figure::Line => [
                O, O, O, O,
                O, O, O, O,
                X, X, X, X,
                O, O, O, O,
            ],
            &Figure::Base => [
                O, O, O, O,
                O, O, X, O,
                O, X, X, X,
                O, O, O, O,
            ],
            &Figure::LeftZig => [
                O, O, O, O,
                O, X, X, O,
                O, O, X, X,
                O, O, O, O,
            ],
            &Figure::RightZig => [
                O, O, O, O,
                O, X, X, O,
                X, X, O, O,
                O, O, O, O,
            ],
        }
    }

    fn rotate(map: &mut [bool; 16]) {
        fn p(x: usize, y: usize) -> usize {
            x + 4*y
        }

        map.swap(p(0,0), p(3,0));
        map.swap(p(0,0), p(0,3));
        map.swap(p(0,3), p(3,3));

        map.swap(p(0,1), p(2,0));
        map.swap(p(0,1), p(1,3));
        map.swap(p(1,3), p(3,2));

        map.swap(p(0,2), p(1,0));
        map.swap(p(0,2), p(2,3));
        map.swap(p(2,3), p(3,1));

        map.swap(p(1,1), p(2,1));
        map.swap(p(1,1), p(1,2));
        map.swap(p(1,2), p(2,2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    quickcheck! {

        fn four_rotations(v: i32) -> bool {

            let drawn = Figure::LeftZig.draw();
            let mut rotated = drawn.clone();

            Figure::rotate(&mut rotated);
            Figure::rotate(&mut rotated);
            Figure::rotate(&mut rotated);
            Figure::rotate(&mut rotated);

            drawn == rotated
        }
    }
}