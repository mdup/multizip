extern crate multizip;

use multizip::{zip3, zip4};

fn main() {
    let a: Vec<i8> = vec![0, 1, 2];
    let b: Vec<i8> = vec![3, 4, 5];
    let c: Vec<i8> = vec![6, 7, 8];
    let d: Vec<i8> = vec![9, 10, 11];

    // With 3 elements:
    let abc1: Vec<i8> = a.iter().zip(b.iter()).zip(c.iter())
                        .map(|((&aa, &bb), &cc)| aa+bb+cc)
                        .collect();
    let abc2: Vec<i8> = zip3(a.iter(), b.iter(), c.iter())
                       .map(|(&aa, &bb, &cc)| aa+bb+cc)
                       .collect();
    println!("a+b+c = {:?} + {:?} + {:?}", a, b, c);
    println!("      = {:?} with std::iter::Iterator::zip()", abc1);
    println!("      = {:?} with multizip::zip3()", abc2);

    // With 4 elements:
    let abcd1: Vec<i8> = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter())
                         .map(|(((&aa, &bb), &cc), &dd)| aa+bb+cc+dd)
                         .collect();
    let abcd2: Vec<i8> = zip4(a.iter(), b.iter(), c.iter(), d.iter())
                        .map(|(&aa, &bb, &cc, &dd)| aa+bb+cc+dd)
                        .collect();
    println!("a+b+c+d = {:?} + {:?} + {:?} + {:?} =", a, b, c, d);
    println!("        = {:?} with std::iter::Iterator::zip()", abcd1);
    println!("        = {:?} with multizip::zip3()", abcd2);

    // In a for loop:
    for (aa, bb, cc) in zip3(a, b, c) {
        println!("aa={}, bb={}, cc={}", aa, bb, cc);
    }
}
