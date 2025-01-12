use std::slice::Windows;
use std::thread;
use std::time::Duration;
use bevy::prelude::*;

#[derive(Default)]
pub struct LudoClientUserInterfacePlugin {

}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LudoClientUserInterfaceState {
    ServerPingMenu,
    #[default]
    WaitingMenu,
    GameMenu,
}

#[derive(Component)]
pub struct LudoTitleBar;

impl Plugin for LudoClientUserInterfacePlugin {
    fn build(&self, application: &mut App) {
        application.init_state::<LudoClientUserInterfaceState>().add_systems(OnEnter(LudoClientUserInterfaceState::WaitingMenu), Self::enable_waiting_menu_interface);
    }
}

impl LudoClientUserInterfacePlugin {
    pub fn enable_waiting_menu_interface(mut commands: Commands, mut window: Query<&mut Window>) {
        thread::sleep(Duration::from_millis(500));
        info!("preparing waiting menu interface...");

        commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));
        commands.spawn((Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            position_type: PositionType::Relative,
            ..default()
        }, BackgroundColor(Color::xyz(	0.239, 0.4118, 0.26766)), BorderRadius::all(Val::Percent(2.))))
            .with_children(|node| {
                node.spawn((Node {
                    width: Val::Percent(100.),
                    height: Val::Px(30.),
                    position_type: PositionType::Relative,
                    ..default()
                })).with_children(|node| {
                    node.spawn((Node {
                        position_type: PositionType::Relative,
                        width: Val::Px(20.),
                        height: Val::Px(20.),
                        top: Val::Px(10.),
                        left: Val::Percent(98.),
                        justify_content: JustifyContent::Center,
                        ..default()
                    }, BackgroundColor(Color::xyz(	0.40849, 0.2312, 0.004231)), BorderRadius::all(Val::Percent(100.)),
                        BorderColor(Color::BLACK),
                    )
                );
            });
        });
    }

}