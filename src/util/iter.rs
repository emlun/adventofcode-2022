use std::collections::HashMap;

pub struct Sliding2<I, T> {
    buffer: Option<T>,
    iter: I,
}

impl<I, T> Iterator for Sliding2<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.buffer.and_then(|buffered| {
            self.iter.next().map(|next| {
                self.buffer = Some(next);
                (buffered, next)
            })
        })
    }
}

pub trait WithSliding
where
    Self: Iterator,
    Self: Sized,
{
    fn sliding2(mut self) -> Sliding2<Self, Self::Item> {
        Sliding2 {
            buffer: self.next(),
            iter: self,
        }
    }
}

impl<I> WithSliding for I where I: Iterator {}

pub trait Countable<A> {
    fn counts(self) -> HashMap<A, usize>;
}

impl<A, I> Countable<A> for I
where
    A: Eq,
    A: std::hash::Hash,
    I: Iterator<Item = A>,
{
    fn counts(self) -> HashMap<A, usize> {
        self.fold(HashMap::new(), |mut result, item| {
            result.entry(item).and_modify(|c| *c += 1).or_insert(1);
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Countable;
    use std::collections::HashMap;

    #[test]
    fn count_empty_is_empty() {
        assert_eq!(Vec::<i32>::new().into_iter().counts(), HashMap::new());
    }

    #[test]
    fn count_one_is_one() {
        assert_eq!(
            vec![0].into_iter().counts(),
            vec![(0, 1)].into_iter().collect()
        );
    }

    #[test]
    fn count_many_is_different() {
        assert_eq!(
            vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 6]
                .into_iter()
                .counts(),
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1), (6, 1)]
                .into_iter()
                .collect()
        );
    }
}
