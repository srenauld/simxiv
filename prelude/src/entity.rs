use super::Action;
use super::AuraEffect;
use super::Moment;
use super::ConditionalAction;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use crate::{SkillType, DamageType};
use super::Aura;
use super::Effect;
use crate::SimError;

#[derive(Clone)]
pub struct Trait {
}

#[derive(Clone)]
pub struct Resource {
    name: String,
    current_value: u32,
    max_value: u32
}
impl Resource {
    fn modify(&mut self, modifier: i32) {
        let intermediate = (self.current_value as i32) + modifier;
        match intermediate < 0 {
            true => self.current_value = 0,
            false => match intermediate > self.max_value as i32 {
                true => self.current_value = self.max_value.clone(),
                false => self.current_value = intermediate as u32
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Job {
    GLA,
    PGL,
    MRD,
    LNC,
    ARC,
    CNJ,
    THM,
    PLD,
    MNK,
    WAR,
    DRG,
    BRD,
    WHM,
    BLM,
    ACN,
    SMN,
    SCH,
    ROG,
    NIN,
    MCH,
    DRK,
    AST,
    SAM,
    RDM
}

#[derive(Clone)]
pub enum Status {
    Casting {
        source: Box<Entity>,
        target: Box<Entity>,
        spell: Action,
        start_time: Moment,
        end_time: Moment
    },
    AnimationLocked {
        action: Action,
        start_time: Moment,
        end_time: Moment
    },
    Idle {
        start_time: Moment
    }
}
impl PartialEq for Status {
    fn eq(&self, rhs:&Self) -> bool {
        match (self, rhs) {
            (
                Status::Casting { source: left_source, target: left_target, spell: left_spell, start_time: left_start, end_time: left_end },
                Status::Casting { source: right_source, target: right_target, spell: right_spell, start_time: right_start, end_time: right_end }
            ) => {
                left_source.id == right_source.id && left_target.id == right_target.id && left_spell.id == right_spell.id
            },
            (
                Status::AnimationLocked { action: left_action, start_time: left_time, end_time: left_end },
                Status::AnimationLocked { action: right_action, start_time: right_time, end_time: right_end },
            ) => {
                left_action.id == right_action.id
            },
            (
                Status::Idle { start_time: left_time },
                Status::Idle { start_time: right_time }
            ) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub level: u16,
    pub job: Option<Job>,
    pub status: Status,
    pub last_auto: Moment,
    pub traits: Vec<Trait>,
    pub auras: HashMap<u32, Vec<Aura>>,
    last_tick: HashMap<(u32, Uuid), Moment>,
    statistics: HashMap<String, u32>,
    resources: HashMap<String, Resource>,
    action_repository: Arc<Vec<Action>>,
    action_list: Vec<ConditionalAction>
}
impl Entity {

    pub fn can_dodge(&self, ability: &u32) -> bool {
        false
    }
    pub fn can_parry(&self, ability: &u32) -> bool {
        false
    }
    pub fn can_block(&self, ability: &u32) -> bool {
        false
    }
    pub fn get_statistic(&self, name:&str) -> u32 {
        match self.statistics.get(&name.to_string()) {
            Some(val) => {
                val.clone()
            },
            None => 0
        }
    }
    pub fn set_statistic(&mut self, name:&str, value:u32) {
        self.statistics.insert(name.to_string(), value);
    }
    pub fn remove_aura(&mut self, id:&u32, source: Option<Uuid>) {
        match self.auras.get_mut(id) {
            Some(ref mut aura_list) => aura_list.retain(|e| match source {
                Some(uuid) => {
                    e.source.id != uuid
                },
                None => false
            }),
            None => ()
        }
    }
    pub fn add_aura(&mut self, aura: Aura) {
        let mut aura_list = self.auras.entry(aura.id).or_insert_with(|| vec![]);
        aura_list.push(aura);
    }
    pub fn has_own_aura(&self, id:&u32) -> Option<Aura> {
        let matching_auras:Vec<Aura> = self.auras_by_id(id).into_iter().filter(|aura| aura.source.id == self.id).collect::<Vec<Aura>>();
        matching_auras.first().cloned()
    }
    pub fn cleanup(&mut self, current_time: Moment) {
        // Get rid of all the auras that are no longer relevant
        self.auras.iter_mut().for_each(|(k, mut aura_list)| {
            aura_list.retain(|o| o.end_time > current_time)
        });
    }
    pub fn auras_by_id(&self, id:&u32) -> Vec<Aura> {
        self.auras.get(id).map(|r| r.clone()).or(Some(vec![])).unwrap()
    }

    pub fn create(name:String, job: Option<Job>, level: u16, apl: Vec<ConditionalAction>, repository: Arc<Vec<Action>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name,
            level: level,
            job: job,
            status: Status::Idle {
                start_time: Moment::new(0, 0)
            },
            traits: vec![],
            auras: HashMap::new(),
            last_auto: Moment::new(0, 0),
            last_tick: HashMap::new(),
            statistics: HashMap::new(),
            resources: HashMap::new(),
            action_repository: repository,
            action_list: apl
        }
    }

    pub fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
    pub fn modify_resource(&mut self, resource_name: String, amount: i32) {
        self.resources.get_mut(&resource_name).map(|ref mut resource| {
            resource.modify(amount)
        });
    }
    pub fn process_dots(&self, moment: Moment) -> Vec<Effect> {
        // Cycle through auras and find DoTs.
        self.auras.iter().fold(vec![], |current_effects, (aura_id, aura)| {
            aura.iter().fold(current_effects, |current_effects, aura| {
                aura.effects.iter().map(|effect| {
                    match effect {
                        AuraEffect::DoT { id, ticks, potency, skill_type, r#type } => {
                            let mut ticks_guard = ticks.write().unwrap();
                            match ticks_guard.first() {
                                Some(next_tick) => match next_tick < &moment {
                                    true => {
                                        // Convert the tick to damage
                                        ticks_guard.remove(0);
                                        Some(Effect::Damage {
                                            periodic: true,
                                            potency: potency.clone(),
                                            source: aura.source.clone(),
                                            target: aura.target.clone(),
                                            skill_type: skill_type.clone(),
                                            r#type: r#type.clone(),
                                            action: aura.id.clone()
                                        })
                                    },
                                    _ => None
                                },
                                _ => None
                            }
                        },
                        _ => None
                    }
                }).filter(|r| r.is_some()).map(|r| r.unwrap()).collect()
            })
        })
    }
    pub fn potency_modifier(&self, skill_id: &u32, base_potency: u32) -> u32 {
        base_potency
    }
    pub fn get_extra_ability_dhc(&self, d_type: &DamageType, skill_type: &SkillType, skill_id: &u32) -> f64 {
        1.0
    }
    pub fn get_extra_ability_chc(&self, d_type: &DamageType, skill_type: &SkillType, skill_id: &u32) -> f64 {
        1.0
    }
    pub fn modify_damage_from_ability(&self, skill_id: &u32, d_type: &DamageType, skill_type: &SkillType, base_damage: u32) -> u32 {
        base_damage
    }
    pub fn get_traits_for_ability_damage(&self, d_type: &DamageType, skill_type: &SkillType, ability_id: u32) -> f64 {
        1.0
    }
    pub fn effects_at(&self, moment: Moment, entities: &HashMap<Uuid, Entity>) -> Result<Vec<Effect>, SimError> {
        let mut new_effects = vec![];

        if let Status::AnimationLocked { ref action, ref start_time, ref end_time } = &self.status {
            match end_time <= &moment {
                true => {
                    new_effects.push(Effect::BeginIdle {
                        target: self.clone(),
                        start: moment.clone()
                    })
                },
                false => ()
            }
        }
        if let Status::Casting { ref source, ref target, ref spell, ref start_time, ref end_time } = &self.status {
            match end_time <= &moment {
                true => {
                    let mut effects = (spell.effect)(source, vec![target]);
                    new_effects.append(&mut effects);
                    match spell.animation_delay {
                        Some(ref delay) => new_effects.push(Effect::BeginAnimationLock {
                            target: self.clone(),
                            action: spell.clone(),
                            start: end_time.clone(),
                            duration: delay.clone()
                        }),
                        None => new_effects.push(Effect::BeginIdle {
                            target: self.clone(),
                            start: end_time.clone()
                        })
                    }
                },
                false => ()
            }
        }
        if let Status::Idle { ref start_time } = &self.status {
            match start_time <= &moment {
                true => {
                    // Go through the APL, see what we can do
                    let repository = Arc::clone(&self.action_repository);
                    match self.action_list.iter().fold(None, move |state, next_action| {
                        state.or_else(|| {
                            if let ConditionalAction::Cast { ref spell, ref selector } = &next_action {
                                return repository.iter().filter(|i| i.id == *spell).collect::<Vec<&Action>>().first().cloned().and_then(|action| {
                                    match (action.available)(&self) {
                                        true => {
                                            (selector)(&self, entities.iter().map(|(k, v)| v).collect())
                                            .and_then(|r| {
                                                let cast_time = (action.cast_time)(self);
                                                match cast_time > Moment::new(0, 0) {
                                                    // We have a reference to an entity to cast on, and a spell. Let's go
                                                    true => Some(vec![Effect::BeginCast {
                                                        source: self.clone(),
                                                        target: entities.get(&r).unwrap().clone(),
                                                        action: action.clone(),
                                                        duration: (action.cast_time)(self)
                                                    }]),
                                                    false => {
                                                        // Instant case. We instantly process the cast effects and return this + animation lock
                                                        let mut action_effects = (action.effect)(self, vec![entities.get(&r).unwrap()]);
                                                        match action.animation_delay {
                                                            Some(ref delay) => action_effects.push(Effect::BeginAnimationLock {
                                                                target: self.clone(),
                                                                action: action.clone(),
                                                                start: moment.clone(),
                                                                duration: delay.clone()
                                                            }),
                                                            None => action_effects.push(Effect::BeginIdle {
                                                                target: self.clone(),
                                                                start: moment.clone()
                                                            })
                                                        }
                                                        Some(action_effects)
                                                    }
                                                }
                                            })
                                        },
                                        false => {
                                            None
                                        }
                                    }
                                })
                            }
                            return None
                        })
                    }) {
                        Some(ref mut new_effect) => {
                            new_effects.append(new_effect);
                        },
                        None => ()
                    }
                },
                false => ()
            }
        }
        Ok(new_effects)
    }
}