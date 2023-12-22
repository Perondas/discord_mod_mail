use crate::Error;

static PATH: &str = "/var/lib/steam-market-bot/settings.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub message_channel: Option<u64>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            message_channel: Default::default(),
        }
    }
}

pub fn create_settings() -> Result<(), Error> {
    let path = std::path::Path::new(PATH);
    if path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(path.parent().unwrap())?;

    let file = std::fs::File::create(path)?;

    let settings = Settings::default();

    serde_json::to_writer_pretty(file, &settings)?;

    Ok(())
}

pub fn read_settings() -> Result<Settings, Error> {
    let path = std::path::Path::new(PATH);

    let file = std::fs::File::open(path)?;

    let settings: Settings = serde_json::from_reader(file)?;

    Ok(settings)
}

pub fn write_settings(settings: Settings) -> Result<(), Error> {
    let path = std::path::Path::new(PATH);

    let file = std::fs::File::create(path)?;

    serde_json::to_writer_pretty(file, &settings)?;

    Ok(())
}
