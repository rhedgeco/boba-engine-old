use std::hint::black_box;

use boba_ecs::{
    pearl::{id::PearlIdSet, PearlSet},
    Pearl, World,
};

pub struct TestPearl1 {
    value: u32,
}

pub struct TestPearl2 {
    value: u32,
}

fn main() {
    let mut world = World::new();

    for _ in 0..100000 {
        let mut pearls = PearlSet::new();
        pearls.insert(TestPearl1 { value: 42 });
        pearls.insert(TestPearl2 { value: 43 });
        world.spawn_with(pearls);
    }

    for _ in 0..100000 {
        let mut pearls = PearlSet::new();
        pearls.insert(TestPearl1 { value: 44 });
        world.spawn_with(pearls);
    }

    for _ in 0..100000 {
        let mut pearls = PearlSet::new();
        pearls.insert(TestPearl1 { value: 45 });
        pearls.insert(TestPearl2 { value: 46 });
        world.spawn_with(pearls);
    }

    let mut query_ids = PearlIdSet::new();
    query_ids.insert(TestPearl1::id());
    query_ids.insert(TestPearl2::id());

    test_system(&mut world);
}

fn test_system(world: &mut World) {
    let mut query_ids = PearlIdSet::new();
    query_ids.insert(TestPearl1::id());
    query_ids.insert(TestPearl2::id());

    let query = world.query_contains(&query_ids);
    for archetype in query {
        println!("New Archetype Query");
        let mut fetcher = archetype.fetch_iter();
        let Some(entities) = fetcher.entities() else { break };
        let Some(pearl1_iter) = fetcher.get::<TestPearl1>() else { break };
        let Some(pearl2_iter) = fetcher.get::<TestPearl2>() else { break };
        for (entity, (pearl1, pearl2)) in entities.iter().zip(pearl1_iter.zip(pearl2_iter)) {
            black_box((&entity, &pearl1, &pearl2));
            println!(
                "Entity: {}, Value1: {}, Value2: {}",
                entity.index(),
                pearl1.value,
                pearl2.value
            );
        }
    }
}
