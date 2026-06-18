use bevy::prelude::*;
use rand;

use crate::components::{ParticleType, Velocity};
use crate::config::{NUM_TYPES, PARTICLE_COLORS, PARTICLE_RADIUS, PARTICLES_PER_TYPE, WORLD_SIZE};
use crate::resources::ResetParticlesEvent;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        // Спавним всё один раз при старте
        app.add_systems(Startup, spawn_particles);
        // Слушаем событие сброса позиций
        app.add_systems(Update, on_reset);
        app.add_message::<ResetParticlesEvent>();
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Один меш на все частицы — круг радиуса PARTICLE_RADIUS
    let circle_mesh = meshes.add(Circle::new(PARTICLE_RADIUS));

    for type_idx in 0..NUM_TYPES {
        // Один материал на каждый тип частиц
        let material = materials.add(PARTICLE_COLORS[type_idx]);

        for _ in 0..PARTICLES_PER_TYPE {
            // Случайная позиция внутри мира
            let x = rand::random_range(-WORLD_SIZE / 2.0..WORLD_SIZE / 2.0);
            let y = rand::random_range(-WORLD_SIZE / 2.0..WORLD_SIZE / 2.0);

            commands.spawn((
                Mesh2d(circle_mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(x, y, 0.0),
                ParticleType(type_idx),
                Velocity(Vec2::ZERO), // пока стоят на месте
            ));
        }
    }
}

/// Слушает ResetParticlesEvent и переставляет все частицы
/// на новые случайные позиции, обнуляя их скорости.
///
/// Мы не удаляем и не создаём частицы заново — это дорого.
/// Просто меняем Transform и Velocity у уже существующих.
fn on_reset(
    mut events: MessageReader<ResetParticlesEvent>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    // Если событий не было — выходим сразу, система вызывается каждый кадр
    if events.is_empty() {
        return;
    }
    // Помечаем все события как прочитанные
    events.clear();

    for (mut transform, mut velocity) in query.iter_mut() {
        // Новая случайная позиция внутри мира
        transform.translation.x = rand::random_range(-WORLD_SIZE / 2.0..WORLD_SIZE / 2.0);
        transform.translation.y = rand::random_range(-WORLD_SIZE / 2.0..WORLD_SIZE / 2.0);

        // Сбрасываем скорость — иначе частицы сразу улетят
        // с той скоростью что накопили до сброса
        velocity.0 = Vec2::ZERO;
    }
}
