use std::{
    env,
    error::Error,
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    path::Path,
};

pub fn search_in_path() -> Result<Vec<String>, Box<dyn Error>> {
    let content = env::var("PATH")?;
    let paths: Vec<&str> = content.split(":").collect();
    let mut commands: Vec<String> = vec![];
    const EXECUTABLE_PERMISSION: u32 = 0o100;

    for path in paths.iter() {
        let path_st = Path::new(path);
        if !path_st.exists() {
            continue;
        }
        let read_dir = fs::read_dir(path_st)?;
        for entry_res in read_dir {
            if entry_res.is_err() {
                continue;
            }
            let entry = entry_res?;
            let f = File::open(entry.path())?;
            let metadata = f.metadata()?;
            if !metadata.is_file() {
                continue;
            }
            let permissions = metadata.permissions();
            if permissions.mode() & EXECUTABLE_PERMISSION != EXECUTABLE_PERMISSION {
                continue;
            }
            let file_name = entry.file_name();
            let file_name_cow = file_name.to_string_lossy();
            if file_name_cow == "xz" {
                continue;
            }
            commands.push(String::from(file_name_cow));
        }
    }

    Ok(commands)
}
