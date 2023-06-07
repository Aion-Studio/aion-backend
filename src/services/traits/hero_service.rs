use crate::{
    models::hero::{Hero, Item, RetinueSlot},
    types::RepoFuture,
};

pub trait HeroService {
    fn create_hero<'a>(&'a self, new_hero: Hero) -> RepoFuture<'a, Hero>;
    fn level_up_hero<'a>(&'a self, hero: Hero) -> RepoFuture<'a, Hero>;
    fn add_experience(&self, hero: &mut Hero, xp: i32);
    fn equip(&self, hero: &mut Hero, item: Item);
    fn assign_follower_to_hero(&self, hero: &mut Hero, slot: RetinueSlot);
}
