import * as fs from 'node:fs/promises';

const makeRange = function* (total = 0, from = 0, step = 1) {
    for (let i = 0; i < total; yield from + i++ * step) {}
};

class Vec2 {
    x: number = 0;
    y: number = 0;

    constructor(x = 0, y = 0) {
        this.x = x;
        this.y = y;
    }

    add(o: Vec2) {
        return new Vec2(this.x + o.x, this.y + o.y);
    }
    lower_any(o: Vec2) {
        return this.x < o.x || this.y < o.y;
    }
    greater_any(o: Vec2) {
        return this.x > o.x || this.y > o.y;
    }
}

class GameState {
    size: number = 5;
    state: number = 0;

    set(pos: Vec2, val: boolean) {
        const bit = pos.y * this.size + pos.x;
        if (val) {
            this.state |= (1 << bit);
        } else {
            this.state &= ~(1 << bit);
        }
    }
    get(pos: Vec2): boolean {
        const bit = pos.y * this.size + pos.x;
        return (Boolean)((this.state >> bit) & 1);
    }
    count_adjacent(pos: Vec2): number {
        const dirs = [
            new Vec2( 0, 1),
            new Vec2( 0,-1),
            new Vec2( 1, 0),
            new Vec2(-1, 0),
        ];
        const is_inside = (x: Vec2) => {
            return !x.lower_any(new Vec2(0, 0))
                && !x.greater_any(new Vec2(this.size - 1, this.size - 1));
        }
        let sum = 0;
        for (const dir of dirs) {
            const np = pos.add(dir);
            if (is_inside(np)) {
                sum += this.get(np) ? 1 : 0;
            }
        }
        return sum;
    }
    serialize(): number {
        return this.state;
    }

    debug() {
        let mask = 0;
        let state = this.state;
        for (const i of makeRange(this.size)) {
            mask = (mask << 1) | 1;
        }
        let s = '';
        for (const y of makeRange(this.size)) {
            for (const x of makeRange(this.size)) {
                s += (state & 1) ? '#' : '.';
                state >>= 1;
            }
            s += '\n';
        }
        console.log(s);
    }
}

(async () => {
    let state = new GameState();
    const inputFile = await fs.open("aoc24/input.txt");
    let y = 0;
    for await (const line of inputFile.readLines()) {
        let x = 0;
        for (const ch of line) {
            if (ch == '#') {
                state.set(new Vec2(x, y), true);
            }
            x++;
        }
        y++;
    }
    state.debug();
    let past_states = new Set([state.serialize()]);
    while (true) {
        let new_state = new GameState();
        for (const y of makeRange(state.size)) {
            for (const x of makeRange(state.size)) {
                const pos = new Vec2(x, y);
                const adjacent = state.count_adjacent(pos);
                const existing = state.get(pos);
                const next_bug = existing
                    ? adjacent === 1
                    : adjacent === 1 || adjacent === 2;
                new_state.set(pos, next_bug);
            }
        }
        state = new_state;
        let v = state.serialize();
        if (past_states.has(v)) {
            console.log(`Found reoccuring`);
            break;
        } else {
            past_states.add(v);
        }
    }
    state.debug();
    console.log(state.state);
})();