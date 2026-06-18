/// Конфигурационный файл
/// Сколько типов частиц (цветов)
pub const NUM_TYPES: usize = 6;

/// Сколько частиц каждого типа спавним
pub const PARTICLES_PER_TYPE: usize = 100;

/// Размер мира (полуширина/полувысота)
pub const WORLD_SIZE: f32 = 800.0;

/// Визуальный радиус одной частицы
pub const PARTICLE_RADIUS: f32 = 3.0;

/// Цвета для каждого типа (RGB)
pub const PARTICLE_COLORS: [bevy::prelude::Color; NUM_TYPES] = [
    bevy::prelude::Color::srgb(1.0, 0.3, 0.3), // красный
    bevy::prelude::Color::srgb(0.3, 1.0, 0.3), // зелёный
    bevy::prelude::Color::srgb(0.3, 0.5, 1.0), // синий
    bevy::prelude::Color::srgb(1.0, 1.0, 0.3), // жёлтый
    bevy::prelude::Color::srgb(1.0, 0.5, 0.1), // оранжевый
    bevy::prelude::Color::srgb(0.8, 0.3, 1.0), // фиолетовый
];

/// Радиус взаимодействия — частицы дальше этого расстояния игнорируют друг друга
pub const INTERACTION_RADIUS: f32 = 80.0;

/// Трение — насколько быстро гасится скорость (0.0 = нет трения, 1.0 = мгновенная остановка)
pub const FRICTION: f32 = 0.05;

/// Масштаб сил взаимодействия.
/// Умножается на вычисленную силу перед добавлением к скорости.
pub const FORCE_SCALE: f32 = 0.5;
