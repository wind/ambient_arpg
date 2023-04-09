
use ambient_api::prelude::*;
use ambient_api::components::core::transform::{lookat_center, rotation, scale, translation};

use crate::components::*;
use crate::common::*;
use crate::state::*;

pub enum SkillEventDef {
    AttackCircle { radius : f32 },
    HealMp { mp : u32 },
    HealHp { hp : u32 },
}


pub struct SkillEvent {
    time : f32,
    def : SkillEventDef,
}

pub struct Skill {
    time : f32,
    cost_hp : u32,
    cost_mp : u32,
    anim : Animation,
    events : Vec<SkillEvent>,
}

pub type UnitStats = Vec<u32>;

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

fn process_circle_targets(entity_id : EntityId, center : Vec3, radius : f32, proc : impl Fn(&mut UnitStats)) {
    let uni_objects_query = query((translation(), stats())).build();
    let unit_objects = uni_objects_query.evaluate();

    for (id, (_, s)) in unit_objects.iter().filter(|(id, (trans, s))| *id != entity_id && trans.distance_squared(center) < radius * radius ) {
        //TODO: find a better way
        let mut s = s.clone();
        proc(&mut s);
        entity::set_component(*id, stats(), s);
        request_state(*id, CharacterState::GetHit);
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
                    SkillEventDef::HealHp { hp } => {

                    },
                    SkillEventDef::HealMp { mp } => {

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

        Self {
            skills,
        }
    }

    pub fn get(&self, id : u32) -> Option<&Skill> {
        self.skills.get(&id)
    }
}

