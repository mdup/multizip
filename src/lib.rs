use std::iter::Zip;

/*
 * zip2
 */
#[derive(Clone)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Zip2<A, B> {
    subzip: Zip<A, B>
}
impl<A, B> Iterator for Zip2<A, B> where
A: Iterator,
B: Iterator
{
    type Item = (A::Item, B::Item);

    #[inline]
    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        self.subzip.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.subzip.size_hint()
    }
}
pub fn zip2<U, V>(u: U, v: V) -> Zip2<U::IntoIter, V::IntoIter> where
    U: IntoIterator,
    V: IntoIterator,
{
    Zip2 {
        subzip: u.into_iter().zip(v.into_iter())
    }
}

/*
 * The generating macro is honestly hard to read, so if you feel like you really
 * need to dive into it, here is an example of what it generates. This is the
 * implementation of zip4.
 *
 * As you can see, it is based on a struct Zip4<A, B, C, D> which acts as the
 * Zip struct in Rust's stdlib. It recursively builds on the previous version,
 * (here: Zip3) and uses Zip to store both a Zip3 + another element, which makes
 * 4 of them in total.
 */
// #[derive(Clone)]
// #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
// pub struct Zip4<A, B, C, D> {
//     subzip: Zip<Zip3<A, B, C>, D>
// }
// impl<A, B, C, D> Iterator for Zip4<A, B, C, D> where
//     A: Iterator,
//     B: Iterator,
//     C: Iterator,
//     D: Iterator,
// {
//     type Item = (A::Item, B::Item, C::Item, D::Item);
//
//     #[inline]
//     fn next(&mut self) -> Option<(A::Item, B::Item, C::Item, D::Item)> {
//         self.subzip.next().map(|((a, b, c), d)| (a, b, c, d))
//     }
//
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.subzip.size_hint()
//     }
// }
//
// pub fn zip4<IA, IB, IC, ID>(ia: IA, ib: IB, ic: IC, id: ID)
//     -> Zip4<IA::IntoIter, IB::IntoIter, IC::IntoIter, ID::IntoIter> where
//     IA: IntoIterator,
//     IB: IntoIterator,
//     IC: IntoIterator,
//     ID: IntoIterator,
// {
//     Zip4 {
//         subzip: zip3(ia, ib, ic).zip(id.into_iter())
//     }
// }

// This macro will create the function `$zipn`, building on the previous stage
// `$zipnprev`. It needs a lot of arguments which are:
//   * $ZipN: the current ZipN struct
//   * $zipn: the current zipn function
//   * $ZipNPrev: the previous ZipN struct (NPrev == N-1)
//   * $zipnprev: the previous zipn function (nprev = n-1)
//   * $A: loops over A, B, C... type of the item zipped over
//   * $a: loops over a, b, c... name of a variable for the type A, B, C...
//   * $IA: loops over IA, IB, IC... type of the Iterator for A
//   * $ia: loops over ia, ib, ic... name of a variable for the type IA, IB, IC...
//   * $ALast: like $A but the last one. For example, it is D in Zip4<A, B, C, D>
//   * $alast: name of a variable for the type $ALast (ex: d if $ALast is D)
//   * $IALast: like $IA but the last one. For example, it is ID in Zip4.
//   * $ialast: name of a variable for the type $IALast (ex: id if $IALast is ID)
//
macro_rules! impl_zipn {
    ($ZipN:ident $zipn:ident $ZipNPrev:ident $zipnprev:ident ( $($A:ident $a:ident $IA:ident $ia:ident)+ ) $ALast:ident $alast:ident $IALast:ident $ialast:ident) => (
        #[derive(Clone)]
        #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
        pub struct $ZipN<$($A),*, $ALast> {
            subzip: Zip<$ZipNPrev<$($A),*>, $ALast>
        }
        impl<$($A),*, $ALast> Iterator for $ZipN<$($A),*, $ALast> where
            $($A: Iterator),*,
            $ALast: Iterator
        {
            type Item = ($($A::Item),*, $ALast::Item);

            #[inline]
            fn next(&mut self) -> Option<($($A::Item),*, $ALast::Item)> {
                self.subzip.next().map(|(( $($a),* ), $alast)| ($($a),*, $alast))
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.subzip.size_hint()
            }
        }
        pub fn $zipn<$($IA),*, $IALast>($($ia: $IA),*, $ialast: $IALast)
           -> $ZipN<$($IA::IntoIter),*, $IALast::IntoIter>
        where
            $($IA: IntoIterator),*,
            $IALast: IntoIterator
        {
            $ZipN {
                subzip: $zipnprev($($ia),*).zip($ialast.into_iter())
            }
        }
    );
}

