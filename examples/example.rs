use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_mod_wanderlust::*;
use bevy_rapier3d::prelude::*;
use char_controller::{CharacterControllerPlugin, PlayerBody, PlayerCamera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CharacterControllerPlugin)
        .add_startup_system(setup)
        .run();
}

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
                    parent.spawn_bundle(Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 10.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..Default::default()
                    });
                });
        });
}
