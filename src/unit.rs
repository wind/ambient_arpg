use ambient_api::prelude::*;
use ambient_api::components::core::transform::{lookat_center, rotation, scale, translation};

use crate::components::*;
use crate::common::*;
use crate::state::*;
use crate::skill::*;

pub type UnitStats = Vec<u32>;
pub type UnitSkills = Vec<u32>;

pub struct Unit {
    pub id : u32,
    pub reborn_time : f32,
    pub model : String,
    pub anim_path : String,
    pub stats : UnitStats,
    pub skills : UnitSkills,
}

impl Unit {
    pub fn get_anim_path(&self, anim : Animation) -> String {
        //format!(self.anim_path, anim.name())
        self.anim_path.replace("{}", anim.name())
    }
}


pub struct UnitManager {
    units : std::collections::HashMap<u32, Unit>,
}

impl UnitManager {
    pub fn new() -> Self {
        let mut units = std::collections::HashMap::new();

        //TODO: load from config file
        {
            let mut skills : UnitSkills = UnitSkills::new();
            skills.push(1);
            skills.push(2);
            skills.push(3);
            skills.push(4);

            let mut stats : UnitStats = UnitStats::new();
            stats.resize(Stats::Count, 0);
            stats[Stats::MaxHp] = 200;
            stats[Stats::MaxMp] = 100;
            stats[Stats::Attack] = 50;
            stats[Stats::Defense] = 20;
            stats[Stats::Hp] = stats[Stats::MaxHp];
            stats[Stats::Mp] =  stats[Stats::MaxMp];

            let model = "assets/WizardPolyArt/Mesh/PolyArtWizardMesh.fbx";
            let anim_path = "assets/WizardPolyArt/Animations/{}.fbx/animations/Take 001.anim";

            let id = 1;
            units.insert(id, Unit {
                id : id,
                reborn_time : 5.0,
                model : model.to_string(),
                anim_path : anim_path.to_string(),
                stats : stats,
                skills : skills,
            });
        }

        {
            let mut skills : UnitSkills = UnitSkills::new();
            skills.push(1);

            let mut stats : UnitStats = UnitStats::new();
            stats.resize(Stats::Count, 0);
            stats[Stats::MaxHp] = 200;
            stats[Stats::MaxMp] = 100;
            stats[Stats::Attack] = 50;
            stats[Stats::Defense] = 20;
            stats[Stats::Hp] = stats[Stats::MaxHp];
            stats[Stats::Mp] =  stats[Stats::MaxMp];

            let model = "assets/WizardPolyArt/Mesh/PolyArtWizardMesh.fbx";
            let anim_path = "assets/WizardPolyArt/Animations/{}.fbx/animations/Take 001.anim";

            let id = 2;
            units.insert(id, Unit {
                id : id,
                reborn_time : 5.0,
                model : model.to_string(),
                anim_path : anim_path.to_string(),
                stats : stats,
                skills : skills,
            });
        }

        Self {
            units,
        }
    }

    pub fn get(&self, id : u32) -> Option<&Unit> {
        self.units.get(&id)
    }
}

pub fn request_ai_target(entity_id : EntityId, target : EntityId) {
    entity::add_component(entity_id, ai_target(), target);
}

pub fn approach_target(entity_id : EntityId, target_id : EntityId) -> bool {
    let pos = entity::get_component(entity_id, translation()).unwrap();
    let target_pos = entity::get_component(target_id, translation()).unwrap();

    if pos.distance_squared(target_pos) < 2. * 2. {
        entity::add_component(entity_id, player_movement_direction(), vec2(0., 0.));
        return true;
    }

    let dir = (target_pos - pos).normalize();
    let rot = Quat::from_rotation_z(-dir.x.atan2(dir.y));
    entity::set_component(entity_id, rotation(), rot);
    entity::add_component(entity_id, player_movement_direction(), Vec2::Y);
    true
}


pub fn do_skill(entity_id : EntityId, target_id : EntityId, skill_id : u32) -> bool {

    let pos = entity::get_component(entity_id, translation()).unwrap();
    let target_pos = entity::get_component(target_id, translation()).unwrap();

    if pos.distance_squared(target_pos) > 2. * 2. {
        //entity::add_component(entity_id, player_movement_direction(), vec2(0., 0.));
        return false;
    }

    entity::add_component(entity_id, requested_skill(), skill_id);
    true
}