impl_zipn!(Zip3 zip3 Zip2 zip2
           (A a IA ia
            B b IB ib)
           C c IC ic);
impl_zipn!(Zip4 zip4 Zip3 zip3
           (A a IA ia
            B b IB ib
            C c IC ic)
           D d ID id);
impl_zipn!(Zip5 zip5 Zip4 zip4
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id)
           E e IE ie);
impl_zipn!(Zip6 zip6 Zip5 zip5
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie)
           F f IF iff);
impl_zipn!(Zip7 zip7 Zip6 zip6
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff)
           G g IG ig);
impl_zipn!(Zip8 zip8 Zip7 zip7
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff
            G g IG ig)
           H h IH ih);
impl_zipn!(Zip9 zip9 Zip8 zip8
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff
            G g IG ig
            H h IH ih)
           I i II ii);
impl_zipn!(Zip10 zip10 Zip9 zip9
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff
            G g IG ig
            H h IH ih
            I i II ii)
           J j IJ ij);
impl_zipn!(Zip11 zip11 Zip10 zip10
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff
            G g IG ig
            H h IH ih
            I i II ii
            J j IJ ij)
           K k IK ik);
impl_zipn!(Zip12 zip12 Zip11 zip11
           (A a IA ia
            B b IB ib
            C c IC ic
            D d ID id
            E e IE ie
            F f IF iff
            G g IG ig
            H h IH ih
            I i II ii
            J j IJ ij
            K k IK ik)
           L l IL il);



#[cfg(test)]
mod tests {
    #![allow(non_upper_case_globals)]
    use super::{zip2, zip3, zip4, zip5, zip6, zip7, zip8, zip9, zip10, zip11,
                zip12};
    const a: [i8; 3] = [0, 1, 2];
    const b: [i8; 3] = [3, 4, 5];
    const c: [i8; 3] = [6, 7, 8];
    const d: [i8; 3] = [9, 10, 11];
    const e: [i8; 3] = [12, 13, 14];
    const f: [i8; 3] = [15, 16, 17];
    const g: [i8; 3] = [18, 19, 20];
    const h: [i8; 3] = [21, 22, 23];
    const i: [i8; 3] = [24, 25, 26];
    const j: [i8; 3] = [27, 28, 29];
    const k: [i8; 3] = [30, 31, 32];
    const l: [i8; 3] = [33, 34, 35];

    #[test]
    fn test_zip2() {
        let ab = zip2(a.iter(),
                      b.iter())
                 .map(|(&aa, &bb)| (aa, bb))
                 .collect::<Vec<_>>();
        assert_eq!(ab, vec![(0, 3),
                            (1, 4),
                            (2, 5)]);
    }

    #[test]
    fn test_zip3() {
        let abc = zip3(a.iter(),
                       b.iter(),
                       c.iter())
                 .map(|(&aa, &bb, &cc)| (aa, bb, cc))
                 .collect::<Vec<_>>();
        assert_eq!(abc, vec![(0, 3, 6),
                             (1, 4, 7),
                             (2, 5, 8)]);
    }

    #[test]
    fn test_zip4() {
        let abcd = zip4(a.iter(),
                        b.iter(),
                        c.iter(),
                        d.iter())
                 .map(|(&aa, &bb, &cc, &dd)| (aa, bb, cc, dd))
                 .collect::<Vec<_>>();
        assert_eq!(abcd, vec![(0, 3, 6, 9),
                              (1, 4, 7, 10),
                              (2, 5, 8, 11)]);
    }

