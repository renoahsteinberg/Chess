// https://de.wikipedia.org/wiki/Schach
fn main() {
    // let p: u64 = 0b00001000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;
    // let idx = 4;
    // let k = p >> (63-idx); // & 1 to compare rightmost bit // 0b10101101 & 0b10000000 (mask for the leftmost bit) != 0
    // let z = (p >> (63-idx)) & 1;
    // let k = p & (1u64 << (63-idx));
    // println!("{:064b}", k)

    let board = Board::init_board(
       0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000, 
       0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111, 
       0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000, 
       0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010, 
       0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100, 
       0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001, 
       0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000, 
       0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
    );

    board.visualize_board();


    // let test = board.occupied(5, 2); works as intended
    // println!("{}", test);


    // let k = board.get_white_pawns();
    // println!("{:064b}", k);
}


// https://en.wikipedia.org/wiki/Bitboard
// https://www.chessprogramming.org/Bitboard_Board-Definition (Denser Board inspiration)
#[allow(dead_code)]
struct Board {
    white: u64, // all white pieces, self.white & self.pawns
    black: u64, // all black pieces, self.back & self.bishops
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
}

#[allow(dead_code)]
impl Board {
    fn init_board(white: u64, black: u64, pawns: u64, knights: u64, bishops: u64, rooks: u64, queens: u64, kings: u64) -> Self {
        Board {
            white,
            black,
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
        }
    }

    fn get_piece(&self, is_white: bool, piece_mask: u64) -> u64 {
        if is_white {
            self.white & piece_mask
        }
        else {
            self.black & piece_mask
        }
    }

    fn get_by_index(&self, row: i8, col: i8) -> Option<(&str, &str)> {
        // let k = p & (1u64 << (63-idx));
        if row < 0 || row > 7 || col < 0 || col > 7 { return None }
        if !self.occupied(row, col) { return None }

        let idx = (col * 8) + row;
        let mask = 1u64 << (63-idx);

        let color = if self.white & mask != 0 { "w" } 
        else if self.black & mask != 0 { "b" } 
        else { return None };

        let pieces = [
            (self.pawns, "P"),
            (self.knights, "N"),
            (self.bishops, "B"),
            (self.rooks, "R"),
            (self.queens, "Q"),
            (self.kings, "K"),
        ];

        let piece = pieces
            .iter()
            .find(|(bb, _)| bb & mask != 0)
            .map(|(_, symbol)| *symbol).unwrap();

        Some((color, piece)) // color will be used later for choosing the right tile for the piece
    }

    fn occupied(&self, row: i8, col: i8) -> bool {
        let idx = (col * 8) + row;
        if idx > 63 || idx < 0 { return false }
        (((self.white | self.black) >> (63-idx)) & 1) == 1
    }

    fn visualize_board(&self) {
        for row in 0..8 {
            for col in 0..8 {
                if let Some((color, piece)) = self.get_by_index(row, col) {
                    print!(" {}{} ", color, piece);
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
    }
}