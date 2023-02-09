use std::cmp::PartialOrd;
use std::ops::Mul;
use std::ops::Sub;

pub struct GridCount {
    points: Vec<Vec<u64>>,
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

    fn to_flag_index<T>(y: T) -> (usize, u64)
    where
        T: PartialOrd<i32>,
        <T as Mul<i32>>::Output: Sub<i32>,
        T: Mul<i32>,
        usize: TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>,
        usize: TryFrom<<T as Mul<i32>>::Output>,
        <usize as TryFrom<<<T as Mul<i32>>::Output as Sub<i32>>::Output>>::Error: std::fmt::Debug,
        <usize as TryFrom<<T as Mul<i32>>::Output>>::Error: std::fmt::Debug,
    {
        let i = Self::to_index(y);
        (i / 64, 1 << (i % 64))
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
        let (iy, mask) = Self::to_flag_index(y);
        if ix >= self.points.len() {
            self.points
                .resize((ix + 1) * 4, Vec::with_capacity((iy + 1) * 10));
        }
        if iy >= self.points[ix].len() {
            self.points[ix].resize((iy + 1) * 4, 0);
        }

        if self.points[ix][iy] & mask == 0 {
            self.len += 1;
        }
        self.points[ix][iy] |= mask;
    }
}

#[derive(Clone)]
pub struct SignedVec<T> {
    pos: Vec<T>,
    neg: Vec<T>,
}

impl<T> Default for SignedVec<T>
where
    Vec<T>: Default,
{
    fn default() -> Self {
        Self {
            pos: Default::default(),
            neg: Default::default(),
        }
    }
}

impl<T> SignedVec<T> {
    #[allow(unused)]
    pub const fn new() -> Self {
        Self {
            pos: Vec::new(),
            neg: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos.is_empty() && self.neg.is_empty()
    }

    pub fn get(&self, i: isize) -> Option<&T> {
        if i < 0 {
            self.neg.get(i.abs_diff(-1))
        } else {
            self.pos.get(i.abs_diff(0))
        }
    }

    #[allow(unused)]
    pub fn get_mut(&mut self, i: isize) -> Option<&mut T> {
        if i < 0 {
            self.neg.get_mut(i.abs_diff(-1))
        } else {
            self.pos.get_mut(i.abs_diff(0))
        }
    }
}

impl<T> SignedVec<T>
where
    T: Clone,
{
    #[allow(unused)]
    pub fn get_mut_or(&mut self, i: isize, v: T) -> &mut T {
        let (vec, ii) = if i < 0 {
            (&mut self.neg, i.abs_diff(-1))
        } else {
            (&mut self.pos, i.abs_diff(0))
        };

        if ii >= vec.len() {
            vec.resize((ii + 1) * 2, v);
        }

        &mut vec[ii]
    }
}

impl<T> SignedVec<T>
where
    T: Default,
{
    pub fn get_mut_or_default(&mut self, i: isize) -> &mut T {
        let (vec, ii) = if i < 0 {
            (&mut self.neg, i.abs_diff(-1))
        } else {
            (&mut self.pos, i.abs_diff(0))
        };

        if ii >= vec.len() {
            vec.resize_with((ii + 1) * 2, Default::default);
        }

        &mut vec[ii]
    }
}
