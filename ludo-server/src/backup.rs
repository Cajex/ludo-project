use bevy::prelude::*;
use ludo_commons::game::LudoGameProfileData;

#[derive(Component)]
pub struct LudoBackupProfileTimer(pub Timer);

pub fn handle_backup_profile_timer(time: Res<Time>, mut timer: Query<&mut LudoBackupProfileTimer>, profiles: Query<&LudoGameProfileData>) {
    if timer.get_single_mut().unwrap().0.tick(time.delta()).just_finished() {
        let mut list = vec![];
        profiles.iter().for_each(|profile| {
            list.push(profile.clone());
        });
        LudoGameProfileData::load_to_file("profiles.json", list.clone()).expect("unable to save ludo game profiles!");
        info!("Backup of game profiles saved. Size: [{}]!", list.len());
    }
}