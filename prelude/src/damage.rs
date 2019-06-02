use super::Entity;
use super::Effect;
use super::aura::{DamageType, SkillType, Element};
use super::Job;
use super::Job::*;
use std::sync::{Arc, Mutex};
use math::round::{floor, ceil};
use rand::Rng;

pub enum AttackRoll {
    Hit(bool),
    CriticalHit(bool)
}
pub enum DefenseRoll {
    Dodge,
    Parry,
    Block,
    Direct
}


pub struct RawDamage {
    value: u32,
    ability: u32,
    range: (u32, u32),
    target: Entity,
    r#type: DamageType,
    attack_roll: AttackRoll
}
pub struct AppliedDamage {
    value: u32,
    r#type: DamageType,
    attack_roll: AttackRoll,
    defense_roll: DefenseRoll
}

pub trait DamageStrategy {
    fn deal_damage(&self, source: &Entity, damage: Effect) -> RawDamage;
    fn apply_damage(&self, target: &Entity, damage: RawDamage) -> AppliedDamage;
}

pub struct AssumedDamageStrategy {
    prng: Arc<Mutex<Box<Rng>>>
}
impl AssumedDamageStrategy {
    pub fn new() -> Self {
        Self {
            prng: Arc::new(Mutex::new(Box::new(rand::thread_rng())))
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
        AppliedDamage {
            value: damage.value,
            r#type: damage.r#type,
            attack_roll: damage.attack_roll,
            defense_roll: DefenseRoll::Direct
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
                let f_pot:f64 = (source.potency_modifier(&action_id, potency)/100).into();

                // Our first split is on which weapon damage to take into account.
                let wd:f64 = match r#type {
                    DamageType::Magic(_) => source.get_statistic("Magic Damage").into(),
                    _ => match skill_type {
                        SkillType::Auto => source.get_statistic("Auto-attack").into(),
                        SkillType::Skill => source.get_statistic("Physical Damage").into(),
                        SkillType::Spell => source.get_statistic("Magic Damage").into()
                    }
                };

                let ap:f64 = source.job.clone().map_or(0, |i| source.get_statistic(self.primary_stat(&i))).into();
                let modifier:f64 = self.level_main(&source.level).into();
                let div_modifier:f64 = self.level_div(&source.level).into();
                let sub_modifier:f64 = self.level_sub(&source.level).into();
                let ap_div:f64 = self.ap_div(&source.level).into();
                let f_wd:f64 = floor(floor(ap * modifier / 1000.0, 0) as f64 + wd, 0);
                let inter_f_ap:f64 = (ap_div * (source.get_statistic(match r#skill_type {
                    SkillType::Auto | SkillType::Skill => "Attack Power",
                    _ => "Magic Attack Power"
                }) as f64) - (modifier as f64))/(modifier + 100.0 as f64);
                let f_ap:f64 = floor(inter_f_ap, 0)/100.0;
                let inter_det:f64 = (130.0 as f64) * (source.get_statistic("DET") as f64 -modifier as f64)/div_modifier+1000.0;
                let inter_tnc:f64 = (130.0 as f64) * (source.get_statistic("TNC") as f64 -sub_modifier)/div_modifier+1000.0;
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
                let roll1:f64 = rng.gen();
                let roll2:f64 = rng.gen();
                let is_direct = (roll1- dhc) >= 0.0;
                let damage_type = match (roll2 - dhc) >= 0.0 {
                    true => AttackRoll::CriticalHit(is_direct),
                    false => AttackRoll::Hit(is_direct)
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
                let random_factor:f64 = rng.gen();
                let apply_damage = |input:f64, factor| {
                    let d:f64 = match periodic {
                        true => {
                            let d:f64 = floor(d * f_ss, 0);
                            let d:f64 = floor(d * random_factor, 0);
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
                            floor(d * random_factor, 0)
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
    use super::{Entity, AssumedDamageStrategy, SkillType, Job, DamageType, Effect};
    use std::sync::Arc;
    #[test]
    fn base_damage_checks_out() {
        let red_mage = Entity::create("red_mage".to_string(), Some(Job::RDM), 70, vec![], Arc::new(vec![]));
        let target = Entity::create("red_mage".to_string(), None, 70, vec![], Arc::new(vec![]));
        let strat = AssumedDamageStrategy::new();

        // First, we set our stats

    }
}