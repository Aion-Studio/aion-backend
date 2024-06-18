use prisma_client_rust::QueryError;

use crate::{models::hero::Hero, webserver::get_prisma_client};

struct HeroRepo {}

impl HeroRepo {
    pub async fn insert_hero(&self, new_hero: Hero) -> Result<Hero, QueryError> {
        let prisma = get_prisma_client();
        // ...

        let account = prisma
            .account()
            .create("tempId123".to_string(), vec![])
            .exec()
            .await
            .unwrap();

        let result = prisma
            .hero()
            .create(
                new_hero.class,
                new_hero.hp,
                new_hero.strength,
                new_hero.aion_capacity,
                account::id::equals(account.clone().id),
                vec![hero::name::set(new_hero.name)],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        let hero: Hero = result.into();

        self.prisma
            .deck()
            .create(vec![deck::hero_id::set(Some(hero.get_id()))])
            .exec()
            .await?;
        let region_name = RegionName::Dusane;
        self.prisma
            .hero_region()
            .create(
                0.0,
                hero::id::equals(hero.get_id()),
                region::name::equals(region_name.to_str()),
                vec![current_location::set(true)],
            )
            .exec()
            .await?;
        let hero = self.hero_by_id(hero.get_id()).await.unwrap();
        Ok(hero)
    }
}
