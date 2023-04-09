#![allow(unused)]

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        rendering::color,
        transform::{lookat_center, rotation, scale, translation},
        player::{player, user_id},
        primitives::{cube, quad},
        physics::{
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider, sphere_collider, visualizing,
        },
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    entity::{AnimationAction, AnimationController},
    //message::server::{MessageExt, Source},
    prelude::*,
    rand,
};


use components::*;

mod skill;
use skill::*;
mod common;
use common::*;
mod state;
use state::*;

pub fn play_animation(unit_id : EntityId, anim : Animation, looping : bool) {
    entity::set_animation_controller(
        unit_id,
        AnimationController {
            actions: &[AnimationAction {
                clip_url: &asset::url(anim.path()).unwrap(),
                looping: looping,
                weight: 1.,
            }],
            apply_base_pose: false,
        },
    );
}

pub fn play_animations(unit_id : EntityId, actions : &[AnimationAction] ) {
    entity::set_animation_controller(
        unit_id,
        AnimationController {
            actions: actions,
            apply_base_pose: false,
        },
    );
}

pub struct Context {
    skill_manager : SkillManager,
}

fn process_skill(context : &Context, entity_id : EntityId, stats : &UnitStats, skill_time : f32) {
    if let Some(skill_id) = entity::get_component(entity_id, requested_skill()) {
        if skill_id != 0 {
            if let Some(skill) = context.skill_manager.get(skill_id) {
                skill.execute(entity_id, stats, skill_time + frametime(), skill_time);
            }
        }
    }
}

fn process_state(context : &Context, entity_id : EntityId, stats : &UnitStats, req_state : CharacterState, state : CharacterState, state_time : f32, direction : Vec2) -> CharacterState{
    entity::set_component(entity_id, player_state_time(), state_time + frametime());

    let mut next_state = state;
    let mut state_time = state_time;

    if req_state != CharacterState::Count {
        next_state = req_state;
        state_time = 0.;
        println!("req state:{} {} {}", entity_id, state as u8, req_state as u8);
        request_state(entity_id, CharacterState::Count);
    }

    if next_state == CharacterState::Dying {
        return state;
    }

    if stats[Stats::Hp] == 0 {
        next_state = CharacterState::Dying;
    }

    if next_state == CharacterState::GetHit {
        let hit_time = 1.0;
        if state_time > hit_time {
            next_state = CharacterState::Idle;
        }
    }

    if next_state == CharacterState::Attacking {
        let skill_time = 1.0;
        if state_time > skill_time {
            entity::set_component(entity_id, requested_skill(), 0);
            next_state = CharacterState::Idle;
        }
    }else if let Some(req_skill) = entity::get_component(entity_id, requested_skill())  {
            if req_skill != 0 {
                next_state = CharacterState::Attacking;
            }else {
                //next_state = CharacterState::Idle;
            }
    }

    if next_state.movable() {
        if direction.length_squared() > 0. {
            next_state = CharacterState::Moving;
        }
        else {
            next_state = CharacterState::Idle;
        }
    }

    if next_state != state {
        println!("change state:{} {}", entity_id, next_state as u8);
        //entity::set_component(entity_id, player_state_last(), state as u8);
        entity::set_component(entity_id, player_state(), next_state as u8);
        entity::set_component(entity_id, player_state_time(), 0.);

        match next_state {
            CharacterState::Idle => play_animation(entity_id, Animation::Idle02, true),
            CharacterState::Moving => {
                let anim1 = if direction.y > 0. { Animation::BattleWalkForward } else { Animation::BattleWalkBack };
                let anim2 = if direction.x > 0. { Animation::BattleWalkLeft } else { Animation::BattleWalkRight };
                play_animations(entity_id, &[
                    AnimationAction {
                        clip_url: &asset::url(anim1.path()).unwrap(),
                        looping: true,
                        weight: direction.y.abs(),
                    },
                    AnimationAction {
                        clip_url: &asset::url(anim2.path()).unwrap(),
                        looping: true,
                        weight: direction.x.abs(),
                    },
                ]);
            },
            CharacterState::Attacking => {
                play_animation(entity_id, Animation::Attack01, false);
            },
            CharacterState::GetHit => {
                play_animation(entity_id, Animation::GetHit, false);
            },
            CharacterState::Dying => {
                play_animation(entity_id, Animation::Die, false);
            },
            _ => unreachable!(),
        }
    }

    next_state
}

