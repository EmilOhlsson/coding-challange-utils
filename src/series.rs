use num;
use std::cmp::Ord;
use std::ops::*;

use constants;
use math::collatz;

pub struct Collatz<T> {
    n: T,
}

impl<T> Collatz<T>
where
    T: Copy
        + Rem<usize, Output = T>
        + Div<usize, Output = T>
        + Add<usize, Output = T>
        + Mul<usize, Output = T>
        + PartialEq<usize>,
{
    pub fn new(n: T) -> Collatz<T> {
        Collatz { n: n }
    }
}

impl<T> Iterator for Collatz<T>
where
    T: num::Integer + num::Unsigned + num::FromPrimitive + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let n = self.n;
        self.n = collatz(self.n);
        Some(n)
    }
}

pub struct Digits {
    n: usize,
}

impl Digits {
    pub fn new(n: usize) -> Digits {
        Digits { n: n }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if &self.n > &0 {
            let res = self.n % 10;
            self.n /= 10;
            return Some(res);
        } else {
            return None;
        }
    }
}

pub struct Fibonacci<T>
where
    T: Add<Output = T> + Clone,
{
    p0: T,
    p1: T,
}

impl<T> Fibonacci<T>
where
    T: Add<Output = T> + Clone,
{
    pub fn new(p0: T, p1: T) -> Fibonacci<T> {
        Fibonacci { p0: p0, p1: p1 }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output = T> + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let nxt = self.p0.clone() + self.p1.clone();
        self.p0 = self.p1.clone();
        self.p1 = nxt.clone();
        Some(nxt)
    }
}

pub struct Triangles<T>
where
    T: AddAssign<usize> + AddAssign<usize> + Clone,
{
    prev: T,
    i: usize,
}

impl<T> Triangles<T>
where
    T: AddAssign<usize> + From<usize> + Clone,
{
    pub fn new() -> Triangles<T> {
        Triangles {
            prev: T::from(0),
            i: 1,
        }
    }
}

impl<T> Iterator for Triangles<T>
where
    T: AddAssign<usize> + From<usize> + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.prev += self.i;
        self.i += 1;
        Some(self.prev.clone())
    }
}

pub struct Primes<T>
where
    T: Rem<Output = T> + Copy + From<usize>,
{
    produced: usize,
    primes: Vec<T>,
}

impl<T> Primes<T>
where
    T: Rem<Output = T> + Copy + From<usize>,
{
    pub fn new() -> Primes<T> {
        Primes {
            produced: 0,
            primes: constants::PRIMES
                .iter()
                .map(|&p| T::from(p))
                .collect::<Vec<_>>(),
        }
    }
}

impl<T> Iterator for Primes<T>
where
    T: Add<Output = T> + Rem<Output = T> + Div<Output = T> + Ord + Copy + From<usize>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result;
        if let Some(&p) = self.primes.get(self.produced) {
            result = Some(p);
        } else {
            let zero = T::from(0);
            let inc = T::from(2);
            let mut candidate = *self.primes.last().unwrap() + inc;
            while self.primes
                .iter()
                .take_while(|&p| *p < candidate / inc + inc)
                .find(|&p| candidate % *p == zero)
                .is_some()
            {
                candidate = candidate + inc;
            }
            self.primes.push(candidate);
            result = Some(candidate);
        }
        self.produced += 1;
        result
    }
}
