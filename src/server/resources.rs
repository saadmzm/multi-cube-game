use std::collections::HashMap;

use bevy::ecs::system::Resource;
use multi_cube_game::PlayerAttributes;
use renet::ClientId;

#[derive(Resource, Clone)]
pub struct PlayerLobby(pub HashMap<ClientId, PlayerAttributes>);