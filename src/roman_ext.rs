#[rustfmt::skip]
const NUMERALS: [(usize, [&str; 10]); 10] = [
    (1_000_000_000, ["", "M̅̅", "M̅̅M̅̅", "M̅̅M̅̅M̅̅", "--", "-", "--", "---", "----", "--"]),
    (  100_000_000, ["", "C̅̅", "C̅̅C̅̅", "C̅̅C̅̅C̅̅", "C̅̅D̅̅", "D̅̅", "D̅̅C̅̅", "D̅̅C̅̅C̅̅", "D̅̅C̅̅C̅̅C̅̅", "C̅̅M̅̅"]),
    (   10_000_000, ["", "X̅̅", "X̅̅X̅̅", "X̅̅X̅̅X̅̅", "X̅̅L̅̅", "L̅̅", "L̅̅X̅̅", "L̅̅X̅̅X̅̅", "L̅̅X̅̅X̅̅X̅̅", "X̅̅C̅̅"]),
    (    1_000_000, ["", "M̅", "M̅M̅", "M̅M̅M̅", "M̅V̅̅", "V̅̅", "V̅̅M̅", "V̅̅M̅M̅", "V̅̅M̅M̅M̅", "M̅X̅̅"]),
    (      100_000, ["", "C̅", "C̅C̅", "C̅C̅C̅", "C̅D̅", "D̅", "D̅C̅", "D̅C̅C̅", "D̅C̅C̅C̅", "C̅M̅"]),
    (       10_000, ["", "X̅", "X̅X̅", "X̅X̅X̅", "X̅L̅", "L̅", "L̅X̅", "L̅X̅X̅", "L̅X̅X̅X̅", "X̅C̅"]),
    (        1_000, ["", "M", "MM", "MMM", "MV̅", "V̅", "V̅M", "V̅MM", "V̅MMM", "MX̅"]),
    (          100, ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]),
    (           10, ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]),
    (            1, ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]),
];

pub const MAX: usize = 3_999_999_999;

pub fn from(n: usize) -> Option<String> {
    if n == 0 || n > MAX {
        return None;
    }
    Some(
        NUMERALS
            .iter()
            .map(|&(base, nums)| nums[(n / base) % 10])
            .collect(),
    )
}
