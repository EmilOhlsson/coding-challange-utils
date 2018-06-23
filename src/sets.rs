fn premutation_generate<T>(out: &mut Vec<Vec<T>>, n: usize, vec: &mut Vec<T>)
where
    T: Copy,
{
    if n == 1 {
        out.push(vec.clone());
    } else {
        for i in 0..(n - 1) {
            premutation_generate(out, n - 1, vec);
            if n % 2 == 0 {
                vec.swap(i, n - 1);
            } else {
                vec.swap(0, n - 1);
            }
        }
        premutation_generate(out, n - 1, vec);
    }
}

pub fn permutations<T>(set: &mut Vec<T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut out = Vec::new();
    premutation_generate(&mut out, set.len(), set);
    return out;
}
