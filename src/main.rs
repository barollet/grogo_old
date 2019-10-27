use std::ops::{Index, IndexMut};

pub type Bitboards = u64;

pub type Player = bool;
pub const BLACK: Player = true;
pub const WHITE: Player = false;

pub struct PlayerDependant<T> {
    elems: [T; 2],
}

impl<T> PlayerDependant<T> {
    pub fn new(elems: [T; 2]) -> Self {
        Self { elems }
    }
}

impl<T> Index<Player> for PlayerDependant<T> {
    type Output = T;
    fn index(&self, p: Player) -> &Self::Output {
        &self.elems[p as usize]
    }
}

impl<T> IndexMut<Player> for PlayerDependant<T> {
    fn index_mut(&mut self, p: Player) -> &mut Self::Output {
        &mut self.elems[p as usize]
    }
}

pub struct Board {
    intersections: [PlayerDependant<Bitboards>; 6],
    size: (usize, usize),
}

fn main() {
    println!("Hello, world!");
}
