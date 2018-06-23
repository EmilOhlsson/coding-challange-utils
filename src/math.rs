use series::Primes;
use std::ops::*;

use num;

pub fn collatz<T>(n: T) -> T
where
    T: num::Integer + num::Unsigned + num::FromPrimitive,
{
    if n.is_even() {
        n / T::from_u32(2).unwrap()
    } else {
        n * T::from_u32(3).unwrap() + T::one()
    }
}

pub fn digitsum<T>(n: &T) -> T
where
    T: Add<Output = T>
        + Rem<usize, Output = T>
        + Div<usize, Output = T>
        + Ord
        + Clone
        + From<usize>,
{
    let mut v = n.clone();
    let mut sum: T = T::from(0);
    let zero = T::from(0);
    while &v > &zero {
        sum = sum + (v.clone() % 10);
        v = v / 10;
    }
    sum
}

pub fn faculty<T>(mut n: T) -> T
where
    for<'a> T: num::Integer + num::Unsigned + MulAssign<&'a T> + SubAssign<&'a T>,
{
    let one: T = T::one();
    let mut result: T = T::one();

    while n > one {
        result *= &n;
        n -= &one;
    }

    result
}

pub fn factorize<T>(mut n: T) -> Vec<usize>
where
    T: DivAssign<usize> + PartialEq<usize> + Rem<usize, Output = usize> + From<usize> + Copy,
{
    let primes: Primes<usize> = Primes::new();
    let mut factors = Vec::new();

    for p in primes {
        if n % p == 0 {
            factors.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        if n == 1 {
            break;
        }
    }

    factors
}

pub fn divisors(n: &usize) -> Vec<usize> {
    (2..(*n / 2 + 2))
        .filter(|d| *n % d == 0)
        .collect::<Vec<usize>>()
}

pub fn divisors_proper(n: &usize) -> Vec<usize> {
    (1..(*n / 2 + 1))
        .filter(|d| *n % d == 0)
        .collect::<Vec<usize>>()
}

#[test]
fn test_divisors_proper() {
    assert_eq!(divisors_proper(&1), vec![]);
    assert_eq!(divisors_proper(&2), vec![1]);
    assert_eq!(divisors_proper(&12), vec![1, 2, 3, 4, 6]);
}

pub fn is_perfect(n: &usize) -> bool {
    *n == divisors_proper(n).iter().sum::<usize>()
}
