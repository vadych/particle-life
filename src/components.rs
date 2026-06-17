use bevy::prelude::*;

// Тип частицы — индекс в матрице взаимодействий и массиве цветов
#[derive(Component)]
pub struct ParticleType(pub usize);

// Скорость частицы — отдельно от Transform,
// чтобы физика работала с ней независимо
#[derive(Component)]
pub struct Velocity(pub Vec2);
