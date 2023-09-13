use crate::engine::Fin;
use crate::{ops::GeneratorState, stack::generator::Gen};
use core::future::Future;

impl<'s, Y, F: Future<Output = Fin>> IntoIterator for Gen<'s, Y, (), F> {
    type Item = Y;
    type IntoIter = IntoIter<'s, Y, F>;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { generator: self }
    }
}

pub struct IntoIter<'s, Y, F: Future<Output = Fin>> {
    generator: Gen<'s, Y, (), F>,
}

impl<'s, Y, F: Future<Output = Fin>> Iterator for IntoIter<'s, Y, F> {
    type Item = Y;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(()) => None,
        }
    }
}

impl<'r, 's, Y, F: Future<Output = Fin>> IntoIterator for &'r mut Gen<'s, Y, (), F> {
    type Item = Y;
    type IntoIter = MutIntoIter<'r, 's, Y, F>;

    fn into_iter(self) -> Self::IntoIter {
        MutIntoIter { generator: self }
    }
}

pub struct MutIntoIter<'r, 's, Y, F: Future<Output = Fin>> {
    generator: &'r mut Gen<'s, Y, (), F>,
}

impl<'r, 's, Y, F: Future<Output = Fin>> Iterator for MutIntoIter<'r, 's, Y, F> {
    type Item = Y;

    fn next(&mut self) -> Option<Self::Item> {
        match self.generator.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(()) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::Fin;
    use crate::stack::{Co, Gen, Shelf};
    #[cfg(feature = "macro")]
    use crate::stack::let_gen_using;
    use core::iter::IntoIterator;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    async fn produce(mut co: Co<'_, i32>) -> Fin {
        co.yield_(10).await;
        co.yield_(20).await;
        co.return_(())
    }

    #[cfg(all(feature = "macro", feature = "alloc"))]
    #[test]
    fn let_gen_using_into_iter() {
        let_gen_using!(gen, produce);

        let items: Vec<_> = gen.into_iter().collect();
        assert_eq!(items, [10, 20]);
    }

    #[cfg(feature = "macro")]
    #[test]
    fn let_gen_using_for_loop() {
        let_gen_using!(gen, produce);

        let mut sum = 0;
        for x in gen {
            sum += x;
        }
        assert_eq!(sum, 30);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn shelf_generator_into_iter() {
        let mut shelf = Shelf::new();
        let gen = Gen::new(&mut shelf, produce);

        let items: Vec<_> = gen.into_iter().collect();
        assert_eq!(items, [10, 20]);
    }

    #[test]
    fn shelf_generator_for_loop() {
        let mut shelf = Shelf::new();
        let gen = Gen::new(&mut shelf, produce);

        let mut sum = 0;
        for x in gen {
            sum += x;
        }
        assert_eq!(sum, 30);
    }
}
