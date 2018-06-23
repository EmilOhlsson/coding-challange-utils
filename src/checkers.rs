use std::fmt::Display;

pub fn is_palindrome<T>(v: &T) -> bool
where
    T: Display,
{
    let a = format!("{}", v);
    let b = a.chars().rev().collect::<String>();
    a == b
}
