pub fn textualize(n: usize) -> String {
    let low = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let pows = ["", "ten", "hundred", "thousand"];

    if n < 20 {
        String::from(low[n])
    } else if n < 100 {
        String::from(tens[n / 10]) + low[n % 10]
    } else if n < 1000 {
        String::from(low[n / 100])
            + pows[2]
            + &if n % 100 != 0 {
                "and".to_owned() + textualize(n % 100).as_str()
            } else {
                String::from("")
            }
    } else if n < 10000 {
        String::from(low[n / 1000]) + pows[3] + textualize(n % 1000).as_str()
    } else {
        panic!("Does not support numbers over 1000");
    }
}
