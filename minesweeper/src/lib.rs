struct Board {
    cells: Vec<Vec<char>>,
    rows_cnt: usize,
    cols_cnt: usize,
}

impl Board {
    fn annotated(&self) -> Vec<String> {
        (0..self.rows_cnt).map(|y| self.annotated_row(y)).collect()
    }

    fn annotated_row(&self, y: usize) -> String {
        self.cells[y]
            .iter()
            .enumerate()
            .map(|(x, &c)| self.annotated_char(x, y, c))
            .collect()
    }

    fn annotated_char(&self, x: usize, y:usize, c:char) -> char {
        match c {
            ' ' => self.get_adjacent_mines_digit(x, y),
            c @ _ => c,
        }
    }

    fn get_adjacent_mines_digit(&self, x: usize, y: usize) -> char {
        let mut count = 0;
        let xi = x as isize;
        let yi = y as isize;

        for j in yi-1 .. yi+2 as isize {
            for i in xi-1 .. xi+2 as isize {
                if let Some((nx, ny)) = self.normalized_cell(i, j) {
                    if self.cells[ny][nx] == '*' {
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            (count + ('0' as u8)) as char
        } else {
            ' '
        }
    }

    fn normalized_cell(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x >= 0 &&
            y >= 0 &&
            x < self.cols_cnt as isize &&
            y < self.rows_cnt as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    fn new(cells: &[&str]) -> Board {
        Board{cells: cells.iter().map(|&s| s.chars().collect()).collect(),
              rows_cnt: cells.len(),
              cols_cnt: cells[0].len()}
    }
}

pub fn annotate(cells: &[&str]) -> Vec<String> {
    let board = Board::new(cells);
    board.annotated()
}
