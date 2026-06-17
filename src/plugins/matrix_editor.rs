use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui}; // <- EguiPrimaryContextPass

use crate::config::{NUM_TYPES, PARTICLE_COLORS};
use crate::resources::{InteractionMatrix, SimState};

pub struct MatrixEditorPlugin;

impl Plugin for MatrixEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass, // <- вместо Update
            matrix_editor_ui.run_if(|state: Res<SimState>| state.show_matrix_editor),
        );
    }
}

fn bevy_color_to_egui(color: Color) -> egui::Color32 {
    let c = color.to_srgba();
    egui::Color32::from_rgb(
        (c.red * 255.0) as u8,
        (c.green * 255.0) as u8,
        (c.blue * 255.0) as u8,
    )
}

// Функция теперь возвращает Result — требование нового API
fn matrix_editor_ui(mut contexts: EguiContexts, mut matrix: ResMut<InteractionMatrix>) -> Result {
    // <- добавили -> Result
    egui::Window::new("Matrix")
        .resizable(false)
        .show(contexts.ctx_mut()?, |ui| {
            // Вместо блока с ui.horizontal для заголовков используем Grid
            // чтобы колонки автоматически совпали с колонками слайдеров
            egui::Grid::new("matrix_grid")
                .spacing([4.0, 4.0])
                .show(ui, |ui| {
                    // Первая строка — заголовки столбцов
                    ui.label(""); // пустая ячейка под подпись строки
                    for type_idx in 0..NUM_TYPES {
                        let color = bevy_color_to_egui(PARTICLE_COLORS[type_idx]);
                        ui.colored_label(color, format!("  {type_idx}  "));
                    }
                    ui.end_row();

                    // Строки матрицы
                    for type_a in 0..NUM_TYPES {
                        let color = bevy_color_to_egui(PARTICLE_COLORS[type_a]);
                        ui.colored_label(color, format!("{type_a}"));

                        for type_b in 0..NUM_TYPES {
                            let color_a = bevy_color_to_egui(PARTICLE_COLORS[type_a]); // получатель — головка
                            let color_b = bevy_color_to_egui(PARTICLE_COLORS[type_b]); // источник — трек

                            // Головка и рамка — цвет A
                            ui.visuals_mut().widgets.inactive.fg_stroke =
                                egui::Stroke::new(2.0, color_a);
                            ui.visuals_mut().widgets.hovered.fg_stroke =
                                egui::Stroke::new(2.0, color_a);
                            ui.visuals_mut().widgets.active.fg_stroke =
                                egui::Stroke::new(2.0, color_a);
                            ui.visuals_mut().selection.bg_fill = color_a;

                            // Трек (фон полоски) — цвет B
                            ui.visuals_mut().widgets.inactive.bg_fill =
                                color_b.gamma_multiply(0.25); // приглушённо
                            ui.visuals_mut().widgets.hovered.bg_fill = color_b.gamma_multiply(0.35);
                            ui.visuals_mut().widgets.active.bg_fill = color_b.gamma_multiply(0.45);

                            ui.add_sized(
                                [40.0, 20.0],
                                egui::Slider::new(&mut matrix.values[type_a][type_b], -1.0..=1.0)
                                    .show_value(false),
                            );
                        }
                        ui.end_row(); // <- обязательно в конце каждой строки Grid
                    }
                });
        });

    Ok(()) // <- обязательно в конце
}
