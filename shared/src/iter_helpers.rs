use std::collections::HashSet;

pub trait CollectIter<T> {
    fn to_vec(self) -> Vec<T>;
    fn to_set(self) -> HashSet<T>
    where
        T: std::hash::Hash + std::cmp::Eq;
}

impl<T, I: Iterator<Item = T>> CollectIter<T> for I {
    fn to_vec(self) -> Vec<T> {
        self.collect::<Vec<_>>()
    }

    fn to_set(self) -> HashSet<T>
    where
        T: std::hash::Hash + std::cmp::Eq,
    {
        self.collect::<HashSet<_>>()
    }
}

pub trait IterExtensions<T> {
    fn has_duplicates(self) -> bool
    where
        T: std::hash::Hash + std::cmp::Eq;

    fn has_duplicates_by<F, T2>(self, f: F) -> bool
    where
        T: std::hash::Hash + std::cmp::Eq,
        T2: std::hash::Hash + std::cmp::Eq,
        F: Fn(&T) -> T2;

    fn has_matching_pair(self, f: impl Fn(&T, &T) -> bool) -> bool;

    fn get_matching_pair_values(self, f: impl Fn(&T, &T) -> bool) -> Vec<(T, T)>
    where
        T: Clone;

    fn count_by(self, f: impl Fn(&T) -> bool) -> usize;
}

impl<T, I: Iterator<Item = T>> IterExtensions<T> for I {
    fn has_duplicates(self) -> bool
    where
        T: std::hash::Hash + std::cmp::Eq,
    {
        let mut set = HashSet::new();

        for item in self {
            if set.contains(&item) {
                return true;
            } else {
                set.insert(item);
            }
        }

        false
    }

    fn has_duplicates_by<F, T2>(self, f: F) -> bool
    where
        T: std::hash::Hash + std::cmp::Eq,
        T2: std::hash::Hash + std::cmp::Eq,
        F: Fn(&T) -> T2,
    {
        let mut set = HashSet::new();

        for item in self {
            let key = f(&item);
            if set.contains(&key) {
                return true;
            } else {
                set.insert(key);
            }
        }

        false
    }

    fn has_matching_pair(self, f: impl Fn(&T, &T) -> bool) -> bool {
        let items = self.to_vec();

        for i in 0..items.len() {
            for j in 0..items.len() {
                if i != j {
                    if f(&items[i], &items[j]) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_matching_pair_values<'a>(self, f: impl Fn(&T, &T) -> bool) -> Vec<(T, T)>
    where
        T: Clone,
    {
        let items = self.to_vec();

        let mut pairs = Vec::new();

        for i in 0..items.len() {
            for j in 0..items.len() {
                if i != j {
                    if f(&items[i], &items[j]) {
                        pairs.push((items[i].clone(), items[j].clone()));
                    }
                }
            }
        }

        pairs
    }

    fn count_by(self, f: impl Fn(&T) -> bool) -> usize {
        let mut count = 0;

        for item in self {
            if f(&item) {
                count += 1;
            }
        }

        count
    }
}

pub trait VecExtensions<T> {
    fn max_val(&self) -> T
    where
        T: std::cmp::PartialOrd;

    fn min_val(&self) -> T
    where
        T: std::cmp::PartialOrd;
}

impl<T: Clone> VecExtensions<T> for Vec<T> {
    fn max_val(&self) -> T
    where
        T: std::cmp::PartialOrd,
    {
        self.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone()
    }

    fn min_val(&self) -> T
    where
        T: std::cmp::PartialOrd,
    {
        self.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
            .clone()
    }
}

pub trait IterSumHelper<T> {
    fn sum2(self) -> T;
}

impl<T, I: Iterator<Item = T>> IterSumHelper<T> for I
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    fn sum2(self) -> T {
        let mut sum = T::default();

        for item in self {
            sum = sum + item;
        }

        sum
    }
}
