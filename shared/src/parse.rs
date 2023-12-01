use std::str::FromStr;

pub fn parse_lines_words(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|l| {
            l.split(|c| c == ' ' || c == '\t')
                .filter(|w| !w.is_empty())
                .map(|w| w.trim())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn parse_lines_items<T: FromStr>(input: &str) -> Vec<Vec<T>>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|l| {
            l.split(|c| c == ' ' || c == '\t')
                .filter(|w| !w.is_empty())
                .map(|w| w.trim().parse::<T>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn parse_lines_items_int(input: &str) -> Vec<Vec<i64>> {
    parse_lines_items::<i64>(input)
}

pub fn parse_lines_items_float(input: &str) -> Vec<Vec<f64>> {
    parse_lines_items::<f64>(input)
}

pub fn parse_lines<T: FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|l| l.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}
