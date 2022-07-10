use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();
        
        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();
        
        Self { animals, food }
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
}
