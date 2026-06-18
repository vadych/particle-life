//! Минималистичный HUD — подсказка по клавишам в углу экрана.
//! Используем Bevy UI: один текстовый узел, никаких зависимостей.

use crate::resources::SimState;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_pause_indicator)
            .add_systems(Update, update_fps_indicator);
    }
}

/// Маркер — чтобы система update_pause_indicator нашла нужный текст
#[derive(Component)]
struct PauseIndicator;

/// Маркер — текстовый узел счётчика FPS в правом нижнем углу.
#[derive(Component)]
struct FpsIndicator;

/// Создаём текстовый узел один раз при старте
fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/arial.ttf");
    commands
        .spawn(Node {
            // Прижимаем к левому нижнему углу
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            left: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|parent| {
            // Статичная подсказка
            parent.spawn((
                Text::new("Space — пауза    R — рандомизация матрицы    T — перезапуск позиций   S — показать/скрыть настройки"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
            ));

            // Индикатор паузы — меняется динамически
            parent.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.8, 0.2)),
                PauseIndicator,
            ));
        });
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            right: Val::Px(16.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
                FpsIndicator,
            ));
        });
}

/// Обновляет индикатор паузы каждый кадр
fn update_pause_indicator(state: Res<SimState>, mut query: Query<&mut Text, With<PauseIndicator>>) {
    // Беvy не вызывает эту систему если SimState не менялся —
    // но мы всё равно проверяем is_changed() для явности
    if !state.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        *text = if state.paused {
            Text::new("|| ПАУЗА")
        } else {
            Text::new("")
        };
    }
}

/// Обновляет счётчик FPS каждый кадр.
///
/// Берёт сглаженное значение из [`DiagnosticsStore`].
/// Цвет меняется в зависимости от значения:
/// - 50+ FPS → белый
/// - 30–50  → жёлтый  
/// - < 30   → красный
fn update_fps_indicator(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &mut TextColor), With<FpsIndicator>>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);

    let color = if fps >= 50.0 {
        Color::srgba(1.0, 1.0, 1.0, 0.5)
    } else if fps >= 30.0 {
        Color::srgb(1.0, 0.8, 0.2)
    } else {
        Color::srgb(1.0, 0.3, 0.3)
    };

    for (mut text, mut text_color) in query.iter_mut() {
        *text = Text::new(format!("FPS: {:.0}", fps));
        text_color.0 = color;
    }
}
