use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let mut cpt = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let x = line.to_string();
        if x.is_empty() {
            result.push(cpt);
            cpt = 0;
        } else {
            let v : u32 = x.parse().unwrap();
            cpt += v;
        }
        print!("{} {}\n", x, x.is_empty());
    }
    result.push(cpt);
    return result;
}

fn main() {
    let mut res = read_lines("input.01");
    res.sort();
    res.reverse();
    for x in &res {
        print!("{}\n", x);
    }
    let out = res[0] + res[1] + res[2];
    print!("result = {}\n", out);
}