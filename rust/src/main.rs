use std::collections::HashMap;

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
    fn render(&self);
}

fn check_nums(f: u8, s: u8) -> bool {
    ((f & 0b011) == (s & 0b011)) // checks if the last 2 bits match, so it's the same piece
        && ((f & 0b100) != (s & 0b100)) // checks if the first bit doesn't match, so it's not the same orientation
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
        if x1 == x2 + 1 { // left to right
            first_num = first_piece.array[(3 + first_piece.offset) % 4];
            second_num = second_piece.array[(1 + second_piece.offset) % 4];
        } else if x2 > 0 && x1 == x2 - 1 { // right to left
            first_num = first_piece.array[(1 + first_piece.offset) % 4];
            second_num = second_piece.array[(3 + second_piece.offset) % 4];
        } else if y1 == y2 + 1 { // bottom to top
            first_num = first_piece.array[(2 + first_piece.offset) % 4];
            second_num = second_piece.array[(0 + second_piece.offset) % 4];
        } else if y2 > 0 && y1 == y2 - 1 { // top to bottom
            first_num = first_piece.array[(0 + first_piece.offset) % 4];
            second_num = second_piece.array[(2 + second_piece.offset) % 4];
        } else {
            panic!("Invalid check {} {} {} {}", x1, y1, x2, y2);
        }
        if check_nums(first_num, second_num) {
            return true;
        }
        false
    }

    fn check_game(self: &Self) -> bool {
        if !self.check_piece(0, 0, 1, 0) { return false; }
        if !self.check_piece(1, 0, 2, 0) { return false; }
        if !self.check_piece(0, 1, 1, 1) { return false; }
        if !self.check_piece(1, 1, 2, 1) { return false; }
        if !self.check_piece(0, 2, 1, 2) { return false; }
        if !self.check_piece(1, 2, 2, 2) { return false; }
        if !self.check_piece(0, 0, 0, 1) { return false; }
        if !self.check_piece(0, 1, 0, 2) { return false; }
        if !self.check_piece(1, 0, 1, 1) { return false; }
        if !self.check_piece(1, 1, 1, 2) { return false; }
        if !self.check_piece(2, 0, 2, 1) { return false; }
        if !self.check_piece(2, 1, 2, 2) { return false; }
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
                                            unsafe {
                                                TOTAL_ROTATIONS += 1;
                                            }
                                            if self.check_game() {
                                                self.render();
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

    fn render(&self) {
        let pieces = HashMap::from([
            (DB, "DB"),
            (DT, "DT"),
            (GB, "GB"),
            (GT, "GT"),
            (MB, "MB"),
            (MT, "MT"),
            (WB, "WB"),
            (WT, "WT")
        ]);

        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 0).array[0]).unwrap_or(&"??"), pieces.get(&self.get(1, 0).array[0]).unwrap_or(&"??"), pieces.get(&self.get(2, 0).array[0]).unwrap_or(&"??"));
        println!(" {}  {} | {}  {} | {}  {}", pieces.get(&self.get(0, 0).array[3]).unwrap_or(&"??"), pieces.get(&self.get(0, 0).array[1]).unwrap_or(&"??"), pieces.get(&self.get(1, 0).array[3]).unwrap_or(&"??"), pieces.get(&self.get(1, 0).array[1]).unwrap_or(&"??"), pieces.get(&self.get(2, 0).array[3]).unwrap_or(&"??"), pieces.get(&self.get(2, 0).array[1]).unwrap_or(&"??"));
        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 0).array[2]).unwrap_or(&"??"), pieces.get(&self.get(1, 0).array[2]).unwrap_or(&"??"), pieces.get(&self.get(2, 0).array[2]).unwrap_or(&"??"));
        println!("-------------------------");

        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 1).array[0]).unwrap_or(&"??"), pieces.get(&self.get(1, 1).array[0]).unwrap_or(&"??"), pieces.get(&self.get(2, 1).array[0]).unwrap_or(&"??"));
        println!(" {}  {} | {}  {} | {}  {}", pieces.get(&self.get(0, 1).array[3]).unwrap_or(&"??"), pieces.get(&self.get(0, 1).array[1]).unwrap_or(&"??"), pieces.get(&self.get(1, 1).array[3]).unwrap_or(&"??"), pieces.get(&self.get(1, 1).array[1]).unwrap_or(&"??"), pieces.get(&self.get(2, 1).array[3]).unwrap_or(&"??"), pieces.get(&self.get(2, 1).array[1]).unwrap_or(&"??"));
        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 1).array[2]).unwrap_or(&"??"), pieces.get(&self.get(1, 1).array[2]).unwrap_or(&"??"), pieces.get(&self.get(2, 1).array[2]).unwrap_or(&"??"));
        println!("-------------------------");

        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 2).array[0]).unwrap_or(&"??"), pieces.get(&self.get(1, 2).array[0]).unwrap_or(&"??"), pieces.get(&self.get(2, 2).array[0]).unwrap_or(&"??"));
        println!(" {}  {} | {}  {} | {}  {}", pieces.get(&self.get(0, 2).array[3]).unwrap_or(&"??"), pieces.get(&self.get(0, 2).array[1]).unwrap_or(&"??"), pieces.get(&self.get(1, 2).array[3]).unwrap_or(&"??"), pieces.get(&self.get(1, 2).array[1]).unwrap_or(&"??"), pieces.get(&self.get(2, 2).array[3]).unwrap_or(&"??"), pieces.get(&self.get(2, 2).array[1]).unwrap_or(&"??"));
        println!("   {}   |   {}   |   {}  ", pieces.get(&self.get(0, 2).array[2]).unwrap_or(&"??"), pieces.get(&self.get(1, 2).array[2]).unwrap_or(&"??"), pieces.get(&self.get(2, 2).array[2]).unwrap_or(&"??"));
        println!("-------------------------");
    }
}

static mut TOTAL_ROTATIONS: u128 = 0;
static mut TOTAL_PLACES: u64 = 0;

fn permutate(arr: Vec<usize>, second: &mut Vec<usize>) {
    if arr.len() == 0 {
        // second contains what we want
        let mut pieces: [Piece; 9] = [PIECES[second[0]].clone(), PIECES[second[1]].clone(), PIECES[second[2]].clone(), PIECES[second[3]].clone(), PIECES[second[4]].clone(), PIECES[second[5]].clone(), PIECES[second[6]].clone(), PIECES[second[7]].clone(), PIECES[second[8]].clone()];
        if pieces.solve() {
            unsafe {
                println!("Total checked {} {}", TOTAL_ROTATIONS, TOTAL_PLACES);
            }
            println!("Found {:?}", pieces);
            pieces.render();
            panic!("Done");
        }
        unsafe {
            TOTAL_PLACES += 1;
            if TOTAL_PLACES % 80 == 0 {
                println!("Progress {} - {}", TOTAL_PLACES, TOTAL_ROTATIONS);
            }
        }
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
    println!("Nothing found");
}
