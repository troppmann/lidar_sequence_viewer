use bevy::{prelude::*, tasks::*};
use futures_lite::future;
use rfd::*;

use super::ui_plugin::UiState;
use crate::{
    io,
    plugins::{lidar::PlayerState, PlayerConfig},
};

#[derive(Copy, Clone)]
pub enum FolderTaskType {
    Seqeunce,
    Label,
}

#[derive(Component)]
pub struct LoadFolderTask {
    task: Task<Option<FileHandle>>,
    folder_type: FolderTaskType,
}

pub fn handle_load_folder_task(
    mut commands: Commands,
    mut read_frame_tasks: Query<(Entity, &mut LoadFolderTask)>,
    mut menu_state: ResMut<UiState>,
    mut player_state: ResMut<PlayerState>,
    mut config: ResMut<PlayerConfig>,
) {
    for (entity, mut folder_task) in &mut read_frame_tasks {
        let folder_type = folder_task.folder_type;
        if let Some(file_handle) = future::block_on(future::poll_once(&mut folder_task.task)) {
            if let Some(folder) = file_handle {
                match folder_type {
                    FolderTaskType::Seqeunce => {
                        match io::read_sequence_from_dir(folder.path().into()) {
                            Ok(sequence) => {
                                player_state.set_sequence(sequence);
                                config.persistent.folder_path =
                                    folder.path().to_str().map(|str| str.to_string());
                                config.save();
                            }
                            Err(error) => {
                                rfd::MessageDialog::new()
                                    .set_title("Error")
                                    .set_description(&format!("{error}"))
                                    .set_buttons(rfd::MessageButtons::Ok)
                                    .show();
                            }
                        }
                    }
                    FolderTaskType::Label => {
                        if let Err(error) = player_state.try_set_labels(folder.path().into()) {
                                rfd::MessageDialog::new()
                                    .set_title("Error")
                                    .set_description(&format!("{error}"))
                                    .set_buttons(rfd::MessageButtons::Ok)
                                    .show();
                        }
                    }
                }
            }
            commands.entity(entity).despawn();
            match folder_type {
                FolderTaskType::Seqeunce => menu_state.folder_dialog.closed(),
                FolderTaskType::Label => menu_state.label_folder_dialog.closed(),
            }
        }
    }
}

pub fn spawn_load_folder_task(commands: &mut Commands, folder_type: FolderTaskType) {
    let task_pool = IoTaskPool::get();
    let task = task_pool.spawn(async move {
        AsyncFileDialog::new()
            .set_directory("/")
            .pick_folder()
            .await
    });
    commands.spawn(LoadFolderTask { task, folder_type });
}
