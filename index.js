const { performance } = require("perf_hooks");

const DB = 0, DT = 1, GB = 2, GT = 3, MB = 4, MT = 5, WB = 6, WT = 7;

const startNow = performance.now();
const now = () => performance.now() - startNow;
const log = (...str) => console.log(Math.round(now()).toString().padStart(5, "0"), ...str);

class Piece extends Array {
    rotate(num = 1) {
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
    }

    check() {
        for (let x = 0; x < 3; x++) {
            for (let y = 0; y < 3; y++) {
                if (x < 2) if (!this.checkPiece(x, y, x + 1, y)) return false;
                if (y < 2) if (!this.checkPiece(x, y, x, y + 1)) return false;
            }
        }
    }

    static solve() {
        const permutations = permutator(pieces);
        var i = 0;
        for(const permutation of permutations) {
            i++;
            const game = new Game(permutation);
            if(game.check()) {
                log("Got a HIT");
                log(game, game.pieces);
                return;
            }
            if(i % 10 === 0)
                log(`total ${permutations.length}, checked ${i}`);
        }
    }
}

Game.solve();