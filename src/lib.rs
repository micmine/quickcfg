use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

/// Returns the path to to the condiguration file
pub async fn get_location(project_name: &str) -> Option<String> {
    if let Some(mut dir) = dirs::config_dir() {
        dir.push(project_name);
        if let Some(dir) = dir.to_str() {
            if fs::create_dir_all(dir).await.is_err() {
                return None;
            }
        }
        dir.push(project_name);
        dir.set_extension("json");

        if let Some(file) = dir.to_str() {
            return Some(file.to_string());
        }
    }

    None
}

/// Loads the config from the filesystem
pub async fn load<T: Default + for<'a> Deserialize<'a> + Serialize>(location: &str) -> T {
    match fs::read_to_string(&location).await {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                save(T::default(), location)
                    .await
                    .expect("Unable to write config");
            }

            T::default()
        }
    }
}

/// Save the config to filesystem
pub async fn save<T:Default + Serialize>(config: T, location: &str) -> Result<(), std::io::Error> {
    let config: String = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(location).await?;

    file.write_all(config.as_bytes()).await?;

    file.flush().await?;

    Ok(())
}
