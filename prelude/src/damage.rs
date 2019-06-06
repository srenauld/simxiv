use super::Entity;
use super::Effect;
use super::aura::{DamageType, SkillType, Element};
use super::Job;
use super::Job::*;
use std::sync::{Arc, Mutex};
use math::round::{floor, ceil};
use rand::{StdRng, Rng};
use rand::rngs::{ThreadRng};
pub trait Random {
    fn gen_f64(&mut self) -> f64;
}
pub struct PassthroughRandom {
    inner: ThreadRng
}
impl PassthroughRandom {
    pub fn new() -> Self {
        Self {
            inner: rand::thread_rng()
        }
    }
}
impl Random for PassthroughRandom {
    fn gen_f64(&mut self) -> f64 {
        self.inner.gen()
    }
}
#[derive(Debug, PartialEq)]
pub enum AttackRoll {
    Hit(bool),
    CriticalHit(bool)
}
#[derive(Debug)]
pub enum DefenseRoll {
    Dodge,
    Parry,
    Block,
    Hit
}

pub struct RawDamage {
    pub value: u32,
    pub ability: u32,
    pub range: (u32, u32),
    pub target: Entity,
    pub r#type: DamageType,
    pub attack_roll: AttackRoll
}
pub struct AppliedDamage {
    value: u32,
    range: (u32, u32),
    r#type: DamageType,
    attack_roll: AttackRoll,
    defense_roll: DefenseRoll
}

pub trait DamageStrategy {
    fn deal_damage(&self, source: &Entity, damage: Effect) -> RawDamage;
    fn apply_damage(&self, target: &Entity, damage: RawDamage) -> AppliedDamage;
}

