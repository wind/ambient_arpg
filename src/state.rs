
use ambient_api::prelude::*;

use crate::components::*;
use crate::common::*;

pub fn request_state(entity_id : EntityId, state : CharacterState) {
    entity::set_component(entity_id, requested_state(), state.0);
}

/*
trait State {
    fn enter( entity_id : EntityId);
    fn update(entity_id : EntityId) -> CharacterState;
    fn exit(entity_id : EntityId);

    fn handle_move(player_id : EntityId) -> bool {
        let direction = entity::get_component(player_id, player_movement_direction()).unwrap();
        let mouse_delta_x = entity::get_component(player_id, player_mouse_delta_x()).unwrap();

        process_movement(player_id, direction, mouse_delta_x);
        direction.length_squared() > 0.
    }
}

struct IdleState;
struct MovingState;
struct AttackingState;
struct DyingState;

impl State for IdleState {
    fn enter(entity_id : EntityId) {
        play_animation(entity_id, Animation::Idle02);
    }

    fn update(entity_id : EntityId) -> CharacterState {
        let is_moving = Self::handle_move(entity_id);
        if is_moving {
            CharacterState::Moving
        }else {
            CharacterState::Idle
        }
    }

    fn exit(entity_id : EntityId) {

    }
}

impl State for MovingState {
    fn enter(entity_id : EntityId) {

    }

    fn update(entity_id : EntityId) -> CharacterState {
        let is_moving = Self::handle_move(entity_id);
        if is_moving {
            CharacterState::Moving
        }else {
            CharacterState::Idle
        }
    }

    fn exit(entity_id : EntityId) {

    }
}

impl State for AttackingState {
    fn enter(entity_id : EntityId) {

    }

    fn update(entity_id : EntityId) -> CharacterState {
        CharacterState::Idle
    }

    fn exit(entity_id : EntityId) {

    }
}

impl State for DyingState {
    fn enter(entity_id : EntityId) {

    }

    fn update(entity_id : EntityId) -> CharacterState {
        CharacterState::Idle
    }

    fn exit(entity_id : EntityId) {

    }
}

macro_rules! gen_state_match {
    ($x:ident, $func:ident, $entity_id:ident) => {
        match $x {
            CharacterState::Idle => IdleState::$func($entity_id),
            CharacterState::Moving => MovingState::$func($entity_id),
            _ => unreachable!(),
        }
    }
}

fn process_state(entity_id : EntityId) {
    let state = entity::get_component(entity_id, player_state()).map(|x| CharacterState::from(x)).unwrap();
    if state == CharacterState::Count {
        return;
    }

    let next_state = gen_state_match!(state, update, entity_id);

    if next_state != state {
        gen_state_match!(state, exit, entity_id);
        gen_state_match!(next_state, enter, entity_id);
        entity::set_component(entity_id, player_state(), next_state as u8);
    }
}
*/

