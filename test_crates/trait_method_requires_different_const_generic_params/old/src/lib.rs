pub trait Example {
    fn becomes_generic(data: [i64; 4]) -> [i64; 4];

    fn gains_const_generics<const N: usize>(data: [i64; N]) -> [i64; 4];

    fn loses_const_generics<const N: usize, const M: usize>(data: [i64; N]) -> [i64; M];
}
