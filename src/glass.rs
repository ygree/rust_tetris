
extern crate rand;

use figures::Figure;
use figures::FigureRepr;
use figures::Point;
use self::rand::Rng;

pub struct Glass {
    pub width: usize,
    pub height: usize,
    map: Vec<bool>,
    pub figure: Option<FigureInGlass>,
}

#[derive(Copy, Clone)]
pub struct FigureInGlass {
    pub figure: FigureRepr,
    pub position: (isize, isize),
}

pub enum MoveDirection {
    Left,
    Right,
    Down
}

impl MoveDirection {

    fn change_pos(self, (row, col): (isize, isize)) -> (isize, isize) {
        use self::MoveDirection::*;
        match self {
            Left => (row, col - 1),
            Right => (row, col + 1),
            Down => (row + 1, col),
        }
    }
}

impl Glass {
    pub fn new(width: usize, height: usize) -> Glass {
        Glass {
            width,
            height,
            map: vec![false; width * height],
            figure: None
        }
    }

    pub fn place(&mut self, figure: FigureRepr, (row, col): (isize, isize)) -> bool {
        if !self.fit_glass(&figure, (row, col)) { false }
        else {
            self.figure = Some(FigureInGlass {
                figure,
                position: (row, col)
            });
            true
        }
    }

    /// (row, col) - position in the glass. 0 row is the upper row.
    fn fit_glass(&self, fmap: &FigureRepr, (row, col): (isize, isize)) -> bool {

        for &Point { x, y } in fmap.blocks.iter() {
            let glass_row = row + y as isize;
            let glass_col = col + x as isize;

            let is_outsize_glass =
                glass_row < 0 || glass_row >= self.height as isize ||
                    glass_col < 0 || glass_col >= self.width as isize;

            let taken = || { self[glass_row as usize][glass_col as usize] };

            if is_outsize_glass || taken() {
                return false;
            }
        }
        true
    }

    pub fn rotate_figure(&mut self) -> bool {
        let orig_figure = self.figure.clone();
        if let Some(FigureInGlass { mut figure, position }) = orig_figure {
            figure.rotate();
            if self.fit_glass(&figure, position) {
                self.figure = Some(FigureInGlass{figure, position});
                return true;
            }
        }
        false
    }

    pub fn relocate_figure(&mut self, direction: MoveDirection) -> bool {
        let orig_figure = self.figure.clone();
        if let Some(FigureInGlass { figure, position }) = orig_figure {
            let new_position = direction.change_pos(position);
            if self.fit_glass(&figure, new_position) {
                self.figure = Some(FigureInGlass{figure, position: new_position});
                return true;
            }
        }
        false
    }

    pub fn freeze_figure(&mut self) {
        if let Some( FigureInGlass { figure, position: (row, col) } ) = self.figure.take() {
            for &Point { x, y } in figure.blocks.iter() {
                let glass_row = row + y as isize;
                let glass_col = col + x as isize;

                let is_outsize_glass =
                    glass_row < 0 || glass_row > self.height as isize || //TODO duplicates see fit_glass
                    glass_col < 0 || glass_col > self.width as isize;

                if !is_outsize_glass {
                    self[glass_row as usize][glass_col as usize] = true;
                }
            }
        }
    }

    pub fn clean_filled_rows(&mut self) {
        for row in (0 .. self.height).rev() {
            loop {
                let filled_up = (0..self.width).all(|col| {
                    self[row][col]
                });

                if !filled_up { break }
                else {
                    for r in (0.. row).rev() { //TODO can be optimized
                        for col in 0..self.width {
                            self[r+1][col] = self[r][col];
                        }
                    }
                }
            }
        }
    }

    pub fn next_figure(&mut self) -> bool {
        let figure = rand::random::<Figure>();
        let mut figure_repr = FigureRepr::new(figure);

        for _ in 0..rand::thread_rng().gen_range(0, 4) {
            figure_repr.rotate();
        }

        let row = 0 - figure_repr.min_y();
        let col = (self.width as isize) / 2 - figure_repr.center_x();
        !self.place(figure_repr, (row, col))
    }
}


impl ::std::ops::Index<usize> for Glass {
    type Output = [bool];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.map[start .. start + self.width]
    }
}

impl ::std::ops::IndexMut<usize> for Glass {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.map[start .. start + self.width]
    }
}

//TODO how to test Glass?