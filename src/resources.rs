use crate::config::NUM_TYPES;

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
#[derive(bevy::prelude::Resource)]
pub struct SimState {
    pub paused: bool,
}

impl Default for SimState {
    fn default() -> Self {
        Self { paused: false }
    }
}

/// Событие — команда на сброс позиций всех частиц.
/// Используем Event вместо флага в SimState, потому что
/// событие живёт ровно один кадр и не требует ручной очистки.
#[derive(bevy::prelude::Message)]
pub struct ResetParticlesEvent;
