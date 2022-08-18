use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_mod_wanderlust::{ControllerInput, WanderlustPlugin};

#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerCamera;

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WanderlustPlugin)
            .add_system(input)
            .add_system(mouse_look);
    }
}

fn input(mut body: Query<(&mut ControllerInput, &GlobalTransform)>, input: Res<Input<KeyCode>>) {
    let (mut body, tf) = body.single_mut();
    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::D) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.forward();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.forward();
    }
    body.jumping = input.pressed(KeyCode::Space);

    body.movement = dir;
}

fn mouse_look(
    mut body: Query<&mut Transform, With<PlayerBody>>,
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<PlayerBody>)>,
    mut input: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut body_tf = body.single_mut();

    let mut cam_tf = camera.single_mut();

    let dt = time.delta_seconds();
    let sens = 0.5;

    for motion in input.iter() {
        // Vertical
        let rot = cam_tf.rotation;
        cam_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::X * -motion.delta.y * dt * sens,
        ));

        // Horizontal
        let rot = body_tf.rotation;
        body_tf.rotate(Quat::from_scaled_axis(
            rot * Vec3::Y * -motion.delta.x * dt * sens,
        ));
    }
}
