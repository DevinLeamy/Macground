use std::path::PathBuf;

use directories::ProjectDirs;

pub fn application_data_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "", "macground").unwrap();
    let data_path = project_dirs.data_dir().to_path_buf();

    std::fs::create_dir_all(&data_path).unwrap();

    data_path
}
