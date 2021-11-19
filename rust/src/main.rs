#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
struct Piece {
    offset: usize,
    array: [u8; 4],
}

const DB: u8 = 0b000;
const DT: u8 = 0b100;
const GB: u8 = 0b001;
const GT: u8 = 0b101;
const MB: u8 = 0b010;
const MT: u8 = 0b110;
const WB: u8 = 0b011;
const WT: u8 = 0b111;

const PIECES: [Piece; 9] = [
    Piece {
        offset: 0,
        array: [WB, MT, GT, DB],
    },
    Piece {
        offset: 0,
        array: [DB, WT, GT, MB],
    },
    Piece {
        offset: 0,
        array: [GB, MT, DT, WB],
    },
    Piece {
        offset: 0,
        array: [GB, MT, DT, WB],
    },
    Piece {
        offset: 0,
        array: [GB, WT, DT, MB],
    },
    Piece {
        offset: 0,
        array: [DB, WT, GT, WB],
    },
    Piece {
        offset: 0,
        array: [DB, GB, WT, MT],
    },
    Piece {
        offset: 0,
        array: [DB, MT, WT, GB],
    },
    Piece {
        offset: 0,
        array: [GB, MT, DB, MB],
    },
];

trait Pieces {
    fn get(self: &Self, x: usize, y: usize) -> &Piece;
    fn check_piece(self: &Self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool;
    fn check_game(self: &Self) -> bool;
    fn solve(&mut self) -> bool;
}

impl Pieces for [Piece; 9] {
    fn get(self: &Self, x: usize, y: usize) -> &Piece {
        return &self[x * 3 + y];
    }

    fn check_piece(self: &Self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        let first_piece = self.get(x1, y1);
        let second_piece = self.get(x2, y2);
        let first_num: u8;
        let second_num: u8;
        if x1 == x2 + 1 {
            first_num = first_piece.array[(3 + first_piece.offset) % 4];
            second_num = second_piece.array[(1 + first_piece.offset) % 4];
        } else if x2 > 0 && x1 == x2 - 1 {
            first_num = first_piece.array[(1 + first_piece.offset) % 4];
            second_num = second_piece.array[(3 + first_piece.offset) % 4];
        } else if y1 == y2 + 1 {
            first_num = first_piece.array[(2 + first_piece.offset) % 4];
            second_num = second_piece.array[(0 + first_piece.offset) % 4];
        } else if y2 > 0 && y1 == y2 - 1 {
            first_num = first_piece.array[(0 + first_piece.offset) % 4];
            second_num = second_piece.array[(2 + first_piece.offset) % 4];
        } else {
            panic!("Invalid check {} {} {} {}", x1, y1, x2, y2);
        }
        if ((first_num >> 1) & (second_num >> 1) == first_num >> 1)
            && ((first_num & 1) ^ (second_num & 1) == 0)
        {
            return true;
        }
        false
    }

    fn check_game(self: &Self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if x < 2 && !self.check_piece(x, y, x + 1, y) {
                    return false;
                }
                if y < 2 && !self.check_piece(x, y, x, y + 1) {
                    return false;
                }
            }
        }
        true
    }
    fn solve(&mut self) -> bool {
        if self.check_game() {
            return true;
        }
        for _ in 1..4 {
            let mut item = &mut self[0];
            item.offset += 1;
            if item.offset > 4 { item.offset = 0; }
            for _ in 1..4 {
                let mut item = &mut self[1];
                item.offset += 1;
                if item.offset > 4 { item.offset = 0; }
                for _ in 1..4 {
                    let mut item = &mut self[2];
                    item.offset += 1;
                    if item.offset > 4 { item.offset = 0; }
                    for _ in 1..4 {
                        let mut item = &mut self[3];
                        item.offset += 1;
                        if item.offset > 4 { item.offset = 0; }
                        for _ in 1..4 {
                            let mut item = &mut self[4];
                            item.offset += 1;
                            if item.offset > 4 { item.offset = 0; }
                            for _ in 1..4 {
                                let mut item = &mut self[5];
                                item.offset += 1;
                                if item.offset > 4 { item.offset = 0; }
                                for _ in 1..4 {
                                    let mut item = &mut self[6];
                                    item.offset += 1;
                                    if item.offset > 4 { item.offset = 0; }
                                    for _ in 1..4 {
                                        let mut item = &mut self[7];
                                        item.offset += 1;
                                        if item.offset > 4 { item.offset = 0; }
                                        for _ in 1..4 {
                                            let mut item = &mut self[8];
                                            item.offset += 1;
                                            if item.offset > 4 { item.offset = 0; }
                                            if self.check_game() {
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

fn permutate(arr: Vec<usize>, second: &mut Vec<usize>) {
    if arr.len() == 0 {
        // second contains what we want
        let mut pieces: [Piece; 9] = [PIECES[second[0]].clone(), PIECES[second[1]].clone(), PIECES[second[2]].clone(), PIECES[second[3]].clone(), PIECES[second[4]].clone(), PIECES[second[5]].clone(), PIECES[second[6]].clone(), PIECES[second[7]].clone(), PIECES[second[8]].clone()];
        if pieces.solve() {
            println!("Foun {:?}", pieces);
            panic!("Done");
        }
        println!("Not yet");
    } else {
        let max = arr.len();
        for i in 0..max {
            let mut curr = arr.clone();
            let mut second = second.clone();
            second.push(curr[i]);
            curr.remove(i);
            permutate(curr.clone(), &mut second.clone());
        }
    }
}

fn main() {
    let mut piece_cloned: [Piece; 9] = PIECES.clone();
    if piece_cloned.solve() {
        println!("Found {:?}", piece_cloned);
        return;
    }
    let mut pieces = Vec::new();
    println!("Permutating");
    let vec = vec![0 as usize, 1, 2, 3, 4, 5, 6, 7, 8];
    permutate(vec, &mut pieces);
}
