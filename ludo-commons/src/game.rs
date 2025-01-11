use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use bevy::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Resource)]
pub struct LudoGameObject {
    pub state: LudoGameState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LudoGameState {
    Waiting,
    InGame,
    Closing
}

#[derive(Serialize, Deserialize, Debug, Clone, Component)]
pub struct LudoGameProfile {
    pub unique_id: [u8; 16],
    pub nickname: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Component)]
pub struct LudoGameProfileData {
    pub unique_id: [u8; 16],
    pub points: u128,
}

impl LudoGameProfileData {
    pub fn load_from_file(file: &str) -> Result<Vec<LudoGameProfileData>> {
        let file_path = PathBuf::from(file);
        if file_path.exists() {
            let mut open_options = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
            if !file.is_empty() {
                let mut buf = String::new();
                open_options.read_to_string(&mut buf)?;
                Ok(serde_json::from_str::<Vec<LudoGameProfileData>>(&buf)?)
            } else {
                open_options.write_all(serde_json::to_string::<Vec<LudoGameProfileData>>(&vec![])?.as_bytes()).expect("Could not write data");
                Ok(vec![])
            }
        } else {
            let mut open_options = OpenOptions::new().write(true).create(true).open(file_path)?;
            open_options.write_all(serde_json::to_string::<Vec<LudoGameProfileData>>(&vec![])?.as_bytes()).expect("Could not write data");
            Ok(vec![])
        }
    }

    pub fn load_to_file(file: &str, list: Vec<LudoGameProfileData>) -> Result<()> {
        let file_path = PathBuf::from(file);
        let mut open_options = OpenOptions::new().write(true).create(true).open(file_path)?;
        open_options.write_all(serde_json::to_string(&list)?.as_bytes()).expect("Could not write data");
        Ok(())
    }
}

impl LudoGameProfile {
    pub fn load_from_file(file: &str) -> Result<LudoGameProfile> {
        let file_path = PathBuf::from(file);
        if file_path.exists() {
            let mut open_options = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
            if !file.is_empty() {
                let mut buf = String::new();
                open_options.read_to_string(&mut buf)?;
                Ok(serde_json::from_str::<LudoGameProfile>(&buf)?)
            } else {
                /* todo: error */
                Ok(LudoGameProfile {
                    unique_id: *Uuid::new_v4().as_bytes(),
                    nickname: "default".to_string(),
                    age: 17,
                })
            }
        } else {
            let mut open_options = OpenOptions::new().write(true).create(true).open(file_path)?;
            open_options.write_all(serde_json::to_string::<LudoGameProfile>(&LudoGameProfile {
                unique_id: *Uuid::new_v4().as_bytes(),
                nickname: "default".to_string(),
                age: 17,
            })?.as_bytes()).expect("Could not write data");
            Ok(LudoGameProfile {
                unique_id: *Uuid::new_v4().as_bytes(),
                nickname: "default".to_string(),
                age: 17,
            })
        }
    }

    pub fn load_to_file(file: &str, profile: LudoGameProfile) -> Result<()> {
        let file_path = PathBuf::from(file);
        let mut open_options = OpenOptions::new().write(true).create(true).open(file_path)?;
        open_options.write_all(serde_json::to_string(&profile)?.as_bytes()).expect("Could not write data");
        Ok(())
    }
}