// const { performance } = require("perf_hooks");

const DB = 0, DT = 1, GB = 2, GT = 3, MB = 4, MT = 5, WB = 6, WT = 7;

const startNow = Date.now();
const now = () => Date.now() - startNow;
const log = (...str) => console.log(Math.round(now()).toString().padStart(5, "0"), ...str);
var int = false;

class Piece extends Array {
    rotate(num = 1) {
        return ;
        for (let i = 0; i < num; i++) {
            this.push(this.shift());
        }
    }
}

const P = (f, s, t, fr) => new Piece(f, s, t, fr);

const permutator = (inputArr) => {
    let result = [];

    const permute = (arr, m = []) => {
        if (arr.length === 0) {
            result.push(m)
        } else {
            for (let i = 0; i < arr.length; i++) {
                let curr = arr.slice();
                let next = curr.splice(i, 1);
                permute(curr.slice(), m.concat(next))
            }
        }
    }

    permute(inputArr)

    return result;
}

const pieces = [
    P(WB, MT, GT, DB),
    P(DB, WT, GT, MB),
    P(GB, MT, DT, WB),
    P(GB, MT, DT, WB),
    P(GB, WT, DT, MB),
    P(DB, WT, GT, WB),
    P(DB, GB, WT, MT),
    P(DB, MT, WT, GB),
    P(GB, MT, DB, MB),
];

var total = 0;

class Game {
    /**
     * @param {Piece[]} pieces
     */
    constructor(pieces) {
        this.pieces = pieces;
    }

    get(x, y) {
        return this.pieces[x * 3 + y];
    }

    checkPiece(x1, y1, x2, y2) {
        let first = this.get(x1, y1);
        let second = this.get(x2, y2);
        if (x1 === x2 + 1) {
            first = first[3];
            second = second[1];
        } else if (x1 === x2 - 1) {
            first = first[1];
            second = second[3];
        } else if (y1 === y2 + 1) {
            first = first[2];
            second = second[0];
        } else if (y1 === y2 - 1) {
            first = first[0];
            second = second[2];
        } else throw new Error("What the fuck");
        if (first % 2 === 0 && second - first !== 1) return false;
        if (first % 2 !== 0 && first - second !== 1) return false;
        return true;
    }

    check() {
        for (let x = 0; x < 3; x++) {
            for (let y = 0; y < 3; y++) {
                if (x < 2) if (!this.checkPiece(x, y, x + 1, y)) return false;
                if (y < 2) if (!this.checkPiece(x, y, x, y + 1)) return false;
            }
        }
    }

    /**
     * Runs for each possible position (permutation), checks all rotations.
     */
    solve() {
        if(this.check()) return true;
        for(let a = 0; a < 4; a++) {
            this.pieces[0].rotate();
            for(let b = 0; b < 4; b++) {
                this.pieces[1].rotate();
                for(let c = 0; c < 4; c++) {
                    this.pieces[2].rotate();
                    for(let d = 0; d < 4; d++) {
                        this.pieces[3].rotate();
                        for(let e = 0; e < 4; e++) {
                            this.pieces[4].rotate();
                            for(let f = 0; f < 4; f++) {
                                this.pieces[5].rotate();
                                for(let g = 0; g < 4; g++) {
                                    this.pieces[6].rotate();
                                    for(let h = 0; h < 4; h++) {
                                        this.pieces[7].rotate();
                                        for(let i = 0; i < 4; i++) {
                                            this.pieces[8].rotate();
                                            total++;
                                            // if(this.check()) return true;
                                            if(int) {
                                                console.log(this.pieces);
                                                return;
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
        return false;
    }

    /**
     * Gets all permutations (i.e. possible positions) of the piece array, and calls solve on all of them.
     */
    static solve() {
        const permutations = permutator(pieces);
        var i = 0;
        for(const permutation of permutations) {
            i++;
            const game = new Game(permutation);
            if(game.solve()) {
                log("Got a HIT");
                log(game, game.pieces);
                return;
            }
            if(i % 10 === 0)
                log(`total ${permutations.length}, checked ${i} int ${int}`);
            if(int) return;
        }
    }
}

process.on("SIGINT", () => {
    int = true;
})

Game.solve();
console.log(total);