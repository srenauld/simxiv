extern crate uuid;
extern crate simxiv_prelude;
use uuid::Uuid;
use std::borrow::BorrowMut;
use simxiv_prelude::{Moment, Entity, Aura, SimError, Action, Status, Effect};
use std::collections::HashMap;


pub struct Engine {
    pub entities: HashMap<Uuid, Entity>,
    pub current_time: Moment
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            current_time: Moment::new(0, 0)
        }
    }
    pub fn add_entity(&mut self, e: Entity) {
        self.entities.insert(e.id, e);
    }
    pub fn crank_by(&mut self, interval: Moment) -> Result<(), SimError> {
        let mut new_entities:HashMap<Uuid, Entity>;
        let new_time = self.current_time.clone();
        // Go through our entities, see what they will do next
        let effects:Result<Vec<Effect>, SimError> = self.entities.iter().fold(Ok(vec![]), |mut state, (id, ref entity)| {
            state.and_then(|mut current_effects| {
                entity.effects_at(new_time.clone(), &self.entities).map(|mut effects| {
                    current_effects.append(&mut effects);
                    current_effects
                })
            })
        });
        
        // At this point we have every effect that will happen. This means that we can safely use our entity map.
        effects.and_then(|e| {
            self.process_effects(new_time.clone(), e)
        }).map(|_| {
            // We're done with this iteration. Let's allow the entities to clear their internal state
            self.entities.iter_mut().map(|(k, mut e)| e.cleanup(new_time.clone()));
            self.current_time = self.current_time.clone() + interval
        })
    }
    pub fn process_effects(&mut self, time:Moment, effects: Vec<Effect>) -> Result<(), SimError> {
        effects.into_iter().fold(Ok(()), |state, effect| {
            state.and_then(|_| {
                if let Effect::BeginIdle { ref target, ref start } = &effect {
                    let mut target_entity = self.entities.get_mut(&target.id).unwrap();
                    println!("{}: Target {} begins to idle", time, target.name);
                    target_entity.set_status(Status::Idle {
                        start_time: start.clone()
                    });
                }
                if let Effect::ApplyAura { ref source, ref target, ref aura, ref duration } = &effect {
                    println!("{}: Target {} applies aura {} on {}", time, source.name, aura, target.name);
                    let mut target_entity = self.entities.get_mut(&target.id).unwrap();
                    target_entity.add_aura(Aura {
                        id: aura.clone(),
                        source: source.clone(),
                        target: target.clone(),
                        start_time: time.clone(),
                        end_time: time.clone() + duration.clone(),
                        effects: vec![]
                    })
                }
                if let Effect::RemoveAura { ref source, ref target, ref aura } = &effect {
                    let mut target_entity = self.entities.get_mut(&target.id).unwrap();
                    target_entity.remove_aura(aura, Some(source.id))
                }
                if let Effect::Damage { ref source, ref target, ref action, ref potency, ref skill_type, ref r#type, ref periodic } = &effect {
                    return Ok(())
                }
                if let Effect::ModifyResource { ref target, ref resource, ref amount } = &effect {
                    let mut target_entity = self.entities.get_mut(&target.id).unwrap();
                    println!("{}: Target {} gained {} {}", time, target.name, resource, amount);
                    target_entity.modify_resource(resource.to_string(), amount.clone());
                }
                if let Effect::BeginCast { ref source, ref target, ref action, ref duration } = &effect {
                    println!("{}: Target {} begins to cast {} on {}", time, source.name, action.id, target.name);
                    let mut target_entity = self.entities.get_mut(&source.id).unwrap();
                    target_entity.set_status(Status::Casting {
                        source: Box::new(source.clone()),
                        target: Box::new(target.clone()),
                        spell: action.clone(),
                        start_time: time.clone(),
                        end_time: time.clone() + duration.clone()
                    })
                }
                if let Effect::BeginAnimationLock { ref target, ref action, ref start, ref duration } = &effect {
                    let mut target_entity = self.entities.get_mut(&target.id).unwrap();
                    println!("{}: Target {} is animation locked from casting {} for {}", time, target.name, action.id, duration);
                    target_entity.set_status(Status::AnimationLocked {
                        action: action.clone(),
                        start_time: time.clone(),
                        end_time: time.clone() + duration.clone()
                    })
                }
                Ok(())
            })
        })
    }
}

#[cfg(test)]
mod tests{
    use simxiv_prelude::{ConditionalAction, Job, Status, Entity, Action, Effect, Moment};
    use crate::Engine;
    use std::sync::Arc;
    use uuid::Uuid;