    #[test]
    fn test_zip5() {
        let abcde = zip5(a.iter(),
                        b.iter(),
                        c.iter(),
                        d.iter(),
                        e.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee)| (aa, bb, cc, dd, ee))
                 .collect::<Vec<_>>();
        assert_eq!(abcde, vec![(0, 3, 6, 9,  12),
                               (1, 4, 7, 10, 13),
                               (2, 5, 8, 11, 14)]);
    }

    #[test]
    fn test_zip6() {
        let abcdef = zip6(a.iter(),
                          b.iter(),
                          c.iter(),
                          d.iter(),
                          e.iter(),
                          f.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff)| (aa, bb, cc, dd, ee, ff))
                 .collect::<Vec<_>>();
        assert_eq!(abcdef, vec![(0, 3, 6, 9,  12, 15),
                                (1, 4, 7, 10, 13, 16),
                                (2, 5, 8, 11, 14, 17)]);
    }

    #[test]
    fn test_zip7() {
        let abcdefg = zip7(a.iter(),
                           b.iter(),
                           c.iter(),
                           d.iter(),
                           e.iter(),
                           f.iter(),
                           g.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefg, vec![(0, 3, 6, 9,  12, 15, 18),
                                 (1, 4, 7, 10, 13, 16, 19),
                                 (2, 5, 8, 11, 14, 17, 20)]);
    }

    #[test]
    fn test_zip8() {
        let abcdefgh = zip8(a.iter(),
                            b.iter(),
                            c.iter(),
                            d.iter(),
                            e.iter(),
                            f.iter(),
                            g.iter(),
                            h.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg, &hh)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg,  hh))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefgh, vec![(0, 3, 6, 9,  12, 15, 18, 21),
                                   (1, 4, 7, 10, 13, 16, 19, 22),
                                   (2, 5, 8, 11, 14, 17, 20, 23)]);
    }

    #[test]
    fn test_zip9() {
        let abcdefghi = zip9(a.iter(),
                             b.iter(),
                             c.iter(),
                             d.iter(),
                             e.iter(),
                             f.iter(),
                             g.iter(),
                             h.iter(),
                             i.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg, &hh, &ii)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg,  hh,  ii))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefghi, vec![(0, 3, 6, 9,  12, 15, 18, 21, 24),
                                    (1, 4, 7, 10, 13, 16, 19, 22, 25),
                                    (2, 5, 8, 11, 14, 17, 20, 23, 26)]);
    }

    #[test]
    fn test_zip10() {
        let abcdefghij = zip10(a.iter(),
                              b.iter(),
                              c.iter(),
                              d.iter(),
                              e.iter(),
                              f.iter(),
                              g.iter(),
                              h.iter(),
                              i.iter(),
                              j.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg, &hh, &ii, &jj)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg,  hh,  ii,  jj))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefghij, vec![(0, 3, 6, 9,  12, 15, 18, 21, 24, 27),
                                     (1, 4, 7, 10, 13, 16, 19, 22, 25, 28),
                                     (2, 5, 8, 11, 14, 17, 20, 23, 26, 29)]);
    }

    #[test]
    fn test_zip11() {
        let abcdefghijk = zip11(a.iter(),
                                b.iter(),
                                c.iter(),
                                d.iter(),
                                e.iter(),
                                f.iter(),
                                g.iter(),
                                h.iter(),
                                i.iter(),
                                j.iter(),
                                k.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg, &hh, &ii, &jj, &kk)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg,  hh,  ii,  jj,  kk))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefghijk, vec![(0, 3, 6, 9,  12, 15, 18, 21, 24, 27, 30),
                                      (1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31),
                                      (2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32)]);
    }

    #[test]
    fn test_zip12() {
        let abcdefghijkl = zip12(a.iter(),
                                 b.iter(),
                                 c.iter(),
                                 d.iter(),
                                 e.iter(),
                                 f.iter(),
                                 g.iter(),
                                 h.iter(),
                                 i.iter(),
                                 j.iter(),
                                 k.iter(),
                                 l.iter())
                 .map(|(&aa, &bb, &cc, &dd, &ee, &ff, &gg, &hh, &ii, &jj, &kk, &ll)|
                       ( aa,  bb,  cc,  dd,  ee,  ff,  gg,  hh,  ii,  jj,  kk,  ll))
                 .collect::<Vec<_>>();
        assert_eq!(abcdefghijkl, vec![(0, 3, 6, 9,  12, 15, 18, 21, 24, 27, 30, 33),
                                      (1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34),
                                      (2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35)]);
    }
}
