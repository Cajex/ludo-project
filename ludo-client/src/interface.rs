use crate::client;
use crate::client::{LudoClientCachedOnlinePlayersProfiles, LudoClientConnectionInfo, LudoClientConnectionStable};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_simple_text_input::{TextInput, TextInputTextColor, TextInputValue};
use std::thread;
use std::time::Duration;
use bevy::winit::WinitSettings;
use ludo_commons::game::LudoGameConfiguration;

const IMAGE_HEIGHT: f32 = 1024.;
const IMAGE_WIDTH: f32 = 1366.;
const IMAGE_FACTOR: f32 = 1.5;

#[derive(Default)]
pub struct LudoClientUserInterfacePlugin;
impl Plugin for LudoClientUserInterfacePlugin {
    fn build(&self, application: &mut App) {
        application.init_state::<LudoClientGameState>().add_systems(
            OnEnter(LudoClientGameState::ServerPingMenu),
            Self::enable_server_ping_menu_interface)
            .insert_resource(WinitSettings::default())
            .add_systems(
            Update, (
                Self::server_ping_menu_interface_interaction_style.run_if(in_state(LudoClientGameState::ServerPingMenu)),
                Self::server_ping_menu_interface_interaction_enter.run_if(in_state(LudoClientGameState::ServerPingMenu)),
                Self::client_update_interface_listener
            )
        );
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LudoClientGameState {
    #[default]
    ServerPingMenu,
    WaitingMenu,
    GameMenu,
}

#[derive(Component)]
pub struct LudoInterfaceMenuComponent;

#[derive(Component)]
pub struct LudoInterfacePingMenuComponent;

#[derive(Component)]
pub struct LudoInterfaceWaitingMenuProfileDescriptorComponent(pub bool, pub u8);

#[derive(Component)]
pub struct LudoInterfaceWaitingMenuMinimumPlayersComponent;

impl LudoClientUserInterfacePlugin {
    pub fn enable_server_ping_menu_interface(mut commands: Commands, mut window: Query<&mut Window>, asset_server: Res<AssetServer>, connection_stable: Res<LudoClientConnectionStable>) {
        thread::sleep(Duration::from_millis(500));
        if connection_stable.0.is_none() {
            window.get_single_mut().unwrap().resolution = WindowResolution::new(IMAGE_WIDTH / IMAGE_FACTOR, IMAGE_HEIGHT / IMAGE_FACTOR);
            window.get_single_mut().unwrap().position = WindowPosition::Centered(MonitorSelection::Primary);
            commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));
            commands.spawn(
                (Node {
                    width: Val::Px(IMAGE_WIDTH / IMAGE_FACTOR),
                    height: Val::Px(IMAGE_HEIGHT / IMAGE_FACTOR),
                    justify_content: JustifyContent::Center,
                    ..default()
                }, LudoInterfaceMenuComponent)
            ).with_children(|parent| {
                parent.spawn(ImageNode {
                    image: asset_server.load("client.image.background.png"),
                    ..default()
                });
                parent.spawn(
                    (Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(370.),
                        height: Val::Px(50.),
                        top: Val::Percent(60.),
                        border: UiRect::all(Val::Px(2.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }, BorderRadius::all(Val::Px(10.)), BorderColor(Color::WHITE), Button, Interaction::None, LudoInterfacePingMenuComponent)
                ).with_children(|parent| {
                    parent.spawn((Text("enter the game".to_string()), LudoInterfacePingMenuComponent));
                });
                parent.spawn(
                    (Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(370.),
                        height: Val::Px(50.),
                        top: Val::Percent(50.),
                        border: UiRect::all(Val::Px(2.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }, LudoInterfacePingMenuComponent, Interaction::None, BorderRadius::all(Val::Px(10.)), BorderColor(Color::WHITE), TextInput, TextInputValue("address".to_string()), TextInputTextColor(TextColor::from(Color::srgb(0.9, 0.9, 0.9))))
                );
            });
        }
    }

    pub fn server_ping_menu_interface_interaction_style(mut interaction_query: Query<(&Interaction, &mut BorderRadius, &mut BorderColor, &Children), (Changed<Interaction>)>) {
        interaction_query.iter_mut().for_each(|(interaction, mut border_radius, mut color, children)| {
            match interaction {
                Interaction::Pressed => {
                }
                Interaction::Hovered => {
                    border_radius.bottom_left = Val::Px(0.);
                    border_radius.bottom_right = Val::Px(0.);
                    border_radius.top_left = Val::Px(0.);
                    border_radius.top_right = Val::Px(0.);
                    color.0 = Color::xyz(0.29, 0.56, 0.17);
                }
                Interaction::None => {
                    border_radius.bottom_left = Val::Px(10.);
                    border_radius.bottom_right = Val::Px(10.);
                    border_radius.top_left = Val::Px(10.);
                    border_radius.top_right = Val::Px(10.);
                    color.0 = Color::WHITE;
                }
            }
        })
    }

    pub fn server_ping_menu_interface_interaction_enter(
        mut commands: Commands,
        entity: Query<Entity, With<LudoInterfaceMenuComponent>>,
        mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
        mut input_query: Query<(&TextInputValue, &Children), With<TextInput>>,
    ) {
        interaction_query.iter_mut().for_each(|(interaction, children)| {
            match interaction {
                Interaction::Pressed => {
                    if let Ok(input) = input_query.get_single_mut() {
                        let input = input.0.0.clone();
                        if input.contains(".") && input.contains(":") {
                            commands.spawn(LudoClientConnectionInfo(input.clone()));
                            let system_id = commands.register_system(client::LudoClientPlugin::connect_client_system);
                            commands.run_system(system_id);
                        } else {
                            if let Ok(entity) = entity.get_single() {
                                let mut entity = commands.get_entity(entity).expect("Entity does not exists!");
                                entity.with_children(|parent| {
                                    parent.spawn(
                                        (Node {
                                            position_type: PositionType::Absolute,
                                            width: Val::Px(370.),
                                            height: Val::Px(50.),
                                            top: Val::Percent(65.),
                                            border: UiRect::all(Val::Px(2.)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        })
                                    ).with_children(|parent| {
                                        parent.spawn((Text("Error: this is not a valid server address!".to_string()), TextFont::from_font_size(13.), LudoInterfacePingMenuComponent, TextColor(Color::xyz(0.44, 0.25, 0.07))));
                                    });
                                });
                            }
                        }
                    }
                }
                Interaction::Hovered => {}
                Interaction::None => {}
            }
        })
    }

    pub fn client_change_state_listener(mut commands: Commands, asset_server: ResMut<AssetServer>, state: Res<State<LudoClientGameState>>, ping_components_query: Query<Entity, With<LudoInterfacePingMenuComponent>>, menu_query: Query<Entity, With<LudoInterfaceMenuComponent>>) {
        match state.get() {
            LudoClientGameState::ServerPingMenu => {

            }
            LudoClientGameState::WaitingMenu => {
                let image_handle = asset_server.load("client.image.profile.png");
                ping_components_query.iter().for_each(|entity| {
                    commands.entity(entity).clear_children().despawn();
                });
                menu_query.iter().for_each(|component| {
                    let mut menu_commands = commands.entity(component);
                    menu_commands.with_children(|parent| {
                        parent.spawn(Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(45.),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        }).with_children(|parent| {
                            parent.spawn((Text("Required players: ".to_string()), TextFont::from_font_size(18.), TextColor::from(Color::WHITE)));
                            parent.spawn((Text("/".to_string()), TextFont::from_font_size(18.), TextColor::from(Color::xyz(0.57, 0.55, 0.10)), LudoInterfaceWaitingMenuMinimumPlayersComponent));
                            parent.spawn((Text(" | of: ".to_string()), TextFont::from_font_size(18.), TextColor::from(Color::WHITE)));
                            parent.spawn((Text("4".to_string()), TextFont::from_font_size(18.), TextColor::from( Color::xyz(0.41, 0.21, 0.02))));
                        });
                        parent.spawn(Node {
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::DEFAULT,
                            flex_direction: FlexDirection::Row,
                            row_gap: Val::Px(50.),
                            top: Val::Percent(60.),
                            ..default()
                        }).with_children(|parent| {
                            for i in 0..4 {
                                parent.spawn((Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(10.)),
                                    margin: UiRect::all(Val::Px(10.)),
                                    ..default()
                                }, BorderRadius::all(Val::Px(10.)), BorderColor(Color::WHITE))).with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        node: Node {
                                            width: Val::Px(50.),
                                            height: Val::Px(50.),
                                            ..default()
                                        },
                                        image: ImageNode {
                                            image: image_handle.clone(),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                    parent.spawn((LudoInterfaceWaitingMenuProfileDescriptorComponent(true, i), Text::new(format!("nickname: User-{}", i+1)), TextFont::from_font_size(12.), TextColor::from(Color::WHITE)));
                                    parent.spawn((LudoInterfaceWaitingMenuProfileDescriptorComponent(false, i), Text::new("offline".to_string()), TextColor::from(Color::xyz(0.41, 0.21, 0.02))));
                                });
                            }
                        });
                    });
                });
            }
            LudoClientGameState::GameMenu => {

            }
        }
    }

    pub fn client_update_interface_listener(state: Res<State<LudoClientGameState>>, cached_online_players: Res<LudoClientCachedOnlinePlayersProfiles>, mut waiting_menu_profile_descriptor_components: Query<(&mut Text, &mut TextColor, &LudoInterfaceWaitingMenuProfileDescriptorComponent)>) {
        match state.get() {
            LudoClientGameState::ServerPingMenu => {}
            LudoClientGameState::WaitingMenu => {
                waiting_menu_profile_descriptor_components.iter_mut().for_each(|(mut text, mut color, component)| {
                    if let Some(profile) = cached_online_players.0.get(component.1 as usize) {
                        if component.0 {
                            text.0 = profile.nickname.clone();
                        } else {
                            text.0 = "online".to_string();
                            color.0 = Color::xyz(0.39, 0.73, 0.24);
                        }
                    } else {
                        if component.0 {
                            text.0 = "unknown".to_string();
                        } else {
                            text.0 = "offline".to_string();
                            color.0 = Color::xyz(0.41, 0.21, 0.02);
                        }
                    }
                })
            }
            LudoClientGameState::GameMenu => {}
        }
    }

    pub fn client_load_minimum_players_system(mut min_players_query: Query<&mut Text, (With<LudoInterfaceWaitingMenuMinimumPlayersComponent>, Without<LudoInterfaceWaitingMenuProfileDescriptorComponent>)>, server_configuration: Res<LudoGameConfiguration>) {
        min_players_query.iter_mut().for_each(|mut text| {
            text.0 = format!("{}", server_configuration.min_players_to_start);
        });
    }

}