    struct State(Moment, Uuid, String);
    fn verify_states(mut engine:Engine, cranking_speed: Moment, mut states: Vec<State>) {
        states.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut filter = move |current_time:&Moment| -> Option<Vec<State>> {
            if states.len() == 0 {
                return None
            }
            let mut output = vec![];
            let mut i = 0;
            while i != states.len() {
                if &states[i].0 < current_time {
                    output.push(states.remove(i));
                } else {
                    i += 1;
                }
            }
            Some(output)
        };
        let mut done = false;
        while !done {
            engine.crank_by(cranking_speed.clone());
            let filter_states = filter(&engine.current_time);
            if (filter_states.is_none()) {
                println!("Done");
                done = true;
            }
            for state in filter_states.or(Some(vec![])).unwrap() {
                // Verify the state
                println!("Verifying state");
                assert_eq!(engine.entities.get(&state.1).map(|e| match state.2.as_str() {
                    "idle" => match e.status {
                        Status::Idle { .. } => {
                            println!("Target is idling");
                            true
                        },
                        _ => {
                            println!("Target is not idling when it should be");
                            false
                        }
                    },
                    "casting" => match e.status {
                        Status::Casting { .. } => {
                            println!("Target is casting");
                            true
                        },
                        _ => {
                            println!("Target is not casting when it should be");
                            false
                        }
                    },
                    "locked" => match e.status {
                        Status::AnimationLocked { .. } => {
                            println!("Target is animation locked");
                            true
                        },
                        _ => {
                            println!("Target is not animation locked when it should be");
                            false
                        }
                    },
                    _ => false
                }), Some(true));
            }
        }
    }
    fn dualcast_generate(action:Action) -> Action {
        action.with_effect_modifier(|source, target| {
            match source.has_own_aura(&2) {
                Some(aura) => {
                    vec![]
                },
                None => {
                    vec![
                        Effect::ApplyAura {
                            source: source.clone(),
                            target: source.clone(),
                            aura: 2,
                            duration: Moment::new(12, 0)
                        }
                    ]
                }
            }
        })
    }

    fn dualcast_consume(action:Action) -> Action {
        action.with_cast_modifier(|ref entity, current_cast_time| match entity.has_own_aura(&2) {
            Some(aura) => Moment::new(0, 0),
            None => current_cast_time
        })
        .with_effect_modifier(|source, target| {
            vec![
                Effect::RemoveAura {
                    source: source.clone(),
                    target: source.clone(),
                    aura: 2
                }
            ]
        })
    }
    #[test]
    fn basic_cast_anim_lock_to_idle() {
        let mut engine = Engine::new();
        let red_mage = Entity::create("red_mage".to_string(), Some(Job::RDM), 70, vec![
            ConditionalAction::Cast {
                spell: 1,
                selector: Arc::new(Box::new(|source, targets| {
                    targets.into_iter().filter(|target| {
                        target.name == "big_bad".to_string()
                    }).collect::<Vec<&Entity>>().first().map(|r| r.id.clone())
                }))
            }
        ], Arc::new(vec![
            dualcast_generate(dualcast_consume(Action::new(1, Moment::new(2,500))
                .with_animation_delay(Some(Moment::new(0, 750)))))
        ]));
        let big_bad = Entity::create("big_bad".to_string(), None, 70, Vec::new(), Arc::new(vec![]));
        let red_mage_id = red_mage.id.clone();
        let big_bad_id = big_bad.id.clone();
        engine.add_entity(red_mage);
        engine.add_entity(big_bad);
        verify_states(engine, Moment::new(0, 100), vec![
            State(Moment::new(1,0), red_mage_id.clone(), "casting".to_string()),
            State(Moment::new(2,750), red_mage_id.clone(), "locked".to_string()),
            State(Moment::new(4,250), red_mage_id.clone(), "idle".to_string())
        ])
    }
    #[test]
    fn test_dualcast() {
        let mut engine = Engine::new();
        let red_mage = Entity::create("red_mage".to_string(), Some(Job::RDM), 70, vec![
            ConditionalAction::Cast {
                spell: 1,
                selector: Arc::new(Box::new(|source, targets| {
                    targets.into_iter().filter(|target| {
                        target.name == "big_bad".to_string()
                    }).collect::<Vec<&Entity>>().first().map(|r| r.id.clone())
                }))
            }
        ], Arc::new(vec![
            dualcast_generate(dualcast_consume(Action::new(1, Moment::new(2,500))
                .with_animation_delay(Some(Moment::new(0, 750)))))
        ]));
        let big_bad = Entity::create("big_bad".to_string(), None, 70, Vec::new(), Arc::new(vec![]));
        let red_mage_id = red_mage.id.clone();
        let big_bad_id = big_bad.id.clone();
        engine.add_entity(red_mage);
        engine.add_entity(big_bad);
        verify_states(engine, Moment::new(0, 100), vec![
            State(Moment::new(1,0), red_mage_id.clone(), "casting".to_string()),
            State(Moment::new(2,750), red_mage_id.clone(), "locked".to_string()),
            // A dualcast should have two locked periods one after the other
            State(Moment::new(3,400), red_mage_id.clone(), "locked".to_string())
        ])
    }

    #[test]
    fn handle_aura_cast_time_interactions() {
        
    }
}
