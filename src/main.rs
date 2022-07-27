use bevy::prelude::*;
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // floor
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 0.5, 5.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    let controller_bundle = CharacterControllerBundle {
        transform: Transform::from_translation([0.0, 5.0, 0.0].into()),
        // settings: ControllerSettings {
        //     force_scale: [1.0, 0.0, 1.0].into(),
        //     ..default()
        // },
        ..default()
    };

    commands.spawn_bundle(controller_bundle);
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
