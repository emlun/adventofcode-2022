use std::cmp::PartialOrd;
use std::ops::Mul;
use std::ops::Sub;

pub struct GridCount {
    points: Vec<Vec<bool>>,
    len: usize,
}

impl GridCount {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            len: 0,
        }
    }

    fn to_index<T>(x: T) -> usize
    where
        T: PartialOrd<i32>,
        <T as Mul<i32>>::Output: Sub<i32>,
        T: Mul<i32>,
        usize: TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>,
        usize: TryFrom<<T as Mul<i32>>::Output>,
        <usize as TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>>::Error: std::fmt::Debug,
        <usize as TryFrom<<T as Mul<i32>>::Output>>::Error: std::fmt::Debug,
    {
        if x < 0 {
            usize::try_from(x * (-2) - 1).unwrap()
        } else {
            usize::try_from(x * 2).unwrap()
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert<T>(&mut self, (x, y): (T, T))
    where
        T: PartialOrd<i32>,
        <T as Mul<i32>>::Output: Sub<i32>,
        T: Mul<i32>,
        usize: TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>,
        usize: TryFrom<<T as Mul<i32>>::Output>,
        <usize as TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>>::Error: std::fmt::Debug,
        <usize as TryFrom<<T as Mul<i32>>::Output>>::Error: std::fmt::Debug,
    {
        let ix = Self::to_index(x);
        let iy = Self::to_index(y);
        if ix >= self.points.len() {
            self.points
                .resize((ix + 1) * 2, Vec::with_capacity((iy + 1) * 2));
        }
        if iy >= self.points[ix].len() {
            self.points[ix].resize((iy + 1) * 2, false);
        }

        if !self.points[ix][iy] {
            self.len += 1;
        }
        self.points[ix][iy] = true;
    }
}