fn process_movement(entity_id : EntityId, state : CharacterState, direction : Vec2, mouse_delta_x : f32) {
    if !state.movable() {
        return;
    }

    let rot = entity::mutate_component(entity_id, rotation(), |x| {
        *x *= Quat::from_rotation_z(mouse_delta_x * 0.01)
    })
    .unwrap_or_default();

    let speed = 0.05;
    let displace = rot * (direction.normalize_or_zero() * speed).extend(-0.1);
    physics::move_character(entity_id, displace, 0.01, frametime());
}

fn spawn_player(id : EntityId, user: String) {
    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(user_id(), user)
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    //init skill
    let mut player_skills : Vec<u32> = Vec::new();
    player_skills.push(1);
    player_skills.push(2);
    player_skills.push(3);
    player_skills.push(4);

    let mut unit_stats : UnitStats = Vec::new();
    unit_stats.resize(Stats::Count, 0);

    unit_stats[Stats::MaxHp] = 200;
    unit_stats[Stats::MaxMp] = 100;
    unit_stats[Stats::Attack] = 50;
    unit_stats[Stats::Defense] = 20;
    unit_stats[Stats::Hp] = unit_stats[Stats::MaxHp];
    unit_stats[Stats::Mp] =  unit_stats[Stats::MaxMp];

    entity::add_components(
        id,
        Entity::new()
            .with_merge(make_transformable())
            //.with_default(cube())
            .with(
                prefab_from_url(),
                asset::url("assets/WizardPolyArt/Mesh/PolyArtWizardMesh.fbx").unwrap(),
            )
            .with(translation(), vec3(0., 0., 0.))
            .with(skills(), player_skills)
            .with(stats(), unit_stats)
            .with(requested_state(), CharacterState::Count as u8)
            .with(player_state(), CharacterState::Idle as u8)
            .with(player_state_time(), 0.)
            .with(player_camera_ref(), camera)
            //.with(color(), rand::random::<Vec3>().extend(1.0))
            .with(character_controller_height(), 2.)
            .with(character_controller_radius(), 0.5)
            .with_default(physics_controlled())
            //.with_default(visualizing()),
    );
}


fn spawn_npc(id : u32) {
    //TODO:init from config id
    let pos = match id {
        0 => vec3(5., 0., 0.),
        1 => vec3(-5., 0., 0.),
        2 => vec3(0., 5., 0.),
        _ => rand::random::<Vec3>(),
    };

    //init skill
    let mut unit_skills : Vec<u32> = Vec::new();
    unit_skills.push(1);

    let mut unit_stats : UnitStats = Vec::new();
    unit_stats.resize(Stats::Count, 0);

    unit_stats[Stats::MaxHp] = 200;
    unit_stats[Stats::MaxMp] = 100;
    unit_stats[Stats::Attack] = 50;
    unit_stats[Stats::Defense] = 20;
    unit_stats[Stats::Hp] = unit_stats[Stats::MaxHp];
    unit_stats[Stats::Mp] =  unit_stats[Stats::MaxMp];

    let entity_id = Entity::new()
        .with_merge(make_transformable())
        //.with_default(cube())
        .with(
            prefab_from_url(),
            asset::url("assets/WizardPolyArt/Mesh/PolyArtWizardMesh.fbx").unwrap(),
        )
        .with(player_movement_direction(), vec2(0., 0.))
        .with(player_mouse_delta_x(), 0.)
        .with(translation(), pos)
        .with(skills(), unit_skills)
        .with(stats(), unit_stats)
        .with(requested_state(), CharacterState::Count as u8)
        .with(player_state(), CharacterState::Idle as u8)
        .with(player_state_time(), 0.)
        //.with(color(), rand::random::<Vec3>().extend(1.0))
        .with(character_controller_height(), 2.)
        .with(character_controller_radius(), 0.5)
        .with_default(physics_controlled())
        //.with_default(visualizing()),
        .spawn();

    play_animation(entity_id, Animation::Idle02, true);
}

