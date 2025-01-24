use bevy::log::info;
use bevy::prelude::{Camera2d, Camera3dBundle, Commands, Entity, MonitorSelection, Query, Window, WindowPosition, With};
use bevy::ui::State;
use bevy::utils::default;
use bevy::window::{WindowMode, WindowResolution};
use crate::interface::{LudoClientGameState, LudoInterfaceMenuComponent};

pub fn client_load_game(mut commands: Commands, mut window_query: Query<&mut Window>) {
    info!("loading game!");
    commands.insert_resource(bevy::prelude::State::new(LudoClientGameState::GameMenu));
    let (camera_id, interface_id) = (commands.register_system(client_despawn_camera), commands.register_system(client_despawn_interface));
    commands.run_system(interface_id);
    let mut window_query = window_query.single_mut();
    window_query.mode = WindowMode::Fullscreen(MonitorSelection::Primary);
    window_query.resolution = Default::default();
    commands.spawn(Camera3dBundle::default());
    commands.run_system(camera_id);
}

fn client_despawn_interface(mut commands: Commands, camera_component: Query<Entity, With<LudoInterfaceMenuComponent>>) {
    commands.get_entity(camera_component.get_single().unwrap()).unwrap().clear().despawn();
}

fn client_despawn_camera(mut commands: Commands, interface_menu: Query<Entity, With<Camera2d>>) {
    commands.get_entity(interface_menu.get_single().unwrap()).unwrap().clear().despawn();
}