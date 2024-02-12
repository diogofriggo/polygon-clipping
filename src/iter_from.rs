use std::{collections::VecDeque, iter::Peekable};

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer_iterator: O,
    inner_iterator: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iterator) = self.inner_iterator {
                if let Some(item) = inner_iterator.next() {
                    return Some(item);
                } else {
                    self.inner_iterator = None;
                }
            } else if let Some(inner_iterator) = self.outer_iterator.next() {
                self.inner_iterator = Some(inner_iterator.into_iter());
            } else {
                return None;
            }
        }
    }
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(outer: O) -> Self {
        Self {
            outer_iterator: outer,
            inner_iterator: None,
        }
    }
}

pub trait IteratorExt: Iterator {
    // Flatten<"Self"> is not clear to me
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized,
    {
        Flatten::new(self)
    }
}

impl<O> IteratorExt for O
where
    O: Iterator,
    O::Item: IntoIterator,
{
}

pub struct IterFrom<T>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    skipped_items: VecDeque<T::Item>,
    peekable_iterator: Peekable<<T as IntoIterator>::IntoIter>,
}

impl<T> IterFrom<T>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    pub fn new(items: T, from_item: T::Item) -> Self {
        let mut skipped_items = VecDeque::new();
        let mut peekable_iterator = items.into_iter().peekable();
        while let Some(item) = peekable_iterator.peek() {
            if item == &from_item {
                break;
            } else {
                skipped_items.push_back(peekable_iterator.next().unwrap());
            }
        }

        Self {
            skipped_items,
            peekable_iterator,
        }
    }
}

pub trait IteratorFrom: Iterator {
    fn from(self, from_item: Self::Item) -> IterFrom<Self>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        IterFrom::new(self, from_item)
    }
}

impl<T> IteratorFrom for T where T: Iterator {}

impl<T> Iterator for IterFrom<T>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.peekable_iterator
            .next()
            .or_else(|| self.skipped_items.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_from() {
        let items = [0, 1, 2];
        let actual_items = items.iter().from(&1);
        let expected_items = [1, 2, 0];
        for (actual, expected) in actual_items.zip(expected_items.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_flatten() {
        let outer = vec![vec![1, 2], vec![3, 4]];
        let actual = outer.into_iter().our_flatten().collect::<Vec<_>>();
        let expected = vec![1, 2, 3, 4];
        assert_eq!(actual, expected);
    }
}
