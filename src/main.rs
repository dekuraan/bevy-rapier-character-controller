use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_mod_wanderlust::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WanderlustPlugin)
        .add_startup_system(setup)
        .add_system(input)
        .add_system(mouse_look)
        .run();
}
#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands) {
    // floor
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(25.0, 0.5, 25.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    let controller_bundle = CharacterControllerBundle {
        transform: Transform::from_translation([0.0, 5.0, 0.0].into()),
        ..default()
    };
    // character
    commands
        .spawn_bundle(controller_bundle)
        .insert(PlayerBody)
        .with_children(|parent| {
            parent
                .spawn_bundle(TransformBundle::default())
                .insert(PlayerCamera)
                .with_children(|parent| {
                    parent.spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 10.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..Default::default()
                    });
                });
        });
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
    // sensitivity: Res<Sensitivity>,
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
