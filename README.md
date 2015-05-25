# multizip
Zip 3, 4, 5 or more Rust iterators

----

## Description

With Rust's stdlib you can only zip 2 iterators at a time, which causes nesting:
```rust
let a: Vec<i8> = vec![0, 1, 2];
let b: Vec<i8> = vec![3, 4, 5];
let c: Vec<i8> = vec![6, 7, 8];
let d: Vec<i8> = vec![9, 10, 11];

let abc: Vec<i8> = a.iter().zip(b.iter()).zip(c.iter())
                    .map(|((&aa, &bb), &cc)| aa+bb+cc).collect();
//                        ((   ,    ),    )
//                        ^~~~~~~~~~^~~~~~^ awkward!

let abcd: Vec<i8> = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter())
                     .map(|(((&aa, &bb), &cc), &dd)| aa+bb+cc+dd).collect();
//                         (((   ,    ),    ),    )
//                         ^~~~~~~~~~~^~~~~~^~~~~~^ ughhh!
```

With `multizip`, you get a flattened version of `zip`:
```rust
let abc: Vec<i8> = multizip::zip3(a.iter(), b.iter(), c.iter())
                   .map(|(&aa, &bb, &cc)| aa+bb+cc)).collect();
//                       (   ,    ,    )
//                       ^~~~~~~~~~~~~~^ oooh!

let abcd: Vec<i8> = multizip::zip4(a.iter(), b.iter(), c.iter(), d.iter())
                    .map(|(&aa, &bb, &cc, &dd)| aa+bb+cc+dd).collect();
//                        (   ,    ,    ,    )
//                        ^~~~~~~~~~~~~~~~~~~^ sweet!
```

## How to use
TODO: upload to crates.io and update here with Cargo instructions.

## FAQ
### How much variables can I zip together?
Rust supports up to 12 variables in a single tuple, so the following are
implemented: `zip2()`, `zip3()`, `zip4()`..., `zip12()`.

If you need more than 12, something is probably wrong with your design. Consider
something more appropriate than tuples.

### What is the advantage of `multizip::zip2()` over `std::iter::zip()`?
The only advantage is the symmetry of arguments, e.g. `zip2(a.iter(),
b.iter())` over `a.iter().zip(b.iter())`.

## Author
Marc Dupont -- [mdup.fr](http://mdup.fr)
