use ambient_api::{
    //message::client::{MessageExt, Target},
    player::KeyCode,
    prelude::*,
};

#[main]
pub fn main() {
    ambient_api::messages::Frame::subscribe(move |_| {
        let (delta, pressed) = player::get_raw_input_delta();

        if delta.keys.contains(&KeyCode::Key1) {
            messages::SetController::new(1u32).send_server_reliable();
        }

        if delta.keys.contains(&KeyCode::Key2) {
            messages::SetController::new(2u32).send_server_reliable();
        }

        if delta.keys.contains(&KeyCode::Key3) {
            messages::SetController::new(3u32).send_server_reliable();
        }

        let mut displace = Vec2::ZERO;
        if pressed.keys.contains(&KeyCode::W) {
            displace.y += 1.0;
        }
        if pressed.keys.contains(&KeyCode::S) {
            displace.y -= 1.0;
        }
        if pressed.keys.contains(&KeyCode::A) {
            displace.x += 1.0;
        }
        if pressed.keys.contains(&KeyCode::D) {
            displace.x -= 1.0;
        }

        messages::Input::new(displace, delta.mouse_position.x).send_server_reliable();
    });
}
