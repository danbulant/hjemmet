
#[derive(Default)]
struct Piece {
    offset: usize,
    array: [u8; 4]
}


const DB: u8 = 0b000;
const DT: u8 = 0b100;
const GB: u8 = 0b001;
const GT: u8 = 0b101;
const MB: u8 = 0b010;
const MT: u8 = 0b110;
const WB: u8 = 0b011;
const WT: u8 = 0b111;

const pieces: [Piece; 9] = [
    Piece { offset: 0, array: [WB, MT, GT, DB] },
    Piece { offset: 0, array: [DB, WT, GT, MB] },
    Piece { offset: 0, array: [GB, MT, DT, WB] },
    Piece { offset: 0, array: [GB, MT, DT, WB] },
    Piece { offset: 0, array: [GB, WT, DT, MB] },
    Piece { offset: 0, array: [DB, WT, GT, WB] },
    Piece { offset: 0, array: [DB, GB, WT, MT] },
    Piece { offset: 0, array: [DB, MT, WT, GB] },
    Piece { offset: 0, array: [GB, MT, DB, MB] },
];

trait Pieces {
    fn get(self: &'static Self, x: usize, y: usize) -> &'static Piece;
    fn check_piece(self: &'static Self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool;
    fn check_game(self: &'static Self) -> bool;
    fn solve(&'static self) -> bool;
}

impl Pieces for [Piece; 9] {
    fn get(self: &'static Self, x: usize, y: usize) -> &'static Piece {
        &self[x * 3 + y]
    }

    fn check_piece(self: &'static Self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let first_piece = self.get(x1, y1);
        let second_piece = self.get(x2, y2);
        let first_num: u8;
        let second_num: u8;
        if x1 == x2 + 1 {
            first_num = first_piece.array[(3 + first_piece.offset) % 4];
            second_num = second_piece.array[(1 + first_piece.offset) % 4];
        } else if x1 == x2 - 1 {
            first_num = first_piece.array[(1 + first_piece.offset) % 4];
            second_num = second_piece.array[(3 + first_piece.offset) % 4];
        } else if y1 == y2 + 1 {
            first_num = first_piece.array[(2 + first_piece.offset) % 4];
            second_num = second_piece.array[(0 + first_piece.offset) % 4];
        } else if y1 == y2 - 1 {
            first_num = first_piece.array[(0 + first_piece.offset) % 4];
            second_num = second_piece.array[(2 + first_piece.offset) % 4];
        } else {
            panic!("Invalid check");
        }
        if ((first_num >> 1) & (second_num >> 1) == first_num >> 1) && ((first_num & 1) ^ (second_num & 1) == 0) {
            return true;
        }
        false
    }

    fn check_game(self: &'static Self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if x < 2 && self.check_piece(x, y, x + 1, y) {
                    return false;
                }
                if y < 2 && self.check_piece(x, y, x, y + 2) {
                    return false;
                }   
            }
        }
        true
    }
    
    fn solve(&'static self) -> bool {
        if self.check_game() {
            return true;
        }
        for _ in 1..4 {
            self[0].offset += 1;
            if self.check_game() {
                return true;
            }
        }
        false
    }
}

fn main() {
    
}
