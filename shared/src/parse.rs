use std::str::FromStr;

use crate::Grid2;

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

pub fn parse_grid2<T: FromStr>(input: &str) -> Grid2<T>
where
    T::Err: std::fmt::Debug,
{
    let parsed = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<T>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    parsed.into()
}

pub trait SplitHelpers {
    fn split_at_char(&self, c: char) -> (&str, &str);
    fn split_at_str<'a>(&'a self, c: &str) -> (&'a str, &'a str);
    fn split_at_whitespace(&self) -> (&str, &str);
}

impl<T: AsRef<str>> SplitHelpers for T {
    fn split_at_char(&self, c: char) -> (&str, &str) {
        let s = self.as_ref();
        let mut iter = s.split(c);
        let first = iter.next();
        let second = iter.next();

        match (first, second) {
            (Some(first), Some(second)) => (first, second),
            _ => panic!("split_at_char failed"),
        }
    }

    fn split_at_str<'a>(&'a self, pat: &str) -> (&'a str, &'a str) {
        let s = self.as_ref();
        let mut iter = s.split(pat);
        let first = iter.next();
        let second = iter.next();

        match (first, second) {
            (Some(first), Some(second)) => (first, second),
            _ => panic!("split_at_str failed"),
        }
    }

    fn split_at_whitespace(&self) -> (&str, &str) {
        let s = self.as_ref();
        let mut iter = s.split_whitespace();
        let first = iter.next();
        let second = iter.next();

        match (first, second) {
            (Some(first), Some(second)) => (first, second),
            _ => panic!("split_at_whitespace failed"),
        }
    }
}
