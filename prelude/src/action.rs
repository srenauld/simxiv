use super::{Entity, Effect};
use std::sync::Arc;
use super::Moment;
use uuid::Uuid;

#[derive(Clone)]
pub enum ConditionalAction {
    Cast {
        spell: u32,
        selector: Arc<Box<Fn(&Entity, Vec<&Entity>) -> Option<Uuid>>>
    }
}

#[derive(Clone, Debug)]
pub enum ActionTarget { 
    Direct {
        range: u32,
        target_mask: u32
    },
    Area {
        range: u32,
        target_mask: u32,
        radius: u32
    }
}

#[derive(Clone)]
pub struct Action {
    pub id: u32,
    pub target_type: ActionTarget,
    pub available: Arc<Box<Fn(&Entity) -> bool>>,
    pub effect: Arc<Box<Fn(&Entity, Vec<&Entity>) -> Vec<Effect>>>,
    pub animation_delay: Option<Moment>,
    pub off_gcd: bool,
    pub cast_time: Arc<Box<Fn(&Entity) -> Moment>>, // None = off-gcd
    pub recast_time: Moment,
}

impl Action {
    pub fn new(id:u32, base_cast_time: Moment) -> Self {
        Self {
            id: id,
            target_type: ActionTarget::Direct {
                range: 40,
                target_mask: 0
            },
            off_gcd: true,
            available: Arc::new(Box::new(|_| true)),
            effect: Arc::new(Box::new(|_, _| vec![])),
            animation_delay: None,
            cast_time: Arc::new(Box::new(move |_| base_cast_time.clone())),
            recast_time: Moment::new(0, 0)
        }
    }
    pub fn with_target_type(self, new_type: ActionTarget) -> Self {
        Self {
            id: self.id,
            target_type: new_type,
            available: self.available,
            effect: self.effect,
            off_gcd: self.off_gcd,
            animation_delay: self.animation_delay,
            cast_time: self.cast_time,
            recast_time: self.recast_time
        }
    }
    pub fn with_available_condition(self, new_avail: impl Fn(&Entity) -> bool + 'static) -> Self {
        Self {
            id: self.id,
            target_type: self.target_type,
            available: Arc::new(Box::new(new_avail)),
            effect: self.effect,
            off_gcd: self.off_gcd,
            animation_delay: self.animation_delay,
            cast_time: self.cast_time,
            recast_time: self.recast_time
        }
    }
    pub fn with_effects(self, new_effect: impl Fn(&Entity, Vec<&Entity>) -> Vec<Effect> + 'static) -> Self {
        Self {
            id: self.id,
            target_type: self.target_type,
            available: self.available,
            effect: Arc::new(Box::new(new_effect)),
            animation_delay: self.animation_delay,
            cast_time: self.cast_time,
            off_gcd: self.off_gcd,
            recast_time: self.recast_time
        }
    }
    pub fn with_effect_modifier(self, new_effect: impl Fn(&Entity, Vec<&Entity>) -> Vec<Effect> + 'static) -> Self {
        let old_effect_fn = Arc::clone(&self.effect);
        Self {
            id: self.id,
            target_type: self.target_type,
            available: self.available,
            effect: Arc::new(Box::new(move |entity, targets| {
                let mut old_effects = (old_effect_fn)(entity, targets.clone());
                let mut new_effects = new_effect(entity, targets);
                old_effects.append(&mut new_effects);
                old_effects
            })),
            animation_delay: self.animation_delay,
            cast_time: self.cast_time,
            off_gcd: self.off_gcd,
            recast_time: self.recast_time
        }
    }
    pub fn with_animation_delay(self, new_delay: Option<Moment>) -> Self {
        Self {
            id: self.id,
            target_type: self.target_type,
            available: self.available,
            effect: self.effect,
            animation_delay: new_delay,
            cast_time: self.cast_time,
            off_gcd: self.off_gcd,
            recast_time: self.recast_time
        }
    }
    pub fn with_cast_modifier(self, modifier: impl Fn(&Entity, Moment) -> Moment + 'static) -> Self {
        let old_cast = Arc::clone(&self.cast_time);
        Self {
            id: self.id,
            target_type: self.target_type,
            available: self.available,
            effect: self.effect,
            animation_delay: self.animation_delay,
            cast_time: Arc::new(Box::new(move |entity| {
                modifier(entity, (old_cast)(entity))
            })),
            off_gcd: self.off_gcd,
            recast_time: self.recast_time
        }
    }
    pub fn with_recast_time(self, new_time: Moment) -> Self {
        Self {
            id: self.id,
            target_type: self.target_type,
            available: self.available,
            effect: self.effect,
            off_gcd: self.off_gcd,
            animation_delay: self.animation_delay,
            cast_time: self.cast_time,
            recast_time: new_time
        }
    }
}