pub struct AssumedDamageStrategy {
    prng: Arc<Mutex<Box<Random>>>
}
impl AssumedDamageStrategy {
    pub fn new() -> Self {
        Self {
            prng: Arc::new(Mutex::new(Box::new(PassthroughRandom::new())))
        }
    }
    pub fn primary_stat(&self, job:&Job) -> &str {
        match job {
            Job::DRG | Job::MNK | Job::WAR | Job::PLD | Job::DRK | Job::SAM => "Strength",
            Job::BRD | Job::NIN | Job::MCH => "Dexterity",
            Job::BLM | Job::SMN | Job::RDM => "Intelligence",
            Job::WHM | Job::AST | Job::SCH => "Mind",
            _ => "Strength"
        }
    }
    pub fn level_main(&self, level: &u16) -> u32 {
        match level {
            1	=>	20	,
            2	=>	21	,
            3	=>	22	,
            4	=>	24	,
            5	=>	26	,
            6	=>	27	,
            7	=>	29	,
            8	=>	31	,
            9	=>	33	,
            10	=>	35	,
            11	=>	36	,
            12	=>	38	,
            13	=>	41	,
            14	=>	44	,
            15	=>	46	,
            16	=>	49	,
            17	=>	52	,
            18	=>	54	,
            19	=>	57	,
            20	=>	60	,
            21	=>	63	,
            22	=>	67	,
            23	=>	71	,
            24	=>	74	,
            25	=>	78	,
            26	=>	81	,
            27	=>	85	,
            28	=>	89	,
            29	=>	92	,
            30	=>	97	,
            31	=>	101	,
            32	=>	106	,
            33	=>	110	,
            34	=>	115	,
            35	=>	119	,
            36	=>	124	,
            37	=>	128	,
            38	=>	134	,
            39	=>	139	,
            40	=>	144	,
            41	=>	150	,
            42	=>	155	,
            43	=>	161	,
            44	=>	166	,
            45	=>	171	,
            46	=>	177	,
            47	=>	183	,
            48	=>	189	,
            49	=>	196	,
            50	=>	202	,
            51	=>	204	,
            52	=>	205	,
            53	=>	207	,
            54	=>	209	,
            55	=>	210	,
            56	=>	212	,
            57	=>	214	,
            58	=>	215	,
            59	=>	217	,
            60	=>	218	,
            61	=>	224	,
            62	=>	228	,
            63	=>	236	,
            64	=>	244	,
            65	=>	252	,
            66	=>	260	,
            67	=>	268	,
            68	=>	276	,
            69	=>	284	,
            70	=>	292	,
            _ => panic!()
        }
    }
    pub fn ap_div(&self, level: &u16) -> u32 { 
        125
    }
    pub fn level_sub(&self, level: &u16) -> u32 {
        match level {
            1	=>	56	,
            2	=>	57	,
            3	=>	60	,
            4	=>	62	,
            5	=>	65	,
            6	=>	68	,
            7	=>	70	,
            8	=>	73	,
            9	=>	76	,
            10	=>	78	,
            11	=>	82	,
            12	=>	85	,
            13	=>	89	,
            14	=>	93	,
            15	=>	96	,
            16	=>	100	,
            17	=>	104	,
            18	=>	109	,
            19	=>	113	,
            20	=>	116	,
            21	=>	122	,
            22	=>	127	,
            23	=>	133	,
            24	=>	138	,
            25	=>	144	,
            26	=>	150	,
            27	=>	155	,
            28	=>	162	,
            29	=>	168	,
            30	=>	173	,
            31	=>	181	,
            32	=>	188	,
            33	=>	194	,
            34	=>	202	,
            35	=>	209	,
            36	=>	215	,
            37	=>	223	,
            38	=>	229	,
            39	=>	236	,
            40	=>	244	,
            41	=>	253	,
            42	=>	263	,
            43	=>	272	,
            44	=>	283	,
            45	=>	292	,
            46	=>	302	,
            47	=>	311	,
            48	=>	322	,
            49	=>	331	,
            50	=>	341	,
            51	=>	342	,
            52	=>	344	,
            53	=>	345	,
            54	=>	346	,
            55	=>	347	,
            56	=>	349	,
            57	=>	350	,
            58	=>	351	,
            59	=>	352	,
            60	=>	354	,
            61	=>	355	,
            62	=>	356	,
            63	=>	357	,
            64	=>	358	,
            65	=>	359	,
            66	=>	360	,
            67	=>	361	,
            68	=>	362	,
            69	=>	363	,
            70	=>	364	,
            _ => panic!()
        }
    }
    pub fn level_div(&self, level: &u16) -> u32 {
        match level {
            1	=>	56	,
            2	=>	57	,
            3	=>	60	,
            4	=>	62	,
            5	=>	65	,
            6	=>	68	,
            7	=>	70	,
            8	=>	73	,
            9	=>	76	,
            10	=>	78	,
            11	=>	82	,
            12	=>	85	,
            13	=>	89	,
            14	=>	93	,
            15	=>	96	,
            16	=>	100	,
            17	=>	104	,
            18	=>	109	,
            19	=>	113	,
            20	=>	116	,
            21	=>	122	,
            22	=>	127	,
            23	=>	133	,
            24	=>	138	,
            25	=>	144	,
            26	=>	150	,
            27	=>	155	,
            28	=>	162	,
            29	=>	168	,
            30	=>	173	,
            31	=>	181	,
            32	=>	188	,
            33	=>	194	,
            34	=>	202	,
            35	=>	209	,
            36	=>	215	,
            37	=>	223	,
            38	=>	229	,
            39	=>	236	,
            40	=>	244	,
            41	=>	253	,
            42	=>	263	,
            43	=>	272	,
            44	=>	283	,
            45	=>	292	,
            46	=>	302	,
            47	=>	311	,
            48	=>	322	,
            49	=>	331	,
            50	=>	341	,
            51	=>	393	,
            52	=>	444	,
            53	=>	496	,
            54	=>	548	,
            55	=>	600	,
            56	=>	651	,
            57	=>	703	,
            58	=>	755	,
            59	=>	806	,
            60	=>	858	,
            61	=>	941	,
            62	=>	1032	,
            63	=>	1133	,
            64	=>	1243	,
            65	=>	1364	,
            66	=>	1497	,
            67	=>	1643	,
            68	=>	1802	,
            69	=>	1978	,
            70	=>	2170	,
            _ => panic!()
        }
    }
}
impl DamageStrategy for AssumedDamageStrategy {
    fn apply_damage(&self, target:&Entity, damage: RawDamage) -> AppliedDamage {
        let mut rng = self.prng.lock().unwrap();
        let div_modifier:f64 = self.level_div(&target.level).into();
        let modifier:f64 = self.level_main(&target.level).into();
        let sub_modifier:f64 = self.level_sub(&target.level).into();
        let f_def:f64 = match damage.r#type {
            DamageType::Magic(_) => {
                let coefficient:f64 = 15.0 * (target.get_statistic("Magic Defense") as f64)/div_modifier;
                floor(coefficient, 0)/100.0
            },
            _ => {
                // Uses phys damage
                let coefficient:f64 = 15.0 * (target.get_statistic("Defense") as f64)/div_modifier;
                floor(coefficient, 0)/100.0
            }
        };

