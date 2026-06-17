//! Обработка пользовательского ввода.
//!
//! Горячие клавиши:
//!   Space — пауза / продолжить
//!   R     — рандомизировать матрицу и перезапустить частицы

use bevy::prelude::*;

use crate::resources::{InteractionMatrix, ResetParticlesEvent, SimState};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard);
    }
}

/// Читает нажатия клавиш и меняет состояние симуляции.
///
/// Используем `just_pressed` а не `pressed` —
/// иначе пауза будет включаться/выключаться каждый кадр пока зажата клавиша.
fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut sim_state: ResMut<SimState>,
    mut matrix: ResMut<InteractionMatrix>,
    mut reset_event: MessageWriter<ResetParticlesEvent>,
) {
    // ── Пауза ────────────────────────────────────────────────────────────────
    if keys.just_pressed(KeyCode::Space) {
        sim_state.paused = !sim_state.paused;

        // Небольшая отладка — видно в консоли запущена ли симуляция
        if sim_state.paused {
            info!("Симуляция на паузе");
        } else {
            info!("Симуляция продолжается");
        }
    }

    // ── Рандомизация матрицы ──────────────────────────────────────────────────
    if keys.just_pressed(KeyCode::KeyR) {
        matrix.randomize();
        info!("Матрица рандомизирована");
    }

    //-- Меняем позиции частиц на новые случайные при нажатии T ───────────────────────────────

    if keys.just_pressed(KeyCode::KeyT) {
        // Отправляем событие — система в particles.rs его поймает
        // и переставит все частицы на случайные позиции
        reset_event.write(ResetParticlesEvent);
        info!("Позиции частиц перезапущены");
    }

    if keys.just_pressed(KeyCode::KeyS) {
        sim_state.show_matrix_editor = !sim_state.show_matrix_editor;
    }
}
