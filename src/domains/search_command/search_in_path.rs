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
    const EXECUTABLE_PERMISSION: u32 = 0o111;

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
            commands.push(String::from(entry.file_name().to_string_lossy()));
        }
    }

    Ok(commands)
}