        // Work out the common factors before we move to anything else
        let inter_det:f64 = floor((130.0 as f64) * (target.get_statistic("Determination") as f64 -modifier as f64)/div_modifier,0)+1000.0;
        let inter_tnc:f64 = floor((100.0 as f64) * ((target.get_statistic("Tenacity") as f64 -sub_modifier))/div_modifier, 0)+1000.0;
        let f_det:f64 = floor(inter_det,0)/1000.0;
        let f_tnc:f64 = floor(inter_tnc,0)/1000.0;
        // TODO: Sheltron
        let block_chance:f64 = match &target.job {
            Some(e) if e == &Job::PLD => floor(30.0 * (target.get_statistic("Block Rate") as f64) / div_modifier + 10.0 ,0),
            _ => 0.0
        };
        // TODO: test magic resistances
        let f_res:f64 = 0.0;
        // TODO: fill these in
        let parry_chance:f64 = 0.0;
        let dodge_chance:f64 = 0.0;
        let combat_roll = match &damage.attack_roll {
            AttackRoll::Hit(direct) => match (target.can_dodge(&damage.ability), rng.gen_f64()) {
                (true, v) if v>dodge_chance => match (target.can_block(&damage.ability), rng.gen_f64()) {
                    (true, v) if v>block_chance => match (target.can_parry(&damage.ability), rng.gen_f64()) {
                        (true, v) if v>parry_chance => DefenseRoll::Hit,
                        _ => DefenseRoll::Parry
                    },
                    _ => DefenseRoll::Block
                },
                _ => DefenseRoll::Dodge
            },
            _ => {
                // Is a crit and therefore cannot be blocked, parried or dodged
                DefenseRoll::Hit
            }
        };
        let d:f64 = floor((damage.value as f64) * (1.0 - f_def) * (1.0 - f_res) * (2.0 - f_tnc) * (1.0 - (match &combat_roll {
            DefenseRoll::Block => {
                floor(30.0 * (target.get_statistic("Block Strength") as f64)/div_modifier + 10.0, 0)/100.0
            },
            _ => 0.0
        })), 0);
        let apply_buffs = |input:&f64, factor:f64| -> f64 {
            floor(input * factor, 0)
        };
        let random_factor:f64 = rng.gen_f64() * 0.10 + 0.95;
        let min = apply_buffs(&d, 0.95);
        let max = apply_buffs(&d, 1.05);
        let actual = apply_buffs(&d, random_factor);

