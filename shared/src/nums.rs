pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

impl One for u64 {
    fn one() -> Self {
        1
    }
}

impl One for usize {
    fn one() -> Self {
        1
    }
}

impl One for i64 {
    fn one() -> Self {
        1
    }
}

impl One for i128 {
    fn one() -> Self {
        1
    }
}

impl One for u128 {
    fn one() -> Self {
        1
    }
}

impl One for isize {
    fn one() -> Self {
        1
    }
}

pub trait ToUnsignedIndex {
    fn to_index(self) -> Option<usize>;
}

impl ToUnsignedIndex for i32 {
    fn to_index(self) -> Option<usize> {
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    }
}

impl ToUnsignedIndex for u32 {
    fn to_index(self) -> Option<usize> {
        Some(self as usize)
    }
}

impl ToUnsignedIndex for u64 {
    fn to_index(self) -> Option<usize> {
        Some(self as usize)
    }
}

impl ToUnsignedIndex for usize {
    fn to_index(self) -> Option<usize> {
        Some(self)
    }
}

impl ToUnsignedIndex for i64 {
    fn to_index(self) -> Option<usize> {
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    }
}

impl ToUnsignedIndex for i128 {
    fn to_index(self) -> Option<usize> {
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    }
}

impl ToUnsignedIndex for u128 {
    fn to_index(self) -> Option<usize> {
        Some(self as usize)
    }
}

impl ToUnsignedIndex for isize {
    fn to_index(self) -> Option<usize> {
        if self < 0 {
            None
        } else {
            Some(self as usize)
        }
    }
}

pub trait ToSignedIndex {
    fn to_index(self) -> isize;
}

impl ToSignedIndex for i32 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for u32 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for u64 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for usize {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for i64 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for i128 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for u128 {
    fn to_index(self) -> isize {
        self as isize
    }
}

impl ToSignedIndex for isize {
    fn to_index(self) -> isize {
        self
    }
}
