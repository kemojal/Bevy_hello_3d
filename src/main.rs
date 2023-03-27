use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;
// use std::time::Duration;

mod constants;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
}



#[derive(Component)]
struct SpawnCubeTimer;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
struct Velocity(Vec3);
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Tower>()
    .register_type::<Lifetime>()
    .add_startup_system(setup)
    .add_system(tower_shooting)
    // .add_system(move_cubes)
    .add_system(move_bullets)
    .add_system(bullet_despawn)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    }).insert(Name::new("Ground"));
    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert(Name::new("Tower"));

}



fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

                commands.spawn((Box, PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
                    material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                }))
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"))
                .insert(Velocity(Vec3::new(10.0, 0.0, 0.0)));
        }
    }
}

// fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
//     for (bullet, mut transform) in &mut bullets {
//         transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
//     }
// }

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// fn spawn_cube(commands: &mut Commands, 
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>) {
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 0.250 })),
//         material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..Default::default()
//     });
// }

pub fn move_cubes(time: Res<Time>, mut query: Query<(&mut Box, &mut Transform)>){
    // let mut rng = rand::thread_rng();

    for (mut block, mut trans) in query.iter_mut() {
        trans.translation.x -= 0.05;
        // trans.rotation.x -= 0.05;
        // let rotation = trans.rotation;
        // let new_rotation = rotation * Quat::from_rotation_z(time.delta_seconds()*rng.gen_range(0.0..0.01));
        // trans.rotation = new_rotation;
        // if trans.translation.y < -6. {
        //     trans.translation.y = rng.gen_range(6.0..9.0);
        // }

    }
}

fn move_bullets(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Name>>)
{
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds(); // Update the position based on the velocity and time
    }
}