        AppliedDamage {
            value: actual as u32,
            range: (min as u32, max as u32),
            r#type: damage.r#type,
            attack_roll: damage.attack_roll,
            defense_roll: combat_roll
        }
    }
    fn deal_damage(&self, source: &Entity, damage: Effect) -> RawDamage {
        // We're going to need the PRNG
        let mut rng = self.prng.lock().unwrap();
        
        // Damage is guaranteed to be typed as Effect::Damage, but due to rust specifics, we need to cast it properly.
        // &Entity exists here for auras that do not snapshot.
        match damage {
            Effect::Damage { source, target, potency, r#type, skill_type, action: action_id, periodic } => {
                // Damage calculations are done differently between periodic and non-periodic damage,
                // as non-periodic already takes into account skill/spell speed on cast and recast 
                // times

                // First, let's calculate a few things
                    
                // Potency is easy:
                let new_pot:f64 = source.potency_modifier(&action_id, potency) as f64;
                let f_pot:f64 = new_pot/100.0;

                // Our first split is on which weapon damage to take into account.
                let wd:f64 = match r#type {
                    DamageType::Magic(_) => source.get_statistic("Magic Damage").into(),
                    _ => match skill_type {
                        SkillType::Auto => source.get_statistic("Auto-attack").into(),
                        SkillType::Skill => source.get_statistic("Physical Damage").into(),
                        SkillType::Spell => source.get_statistic("Magic Damage").into()
                    }
                };

                //let ap:f64 = source.job.clone().map_or(0, |i| source.get_statistic(self.primary_stat(&i))).into();
                let ap:f64 = 105.0;
                let modifier:f64 = self.level_main(&source.level).into();
                let div_modifier:f64 = self.level_div(&source.level).into();
                let sub_modifier:f64 = self.level_sub(&source.level).into();
                let ap_div:f64 = self.ap_div(&source.level).into();
                let f_wd:f64 = floor(floor(ap * modifier / 1000.0, 0) as f64 + wd, 0);
                let inter_f_ap:f64 = floor((((source.get_statistic(match r#skill_type {
                    SkillType::Auto | SkillType::Skill => "Attack Power",
                    _ => "Magic Attack Power"
                }) as f64) - (modifier as f64)) * 10000.0)/(80.0 * (modifier as f64)), 0);
                println!("div {:?} ap {:?} mod {:?} inter {:?}", ap_div, (source.get_statistic(match r#skill_type {
                    SkillType::Auto | SkillType::Skill => "Attack Power",
                    _ => "Magic Attack Power"
                }) as f64), modifier, inter_f_ap);
                let f_ap:f64 = (100.0 + inter_f_ap)/100.0;
                let inter_det:f64 = floor((130.0 as f64) * (source.get_statistic("Determination") as f64 -modifier as f64)/div_modifier,0)+1000.0;
                let inter_tnc:f64 = floor((100.0 as f64) * ((source.get_statistic("Tenacity") as f64 -sub_modifier))/div_modifier, 0)+1000.0;
                let f_det:f64 = floor(inter_det,0)/1000.0;
                let f_tnc:f64 = floor(inter_tnc,0)/1000.0;
                let f_ss:f64 = match periodic {
                    true => 1.0,
                    false => 1.0
                };
                let f_traits:f64 = source.get_traits_for_ability_damage(&r#type, &skill_type, action_id);

                // Now that we're done with the first chain, we can work out the crit and dhit chances
                let inter_chc:f64 = 200.0 * (source.get_statistic("Critical Hit Rate") as f64 - sub_modifier)/div_modifier + 50.0;
                let inter_dhc:f64 = 550.0 * (source.get_statistic("Direct Hit Rate") as f64 - sub_modifier)/div_modifier + 50.0;
                let base_chc:f64 = floor(inter_chc, 0)/10.0;
                let base_dhc:f64 = floor(inter_dhc, 0)/10.0;
                let additional_chc_from_traits:f64 = source.get_extra_ability_chc(&r#type, &skill_type, &action_id);
                
                let additional_dhc_from_traits:f64 = source.get_extra_ability_dhc(&r#type, &skill_type, &action_id);
                let dhc:f64 = base_dhc + additional_dhc_from_traits;
                let chc:f64 = base_chc + additional_chc_from_traits;

                let inter_chr:f64 = 200.0 * (source.get_statistic("Critical Hit Rate") as f64 - sub_modifier)/div_modifier + 1000.0;
                let f_chr:f64 = floor(inter_chr, 0)/1000.0;
                // This tells us what we rolled offensively
                let roll1:f64 = rng.gen_f64();
                let roll2:f64 = rng.gen_f64();
                let is_direct = (roll1- dhc) >= 0.0;
                let damage_type = match (roll2 - dhc) >= 0.0 {
                    true => AttackRoll::CriticalHit(is_direct),
                    _  => AttackRoll::Hit(is_direct)
                };
                // From here, we have enough for the first half of the damage calculation:
                let d:f64 = floor(
                    f_pot *
                    f_wd  *
                    f_ap  *
                    f_det *
                    f_tnc *
                    f_traits
                , 0);
                let random_factor:f64 = rng.gen_f64() * 0.10 + 0.95;
                let apply_damage = |input:f64, factor| {
                    let d:f64 = match periodic {
                        true => {
                            let d:f64 = floor(d * f_ss, 0);
                            let d:f64 = floor(d * factor, 0);
                            let d:f64 = floor(d * (match damage_type {
                                AttackRoll::CriticalHit(_) => f_chr,
                                AttackRoll::Hit(_) => 1.0
                            }), 0);
                            floor(d * (match damage_type {
                                AttackRoll::CriticalHit(v) | AttackRoll::Hit(v) if v == true => 1.25,
                                _ => 1.0
                            }), 0)
                        },
                        false => {
                            let d:f64 = floor(d * (match damage_type {
                                AttackRoll::CriticalHit(_) => f_chr,
                                AttackRoll::Hit(_) => 1.0
                            }), 0);
                            let d:f64 = floor(d * (match damage_type {
                                AttackRoll::CriticalHit(v) | AttackRoll::Hit(v) if v == true => 1.25,
                                _ => 1.0
                            }), 0);
                            floor(d * factor, 0)
                        }
                    };
                    source.modify_damage_from_ability(&action_id, &r#type, &skill_type, (d/1.0) as u32)
                };
                let min = apply_damage(d, 0.95);
                let max = apply_damage(d, 1.05);
                let actual = apply_damage(d, random_factor);
                RawDamage {
                    value: actual,
                    range: (min, max),
                    r#type: r#type,
                    ability: action_id,
                    target: target,
                    attack_roll: damage_type
                }
            },
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DamageStrategy;
    use super::{Entity, AssumedDamageStrategy, AttackRoll, SkillType, Job, DamageType, Effect};
    use std::sync::Arc;
    #[test]
    fn base_damage_checks_out() {
        let mut red_mage = Entity::create("red_mage".to_string(), Some(Job::DRK), 70, vec![], Arc::new(vec![]));
        let mut target = Entity::create("red_mage".to_string(), None, 70, vec![], Arc::new(vec![]));
        let strat = AssumedDamageStrategy::new();
        red_mage.set_statistic("Strength", 2011);
        red_mage.set_statistic("Critical Hit Rate", 1155);
        red_mage.set_statistic("Determination", 1834);
        red_mage.set_statistic("Direct Hit Rate", 423);
        red_mage.set_statistic("Attack Power", 2011);
        red_mage.set_statistic("Skill Speed", 603);
        red_mage.set_statistic("Spell Speed", 364);
        red_mage.set_statistic("Tenacity", 1223);
        red_mage.set_statistic("Physical Damage", 105);
        let effect = Effect::Damage {
            source: red_mage.clone(),
            target: target.clone(),
            potency: 150,
            r#type: DamageType::Slashing,
            skill_type: SkillType::Skill,
            action: 2,
            periodic: false
        };
        let mut hit = None;
        while (!hit.is_some()) {
            let output = strat.deal_damage(&red_mage,effect.clone());
            if output.attack_roll == AttackRoll::Hit(false) {
                hit = Some(output);
            }
        }
        let actual_hit = hit.unwrap();
        assert!(1827 > actual_hit.range.0 && 2008 < actual_hit.range.1 && (((2008-1827)/(actual_hit.range.1-actual_hit.range.0)) as f64) < 0.01);
        // First, we set our stats
    }
}