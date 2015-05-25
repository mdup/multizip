# multizip
Zip 2, 3, 4 or more Rust iterators

----

## Description

With Rust's stdlib you can only zip 2 iterators at a time:
```
    let a: Vec<i8> = vec![0, 1, 2];
    let b: Vec<i8> = vec![3, 4, 5];
    let c: Vec<i8> = vec![6, 7, 8];
    let d: Vec<i8> = vec![9, 10, 11];

    let abc = a.iter().zip(b.iter()).zip(c.iter())
               .map(|((&aa, &bb), &cc)| aa+bb+cc);
    //               ((   ,    ),    )
    //               ^~~~~~~~~~^~~~~~^ awkward!

    let abcd = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter())
               .map(|(((&aa, &bb), &cc), &dd)| aa+bb+cc+dd);
    //               (((   ,    ),    ),    )
    //               ^~~~~~~~~~~^~~~~~^~~~~~^ ughhh!
```

With `multizip`, you get a flattened version of `zip`:
```
    let abc = multizip::zip3(a.iter(),
                             b.iter(),
                             c.iter())
             .map(|(&aa, &bb, &cc)| aa+bb+cc))
    //             (   ,    ,    )
    //             ^~~~~~~~~~~~~~^ oooh!

    let abcd = multizip::zip4(a.iter(),
                              b.iter(),
                              c.iter(),
                              d.iter())
             .map(|(&aa, &bb, &cc, &dd)| aa+bb+cc+dd)
    //             (   ,    ,    ,    )
    //             ^~~~~~~~~~~~~~~~~~~^ sweet!
```

## How to use
TODO: upload to crates.io and update here with Cargo instructions.

## FAQ
### Up to how much variables can I zip together?
Rust supports up to 12 variables in a single tuple, so the following are
implemented: `zip2()`, `zip3()`, `zip4()`..., `zip12()`.

If you need more than 12, something is probably wrong with your design. Consider
something more appropriate than tuples.

### What is the advantage of `zip2()` over `std::iter::zip()`?
The only advantage is the symmetry of arguments, e.g. `zip2(a.iter(),
b.iter())` over `a.iter().zip(b.iter())`.

## Author
Marc Dupont -- [mdup.fr](http://mdup.fr)
