pub struct ChessPosition {
    row: i8,
    col: i8,
}

impl ChessPosition {
    pub fn new(row: u8, col: u8) -> Result<ChessPosition, String> {
        if row < 8 && col < 8 {
            Ok(ChessPosition{row: row as i8, col: col as i8})
        } else {
            Err("Invalid Position".to_string())
        }
    }
}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen{pos: pos}
    }
    pub fn can_attack(&self, queen: &Queen) -> bool {
        let row_distance = (self.pos.row - queen.pos.row).abs();
        let col_distance = (self.pos.col - queen.pos.col).abs();

        if row_distance == 0 {
            true
        } else if col_distance == 0 {
            true
        } else if row_distance == col_distance {
            true
        } else {
            false
        }
    }
}
