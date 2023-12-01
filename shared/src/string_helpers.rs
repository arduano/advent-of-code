pub trait StringCharHelpers {
    fn sort_chars(&self) -> String;
}

impl StringCharHelpers for str {
    fn sort_chars(&self) -> String {
        let mut chars = self.chars().collect::<Vec<_>>();
        chars.sort();
        chars.into_iter().collect()
    }
}

impl StringCharHelpers for String {
    fn sort_chars(&self) -> String {
        self.as_str().sort_chars()
    }
}
