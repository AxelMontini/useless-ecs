use ecs_core::{Component, World};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Thingy<'a> {
    thing: &'a u32,
}

impl<'a> Component for Thingy<'a> {}

#[allow(unused)]
impl<'a> Thingy<'a> {
    pub fn new(thing: &'a u32) -> Self {
        Self { thing }
    }
}

#[allow(unused)]
fn main() {
    let mut world = World::new();

    // Create the two entities we're gonna test with (requires mutable borrow)
    let player = world.create_entity();
    let enemy = world.create_entity();

    // Creates the storages (requires mutable borrow)
    let _storage = world.ensure_storage_exists::<Position>();
    let _storage = world.ensure_storage_exists::<Thingy>();

    if let Some(storage) = world.storage_mut::<Position>() {
        storage.insert(player, Position::new(0.0, 0.0, 0.0));
        storage.insert(enemy, Position::new(1.0, 0.0, 0.0));

        // Mutable borrow ends here

        let player_pos = storage.get(player);

        // storage.remove(player); // this would trigger an error: Mutable borrow while immutably borrowed

        println!("Position of player: {:?}", player_pos);
        println!("Position of enemy: {:?}", storage.get(enemy));
    }

    // Attempt to insert some stuff that outlives the things it references
    if let Some(storage) = world.storage_mut::<Thingy>() {
        let short_lived = 33u32;

        // storage.insert(player, Thingy::new(&short_lived)); // Error because short_lived must have 'static lifetime
    }
}
