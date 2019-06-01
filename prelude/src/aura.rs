use super::Entity;
use super::Moment;
#[derive(Clone)]
pub enum AuraEffect {
    DoT {
        ticks: Vec<Moment>
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