extern crate multizip;

use multizip::zip3;

fn main() {
    let a: Vec<i8> = vec![0, 1, 2];
    let b: Vec<i8> = vec![3, 4, 5];
    let c: Vec<i8> = vec![6, 7, 8];
    for (aa, bb, cc) in zip3(a, b, c) {
        println!("aa={}, bb={}, cc={}", aa, bb, cc);
    }
}

