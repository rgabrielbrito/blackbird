use crate::*;

#[derive(Clone, Debug, Default)]
pub struct RankSelection;

impl SelectionMethod for RankSelection {
    fn select<'a, I>(&self, _rng: &mut dyn RngCore, _population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        todo!()
    }
}
