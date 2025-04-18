use bevy::ecs::event::Event;
use multi_cube_game::PlayerAttributes;
use renet::ClientId;

#[derive(Event)]
pub struct PlayerSpawnEvent(pub ClientId);

#[derive(Event)]
pub struct PlayerDespawnEvent(pub ClientId);

#[derive(Event)]
pub struct PlayerMoveEvent(pub ClientId, pub [f32; 3]);

#[derive(Event)]
pub struct LobbySyncEvent(pub std::collections::HashMap<ClientId, PlayerAttributes>);