use super::Entity;
use super::Moment;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub enum Element {
    Fire,
    Earth,
    Unaspected
}

#[derive(Clone)]
pub enum DamageType {
    Blunt,
    Slashing,
    Piercing,
    Magic(Element)
}

#[derive(Clone)]
pub enum SkillType {
    Auto,
    Skill,
    Spell
}

#[derive(Clone)]
pub enum AuraEffect {
    DoT {
        id: u32,
        ticks: Arc<RwLock<Vec<Moment>>>,
        potency: u32,
        skill_type: SkillType,
        r#type: DamageType
    },
    DamageModifier {
        damage_type: Vec<DamageType>,
        modifier: i32
    },
    PotencyModifier {
        damage_type: Vec<DamageType>,
        modifier: i32
    }
}
#[derive(Clone)]
pub struct Aura {
    pub id: u32,
    pub source: Entity,
    pub target: Entity,
    pub start_time: Moment,
    pub end_time: Moment,
    pub effects: Vec<AuraEffect>
}