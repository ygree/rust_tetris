
extern crate rand;

use figures::Figure;
use figures::FigureMap;
use self::rand::Rand;
use self::rand::Rng;

pub struct Glass {
    pub width: usize,
    pub height: usize,
    map: Vec<bool>,
    pub figure: Option<FigureInGlass>,
}

#[derive(Copy, Clone)]
pub struct FigureInGlass {
    pub figure: FigureMap,
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

    pub fn place(&mut self, fmap: FigureMap, (row, col): (isize, isize)) -> bool {
        if !self.fit_glass(&fmap, (row, col)) { false }
        else {
            self.figure = Some(FigureInGlass {
                figure: fmap,
                position: (row, col)
            });
            true
        }
    }

    /// (row, col) - position in the glass. 0 row is the upper row.
    fn fit_glass(&self, fmap: &FigureMap, (row, col): (isize, isize)) -> bool {

        for figure_row in 0 .. fmap.height() {
            for figure_col in 0 .. fmap.width() {
                let glass_row = row + figure_row as isize;
                let glass_col = col + figure_col as isize;

                let is_outsize_glass =
                    glass_row < 0 || glass_row >= self.height as isize ||
                    glass_col < 0 || glass_col >= self.width as isize;

                let has_value = fmap[figure_row][figure_col];

                if has_value {
                    if is_outsize_glass || self[glass_row as usize][glass_col as usize] {
                        return false;
                    }
                }
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

            for figure_row in 0 .. figure.height() {
                for figure_col in 0..figure.width() {
                    let glass_row = row + figure_row as isize;
                    let glass_col = col + figure_col as isize;

                    let is_outsize_glass =
                        glass_row < 0 || glass_row > self.height as isize || //TODO duplicates see fit_glass
                        glass_col < 0 || glass_col > self.width as isize;

                    let has_value = figure[figure_row][figure_col];

                    if has_value && !is_outsize_glass {
                        self[glass_row as usize][glass_col as usize] = true;
                    }
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
        let figure_map = figure.draw();
        let row = 0;
        let col = (self.width as isize - 4) / 2 - 1;
        !self.place(figure_map, (row, col))
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
