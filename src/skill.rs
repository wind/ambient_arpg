
use ambient_api::prelude::*;
use ambient_api::components::core::transform::{lookat_center, rotation, scale, translation};
use ambient_api::components::core::player::{player, user_id};

use crate::components::*;
use crate::common::*;
use crate::state::*;
use crate::unit::*;

pub enum SkillEventDef {
    AttackCircle { radius : f32 },
    HealMp { value : u32 },
    HealHp { value : u32 },
}


pub struct SkillEvent {
    time : f32,
    def : SkillEventDef,
}

pub struct Skill {
    pub time : f32,
    pub cost_hp : u32,
    pub cost_mp : u32,
    pub anim : Animation,
    events : Vec<SkillEvent>,
}

fn process_damage(attack_stats : &UnitStats, defense_stats : &mut UnitStats) {
    let damage = if attack_stats[Stats::Attack] > defense_stats[Stats::Defense] { attack_stats[Stats::Attack] - defense_stats[Stats::Defense] } else {1};
    //let mut rng = rand::thread_rng();
    //rng.gen_range(damage/2..damage);

    if defense_stats[Stats::Hp] > damage {
        defense_stats[Stats::Hp] -= damage;
    }else {
        defense_stats[Stats::Hp] = 0;
    }

    println!("damage:{} left:{}", damage, defense_stats[Stats::Hp]);
}

pub fn get_target(entity_id : EntityId) -> EntityId {
    let uni_objects_query = query((stats())).build();
    let unit_objects = uni_objects_query.evaluate();
    unit_objects.iter().filter(|(id, (s))| *id == entity_id).next().map_or(EntityId::null(), |x| x.0)
}

fn process_circle_targets(entity_id : EntityId, center : Vec3, radius : f32, proc : impl Fn(&mut UnitStats)) {
    let uni_objects_query = query((translation(), stats())).build();
    let unit_objects = uni_objects_query.evaluate();

    for (id, (_, s)) in unit_objects.iter().filter(|(id, (trans, s))| *id != entity_id && s[Stats::Hp] > 0 && trans.distance_squared(center) < radius * radius ) {
        entity::mutate_component(*id, stats(), |x| {
            proc(x);
        });
        request_state(*id, CharacterState::GetHit);
        if !entity::has_component(*id, player()) {
            request_ai_target(*id, entity_id);
        }
    }
}

impl Skill {
    pub fn execute(&self, entity_id : EntityId, stats : &UnitStats, time: f32, last_time : f32) {
        for event in self.events.iter() {
            if event.time >= last_time && event.time < time {
                //let attack_stats = entity::get_component(entity_id, stats()).unwrap();
                match event.def {
                    SkillEventDef::AttackCircle{ radius } => {
                        println!("AttackCircle:{}", entity_id);
                        let center = entity::get_component(entity_id, translation()).unwrap();
                        process_circle_targets(entity_id, center, radius, |defense_stats| {
                            process_damage(stats, defense_stats);
                        });
                    },
                    SkillEventDef::HealHp { value } => {
                        let mut hp = stats[Stats::Hp] + value;
                        if hp > stats[Stats::Hp] {
                            hp = stats[Stats::MaxHp];
                        }
                        if hp != stats[Stats::Hp] {
                            entity::mutate_component(entity_id, crate::components::stats(), |x| x[Stats::Hp] = hp );
                        }
                    },
                    SkillEventDef::HealMp { value } => {
                        let mut mp = stats[Stats::Mp] + value;
                        if mp > stats[Stats::Mp] {
                            mp = stats[Stats::MaxMp];
                        }
                        if mp != stats[Stats::Mp] {
                            entity::mutate_component(entity_id, crate::components::stats(), |x| x[Stats::Mp] = mp );
                        }
                    },
                }
            }
        }
    }
}

pub struct SkillManager {
    skills : std::collections::HashMap<u32, Skill>,
}

impl SkillManager {
    pub fn new() -> Self {
        let mut skills = std::collections::HashMap::new();

        //TODO: load from config file
        skills.insert(1, Skill {
            time : 1.,
            cost_mp : 10,
            cost_hp : 0,
            anim : Animation::Attack01,
            events : vec!(
                SkillEvent {
                    time : 0.3,
                    def : SkillEventDef::AttackCircle { radius : 2. }
                }
            )
        });

        skills.insert(2, Skill {
            time : 0.5,
            cost_mp : 10,
            cost_hp : 0,
            anim : Animation::DefendStart,
            events : vec!(
                SkillEvent {
                    time : 0.,
                    def : SkillEventDef::HealHp { value : 100 }
                }
            )
        });

        skills.insert(3, Skill {
            time : 1.,
            cost_mp : 10,
            cost_hp : 0,
            anim : Animation::Attack02Start,
            events : vec!(
                SkillEvent {
                    time : 0.3,
                    def : SkillEventDef::AttackCircle { radius : 2. }
                }
            )
        });

        skills.insert(4, Skill {
            time : 1.,
            cost_mp : 10,
            cost_hp : 0,
            anim : Animation::Attack04,
            events : vec!(
                SkillEvent {
                    time : 0.3,
                    def : SkillEventDef::AttackCircle { radius : 2. }
                }
            )
        });

        Self {
            skills,
        }
    }

    pub fn get(&self, id : u32) -> Option<&Skill> {
        self.skills.get(&id)
    }
}

