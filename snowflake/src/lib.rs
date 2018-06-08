// https://joshleeb.com/posts/rust-wasm-snowhash/
extern crate rand;

mod bit;
mod bitstr;
pub mod point;
pub mod draw;

use point::{ Point };
use rand::{ Rng, SeedableRng, StdRng };
use bitstr::BitStr;

pub fn hash_sum(hash: &str) -> usize {
    hash.as_bytes().into_iter().fold(0, |acc, byte| acc + *byte as usize)
}

pub fn extend(point: &Point, closed: &Vec<Point>) -> Vec<Point> {
    point.neighbours()
        .into_iter()
        .filter(|p| in_slice(p) && !closed.contains(p))
        .collect()
}

pub fn in_slice(point: &Point) -> bool {
    point.x() >= point.y() && point.y() >= 0
}

pub fn generate(hash: &str) -> Vec<Point> {
    let seed: &[_] = &[hash_sum(hash)];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut open = vec![Point::origin()];
    let mut closed = vec![];
    let mut filled = vec![];

    for bit in BitStr::from_str(hash) {
        let index = rng.gen_range(0, open.len());
        let point = open.remove(index);

        if point.on_axis() || bit.as_bool() {
            let mut reflection = point.reflection();
            filled.append(&mut reflection);

            let mut extension = extend(&point, &closed);
            open.append(&mut extension);
        }

        closed.push(point);
    }

    filled
}

pub fn create_hash(len: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(len).collect()
}
