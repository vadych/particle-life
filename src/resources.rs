use bevy::{
    ecs::{entity::Entity, resource::Resource},
    math::Vec2,
};

use crate::config::{INTERACTION_RADIUS, NUM_TYPES};
use std::collections::HashMap;

#[derive(bevy::prelude::Resource)]
pub struct InteractionMatrix {
    // matrix[a][b] = сила, с которой тип B действует на тип A
    // от -1.0 (сильное отталкивание) до 1.0 (сильное притяжение)
    pub values: [[f32; NUM_TYPES]; NUM_TYPES],
}

impl InteractionMatrix {
    /// Случайная матрица при создании симуляции
    pub fn random() -> Self {
        let mut m = Self {
            values: [[0.0; NUM_TYPES]; NUM_TYPES],
        };
        m.randomize();
        m
    }

    /// Заполняет все ячейки новыми случайными значениями [-1.0 .. 1.0].
    /// Вызывается как при инициализации, так и по клавише R.
    pub fn randomize(&mut self) {
        for row in self.values.iter_mut() {
            for val in row.iter_mut() {
                *val = rand::random_range(-1.0f32..1.0f32);
            }
        }
    }
}

/// Глобальное состояние симуляции.
/// Вынесено в отдельный ресурс чтобы любая система могла
/// проверить или изменить паузу независимо от других.
#[derive(bevy::prelude::Resource, Default)]
pub struct SimState {
    pub paused: bool,
    pub show_matrix_editor: bool,
}

/// Событие — команда на сброс позиций всех частиц.
/// Используем Event вместо флага в SimState, потому что
/// событие живёт ровно один кадр и не требует ручной очистки.
#[derive(bevy::prelude::Message)]
pub struct ResetParticlesEvent;

/// Пространственная хэш-сетка для ускорения поиска соседей.
///
/// Делит мировое пространство на ячейки размером [`cell_size`] × [`cell_size`].
/// Позволяет искать соседей в радиусе за O(1) вместо O(n),
/// проверяя только 9 соседних ячеек вместо всех частиц.
///
/// Перестраивается полностью каждый тик перед [`apply_interactions`].
#[derive(Resource)]
pub struct SpatialGrid {
    pub cells: HashMap<(i32, i32), Vec<(Entity, Vec2)>>,
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn clear(&mut self) {
        self.cells.clear();
    }
    /// Вставляет сущность с позицией в соответствующую ячейку.
    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let key = self.cell_key(pos);
        self.cells.entry(key).or_default().push((entity, pos));
    }

    pub fn cell_key(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    /// Возвращает итератор по всем записям в 9 ячейках вокруг `pos`.
    ///
    /// Включает саму частицу — фильтровать на стороне вызывающего.
    pub fn query_neighbors(&self, pos: Vec2) -> impl Iterator<Item = &(Entity, Vec2)> {
        let (cx, cy) = self.cell_key(pos);
        let cells = &self.cells;
        (-1i32..=1)
            .flat_map(move |dx| (-1i32..=1).map(move |dy| (cx + dx, cy + dy)))
            .filter_map(|key| cells.get(&key))
            .flatten()
    }
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            cell_size: INTERACTION_RADIUS,
        }
    }
}
// pub fn new(...) — удалить
