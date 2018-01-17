
use figures::Figure;
use figures::FigureMap;

struct Glass {
    width: usize,
    height: usize,
    map: Vec<bool>,
    figure: Option<FigureInGlass>,
}

#[derive(Copy, Clone)]
struct FigureInGlass {
    figure: FigureMap,
    position: (isize, isize),
}

enum MoveDirection {
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
    fn new(width: usize, height: usize) -> Glass {
        Glass {
            width,
            height,
            map: Vec::with_capacity(width * height),
            figure: None
        }
    }

    fn place(&mut self, fmap: FigureMap, (row, col): (isize, isize)) -> bool {
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
                    glass_row < 0 || glass_row > self.width as isize ||
                    glass_col < 0 || glass_col > self.height as isize;

                let has_value = fmap[figure_row][figure_col];

                if has_value && (is_outsize_glass || self[glass_row as usize][glass_col as usize]) {
                    return false;
                }
            }
        }
        true
    }

    fn rotate_figure(&mut self) -> bool {
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

    fn relocate_figure(&mut self, direction: MoveDirection) -> bool {
        let orig_figure = self.figure.clone();
        if let Some(FigureInGlass { mut figure, position }) = orig_figure {
            let new_position = direction.change_pos(position);
            if self.fit_glass(&figure, new_position) {
                self.figure = Some(FigureInGlass{figure, position: new_position});
                return true;
            }
        }
        false
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