#![feature(min_type_alias_impl_trait)]

use rand::seq::SliceRandom;
use rand::RngCore;
use std::iter::FromIterator;
use std::ops::Index;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let _parent_one = self.selection_method.select(rng, population).chromosome();

                let _parent_two = self.selection_method.select(rng, population).chromosome();

                todo!()
            })
            .collect()
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[allow(clippy::len_without_is_empty)]
impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn roulette_wheel_selection() {
        let method = RouletteWheelSelection::default();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(3.0),
            TestIndividual::new(4.0),
        ];

        let actual_histogram = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(BTreeMap::default(), |mut histogram, individual| {
                *histogram.entry(individual.fitness() as i32).or_default() += 1;
                histogram
            });

        let expected_histogram = maplit::btreemap! {
            1 => 98,
            2 => 202,
            3 => 301,
            4 => 399,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }

    fn chromosome() -> Chromosome {
        Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        }
    }

    mod len {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(chromosome().len(), 3);
        }
    }

    mod iter {
        use super::*;

        #[test]
        fn test() {
            let chromosome = chromosome();
            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &3.0);
            assert_eq!(genes[1], &1.0);
            assert_eq!(genes[2], &2.0);
        }
    }

    mod iter_mut {
        use super::*;

        #[test]
        fn test() {
            let mut chromosome = chromosome();

            chromosome.iter_mut().for_each(|gene| {
                *gene *= 10.0;
            });

            let genes: Vec<_> = chromosome.iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], &30.0);
            assert_eq!(genes[1], &10.0);
            assert_eq!(genes[2], &20.0);
        }
    }

    mod index {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod from_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

            assert_eq!(chromosome[0], 3.0);
            assert_eq!(chromosome[1], 1.0);
            assert_eq!(chromosome[2], 2.0);
        }
    }

    mod into_iterator {
        use super::*;

        #[test]
        fn test() {
            let chromosome = Chromosome {
                genes: vec![3.0, 1.0, 2.0],
            };

            let genes: Vec<_> = chromosome.into_iter().collect();

            assert_eq!(genes.len(), 3);
            assert_eq!(genes[0], 3.0);
            assert_eq!(genes[1], 1.0);
            assert_eq!(genes[2], 2.0);
        }
    }
}
