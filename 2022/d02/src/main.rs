use std::fs::read_to_string;

fn pcs(x: char) -> u32 {
    match x {
        'A' => 1, // rock
        'B' => 2, // paper
        'C' => 3, // scissor
        'X' => 1, // rock
        'Y' => 2, // paper
        'Z' => 3, // scissor
        _ => 16,
    }
}

fn part1() {
    let mut res: u32 = 0;
    for line in read_to_string("src/01.txt").unwrap().lines() {
        let left = line.chars().nth(0).unwrap();
        let right = line.chars().nth(2).unwrap();

        if pcs(left) == pcs(right) {
            res += pcs(right) + 3;
        } else if left == 'A' && right == 'Z' {
            res += pcs(right) + 0;
        } else if left == 'B' && right == 'X' {
            res += pcs(right) + 0;
        } else if left == 'C' && right == 'Y' {
            res += pcs(right) + 0;
        } else {
            res += pcs(right) + 6;
        }
    }
    print!("{}\n", res);
}

fn part2() {
    let mut res : u32 = 0;
    for line in read_to_string("src/02.txt").unwrap().lines() {
        let left = line.chars().nth(0).unwrap();
        let right = line.chars().nth(2).unwrap();

        // lose
        if right == 'X' {
            let right_lose = match left {
                'A' => 'Z',
                'B' => 'X',
                'C' => 'Y',
                _ => 'L' // don't know what to do ! :)
            };
            res += pcs(right_lose);
        }
        // draw
        else if right == 'Y' {
            let right_draw = match left {
                'A' => 'X',
                'B' => 'Y',
                'C' => 'Z',
                _ => 'L' // don't know what to do ! :)

            };
            res += pcs(right_draw);
        }
        // win
        else {
            let right_win = match left {
                'A' => 'Y',
                'B' => 'Z',
                'C' => 'X',
                _ => 'L' // don't know what to do ! :)

            };
            res += pcs(right_win);
        }
    }
    print!("{}\n", res);
}

fn main() {
    part1();
    part2();
}
