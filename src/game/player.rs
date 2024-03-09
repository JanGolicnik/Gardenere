use std::collections::HashMap;

use super::plant::PlantType;

pub struct Player {
    pub hp: f32,
    pub coins: u32,
    pub total_coins: u32,
    pub owned_seeds: HashMap<PlantType, u32>,
    pub owned_pots: u32,
    pub has_axe: bool,
}