#[main]
pub fn main() {
    let context = Context {
        skill_manager : SkillManager::new(),
    };

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 10.)
        .with(color(), vec4(0.5, 0.5, 0.5, 1.))
        .with(translation(), vec3(0., 0., 0.0))
        .with_default(plane_collider())
        .spawn();

    spawn_npc(0);
    spawn_npc(1);
    spawn_npc(2);
    spawn_npc(3);

    spawn_query((player(), user_id())).bind(move |players| {
        for (id, (_, user)) in players {
            spawn_player(id, user);
            //play_animation(id, Animation::Idle02);
        }
    });

    let unit_id = Entity::new()
        .with_merge(make_transformable())
        //.with_default(cube())
        .with(
            prefab_from_url(),
            asset::url("assets/WizardPolyArt/Mesh/PolyArtWizardMesh.fbx").unwrap(),
        )
        .with(translation(), vec3(2., 2., 0.0))
        .spawn();

    let mut anim = Animation::Idle01;
    play_animations(unit_id, &[
        AnimationAction {
            clip_url: &asset::url(Animation::BattleWalkForward.path()).unwrap(),
            looping: true,
            weight: 1.,
        },
        AnimationAction {
            clip_url: &asset::url(Animation::BattleWalkLeft.path()).unwrap(),
            looping: true,
            weight: 1.,
        },
    ]);

    query((
        player(),
        player_movement_direction(),
        player_mouse_delta_x(),
    ))
    .each_frame(move |players| {
        for (player_id, _) in players {
            //process_state(player_id);
        }
    });

    query((
        stats(),
        requested_state(),
        player_state(),
        player_state_time(),
        player_movement_direction(),
        player_mouse_delta_x(),
        rotation(),
    ))
    .each_frame(move |players| {
        for (player_id, (stats, req_state, state, state_time, direction, mouse_delta_x, rot)) in players {
            let state = process_state(&context, player_id, &stats, CharacterState::from(req_state), CharacterState::from(state), state_time, direction);

            if state == CharacterState::Attacking {
                process_skill(&context, player_id, &stats, state_time);
            }

            process_movement(player_id, state, direction, mouse_delta_x);
        }
    });

    query((
        player(),
        player_camera_ref(),
        translation(),
        rotation(),
    ))
    .each_frame(move |players| {
        for (player_id, (_, camera_id, pos, rot)) in players {

            let forward = rot * Vec3::Y;
            entity::set_component(camera_id, lookat_center(), pos);
            entity::set_component(camera_id, translation(), pos - forward * 4. + Vec3::Z * 2.);
        }
    });

    // When a player despawns, clean up their objects.
    //let player_objects_query = query(user_id()).build();
    despawn_query(user_id()).requires(player()).bind({
        move |players| {
            //let player_objects = player_objects_query.evaluate();
            for (id, player_user_id) in &players {
               // for (id, _) in player_objects
                  //  .iter()
                   // .filter(|(_, object_user_id)| *player_user_id == *object_user_id)
                //{
                    entity::despawn(*id);
                //}
            }
        }
    });

    messages::SetController::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        if msg.value == 1 {
            anim = Animation::from( (anim as u8 + 1) % Animation::Count as u8 );
            if anim != Animation::Count {
                play_animation(unit_id, anim, true);
            }
        }

        if msg.value == 2 {
            entity::add_component(player_id, requested_skill(), 1);
        }
    });

    messages::Input::subscribe(move |source, msg| {
        let Some(player_id) = source.client_entity_id() else { return; };

        entity::add_component(player_id, player_movement_direction(), msg.direction);
        entity::add_component(player_id, player_mouse_delta_x(), msg.mouse_delta_x);
    });
}
