use std::fmt::Debug;
use std::str::FromStr;

pub fn p<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    s.parse().unwrap()
}

pub fn ints_g<T, C: FromIterator<T>>(s: &str) -> C
where
    T: FromStr,
    T::Err: Debug,
{
    let signed = "-1".parse::<T>().is_ok();

    s.split(|c: char| !(c.is_numeric() || (signed && c == '-')))
        .filter(|s| !s.is_empty())
        .map(p)
        .collect()
}

pub fn ints<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    ints_g(s)
}

pub fn ints_n<T, const N: usize>(s: &str) -> [T; N]
where
    T: FromStr,
    T::Err: Debug,
{
    ints(s).try_into().ok().unwrap()
}

pub const INPUT: &str =
    include_str!(r#"\\wsl.localhost\Arch\home\the0x539\src\rust\aoc2023\day24\input.txt"#);

pub const TEST_INPUT: &str =
    include_str!(r#"\\wsl.localhost\Arch\home\the0x539\src\rust\aoc2023\day24\test.txt"#);
