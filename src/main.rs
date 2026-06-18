mod components;
mod config;
mod plugins;
mod resources;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use plugins::particles::ParticlesPlugin;
use plugins::physics::PhysicsPlugin;
use resources::InteractionMatrix;

use crate::plugins::controls::ControlsPlugin;
use crate::plugins::hud::HudPlugin;
use crate::plugins::matrix_editor::MatrixEditorPlugin;
use crate::resources::{SimState, SpatialGrid};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Particle Life".into(),
                resolution: (900, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(EguiPlugin::default())
        // Регистрируем матрицу как ресурс со случайными значениями
        .insert_resource(InteractionMatrix::random())
        .insert_resource(SimState::default())
        .insert_resource(SpatialGrid::default())
        .add_plugins(ParticlesPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(MatrixEditorPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
