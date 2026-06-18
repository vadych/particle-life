//! Плагин физики — сердце симуляции Particle Life.
//!
//! Каждый кадр выполняются три шага в строгом порядке:
//!   1. `apply_interactions` — считаем силы притяжения/отталкивания между частицами
//!   2. `apply_friction`     — гасим скорость, чтобы система не взрывалась
//!   3. `integrate_velocity` — двигаем частицы и оборачиваем мир тороидом

use bevy::prelude::*;

use crate::components::{ParticleType, Velocity};
use crate::config::{FORCE_SCALE, FRICTION, INTERACTION_RADIUS, WORLD_SIZE};
use crate::resources::{InteractionMatrix, SimState, SpatialGrid};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_interactions, apply_friction, integrate_velocity)
                // .chain() гарантирует что системы выполняются именно в этом порядке,
                // а не параллельно — важно, иначе velocity и position могут рассинхронизироваться
                .chain()
                // Вся цепочка систем НЕ запускается если симуляция на паузе.
                // run_if принимает замыкание, получающее ресурс из мира.
                .run_if(|state: Res<SimState>| !state.paused),
        );
    }
}

/// Главная система симуляции — вычисляет силы между каждой парой частиц.
///
/// Для каждой частицы A перебираем всех соседей B в радиусе [`INTERACTION_RADIUS`].
/// Сила делится на две зоны:
///
/// ```
/// 0.0          0.3                1.0   (нормализованное расстояние)
///  |────────────|───────────────────|
///  ^ отталкивание (жёсткое ядро)   ^ сила из матрицы (может быть + или -)
/// ```
///
/// Это имитирует поведение реальных атомов: вблизи — всегда отталкивание,
/// на средней дистанции — взаимодействие по правилам матрицы.
///
/// **Сложность:** O(n²) — каждая частица проверяет каждую.
/// При n=1000 это ~1 000 000 операций за кадр. Позже заменим на spatial grid.
pub fn apply_interactions(
    mut grid: ResMut<SpatialGrid>,
    matrix: Res<InteractionMatrix>,
    state: Res<SimState>,
    mut query: Query<(Entity, &Transform, &ParticleType, &mut Velocity)>,
) {
    if state.paused {
        return;
    }

    // Фаза 1: перестраиваем сетку
    grid.clear();
    for (entity, transform, _, _) in query.iter() {
        grid.insert(entity, transform.translation.truncate());
    }

    // Фаза 2: снимок — HashMap<Entity, (Vec2, usize)>
    // HashMap нужен чтобы искать тип соседа по Entity за O(1)
    let snapshot: std::collections::HashMap<Entity, (Vec2, usize)> = query
        .iter()
        .map(|(e, t, pt, _)| (e, (t.translation.truncate(), pt.0)))
        .collect();

    let half = WORLD_SIZE * 0.5;

    // Фаза 3: силы
    for (&entity, &(pos, type_a)) in &snapshot {
        let mut force = Vec2::ZERO;

        // query_neighbors возвращает &(Entity, Vec2) — деструктурируем как пару
        for &(neighbor_entity, neighbor_pos) in grid.query_neighbors(pos) {
            if neighbor_entity == entity {
                continue;
            }

            // Тип соседа берём из HashMap по его Entity
            let Some(&(_, type_b)) = snapshot.get(&neighbor_entity) else {
                continue;
            };

            // Тороидальное смещение
            let mut delta = neighbor_pos - pos;
            if delta.x > half {
                delta.x -= WORLD_SIZE;
            }
            if delta.x < -half {
                delta.x += WORLD_SIZE;
            }
            if delta.y > half {
                delta.y -= WORLD_SIZE;
            }
            if delta.y < -half {
                delta.y += WORLD_SIZE;
            }

            let dist = delta.length();
            if dist < 1e-6 || dist > INTERACTION_RADIUS {
                continue;
            }

            let t = dist / INTERACTION_RADIUS;
            let direction = delta / dist;

            let force_magnitude: f32 = if t < 0.3 {
                // Жёсткое ядро: всегда отталкивание
                t / 0.3 - 1.0
            } else {
                // Зона матрицы: треугольный профиль, пик посередине [0.3, 1.0]
                let strength = matrix.values[type_a][type_b];
                let zone_half = (1.0_f32 - 0.3) * 0.5; // 0.35
                let peak = 0.3_f32 + zone_half; // 0.65
                strength * (1.0 - (t - peak).abs() / zone_half)
            };

            force += direction * force_magnitude;
        }

        if let Ok((_, _, _, mut vel)) = query.get_mut(entity) {
            vel.0 += force * FORCE_SCALE;
        }
    }
}
/// Гасит скорость каждой частицы на коэффициент [`FRICTION`] каждый кадр.
///
/// Без трения система консервативна — скорости бесконечно накапливались бы.
/// Трение создаёт баланс: силы разгоняют, трение тормозит.
/// При правильном балансе возникают устойчивые структуры.
///
/// Формула: `v = v * (1 - friction)`
/// Например, при FRICTION=0.05: каждый кадр скорость умножается на 0.95
fn apply_friction(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        // Умножение вектора на скаляр < 1.0 — самый простой способ затухания.
        // Это "экспоненциальное" затухание: скорость никогда не станет ровно 0,
        // но очень быстро приближается к нему без посторонних сил.
        velocity.0 *= 1.0 - FRICTION;
    }
}

/// Применяет накопленную скорость к позиции частицы.
///
/// После обновления позиции проверяем тороидальные границы:
/// если частица вылетела за край мира — она появляется с противоположной стороны.
/// Это убирает "стеночный эффект" и создаёт ощущение бесконечного пространства.
///
/// ```
/// ┌──────────────┐       ┌──────────────┐
/// │          →→→●│  ==>  │●             │
/// └──────────────┘       └──────────────┘
/// ```
fn integrate_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        // Шаг интегрирования: pos += vel * dt
        // Мы пропускаем явный dt, потому что Bevy вызывает Update ~60 раз/сек —
        // скорость уже "откалибрована" под этот темп через коэффициент 0.5 выше.
        // TODO:Позже можно добавить Time<Fixed> для frame-rate независимости.
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;

        // ── Тороидальные границы ──────────────────────────────────────────────
        // WORLD_SIZE — полуразмер мира, мир простирается от -WORLD_SIZE до +WORLD_SIZE
        let half = WORLD_SIZE;

        // Горизонталь
        if transform.translation.x > half {
            transform.translation.x -= half * 2.0; // вылетел вправо → появляется слева
        }
        if transform.translation.x < -half {
            transform.translation.x += half * 2.0; // вылетел влево → появляется справа
        }

        // Вертикаль
        if transform.translation.y > half {
            transform.translation.y -= half * 2.0; // вылетел вверх → появляется снизу
        }
        if transform.translation.y < -half {
            transform.translation.y += half * 2.0; // вылетел вниз → появляется сверху
        }
    }
}
