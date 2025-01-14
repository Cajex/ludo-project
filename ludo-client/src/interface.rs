use std::slice::Windows;
use std::thread;
use std::time::Duration;
use bevy::ecs::entity::Entities;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use bevy::window::WindowResolution;
use bevy_simple_text_input::{TextInput, TextInputSettings, TextInputTextColor, TextInputValue};
use crate::client::LudoClientChangeStateEvent;

const IMAGE_HEIGHT: f32 = 1024.;
const IMAGE_WIDTH: f32 = 1366.;
const IMAGE_FACTOR: f32 = 1.5;

#[derive(Default)]
pub struct LudoClientUserInterfacePlugin {

}

impl Plugin for LudoClientUserInterfacePlugin {
    fn build(&self, application: &mut App) {
        application.init_state::<LudoClientUserInterfaceState>().add_systems(
            OnEnter(LudoClientUserInterfaceState::ServerPingMenu),
            Self::enable_server_ping_menu_interface
        ).add_systems(
            Update, (
                Self::server_ping_menu_interface_interaction_style.run_if(in_state(LudoClientUserInterfaceState::ServerPingMenu)),
                Self::server_ping_menu_interface_interaction_enter.run_if(in_state(LudoClientUserInterfaceState::ServerPingMenu)),
                Self::client_change_state_listener.run_if(in_state(LudoClientUserInterfaceState::WaitingMenu)),
            )
        );
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LudoClientUserInterfaceState {
    #[default]
    ServerPingMenu,
    WaitingMenu,
    GameMenu,
}

#[derive(Component)]
pub struct LudoInterfaceDraggable(bool, Vec2);

#[derive(Component)]
pub struct LudoInterfaceMenuComponent;

#[derive(Component)]
pub struct LudoInterfacePingMenuComponent;

impl LudoClientUserInterfacePlugin {
    pub fn enable_server_ping_menu_interface(mut commands: Commands, mut window: Query<&mut Window>, asset_server: Res<AssetServer>) {
        thread::sleep(Duration::from_millis(500));
        info!("preparing waiting menu interface...");

        window.get_single_mut().unwrap().resolution = WindowResolution::new(IMAGE_WIDTH / IMAGE_FACTOR, IMAGE_HEIGHT / IMAGE_FACTOR);
        window.get_single_mut().unwrap().position = WindowPosition::Centered(MonitorSelection::Primary);
        commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));
        commands.spawn(
            (Node {
                width: Val::Px(IMAGE_WIDTH / IMAGE_FACTOR),
                height: Val::Px(IMAGE_HEIGHT / IMAGE_FACTOR),
                justify_content: JustifyContent::Center,
                ..default()
            }, LudoInterfaceMenuComponent, LudoInterfaceDraggable(true, Vec2::ZERO))
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
        mut event_writer: EventWriter<LudoClientChangeStateEvent>
    ) {
        interaction_query.iter_mut().for_each(|(interaction, children)| {
            match interaction {
                Interaction::Pressed => {
                    if let Ok(input) = input_query.get_single_mut() {
                        let input = input.0.0.clone();
                        if input.contains(".") && input.contains(":") {
                            commands.set_state(LudoClientUserInterfaceState::WaitingMenu);
                            event_writer.send(LudoClientChangeStateEvent);
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

    pub fn client_change_state_listener(mut commands: Commands, mut event_reader: EventReader<LudoClientChangeStateEvent>, asset_server: ResMut<AssetServer>, state: Res<State<LudoClientUserInterfaceState>>, ping_components_query: Query<Entity, With<LudoInterfacePingMenuComponent>>, menu_query: Query<Entity, With<LudoInterfaceMenuComponent>>) {
        for _ in event_reader.read() {
            match state.get() {
                LudoClientUserInterfaceState::ServerPingMenu => {

                }
                LudoClientUserInterfaceState::WaitingMenu => {
                    let image_handle = asset_server.load("client.image.profile.png");
                    ping_components_query.iter().for_each(|entity| {
                        commands.entity(entity).clear_children().despawn();
                    });
                    menu_query.iter().for_each(|component| {
                        let mut menu_commands = commands.entity(component);
                        menu_commands.with_children(|parent| {
                            parent.spawn(Node {
                                position_type: PositionType::Absolute,
                                align_items: AlignItems::DEFAULT,
                                flex_direction: FlexDirection::Row,
                                row_gap: Val::Px(50.),
                                top: Val::Percent(60.),
                                ..default()
                            }).with_children(|parent| {
                                for i in 1..5 {
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
                                        parent.spawn((Text::new(format!("nickname: User-{}", i)), TextFont::from_font_size(12.), TextColor::from(Color::WHITE)));
                                        parent.spawn((Text::new("offline".to_string()), TextColor::from(Color::xyz(0.41, 0.21, 0.02))));
                                    });
                                }
                            });
                        });
                    });
                }
                LudoClientUserInterfaceState::GameMenu => {

                }
            }
        }
    }

}