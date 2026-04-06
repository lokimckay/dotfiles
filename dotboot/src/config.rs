use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BootConfig {
    pub symlink: Vec<SymlinkRule>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    #[serde(alias = "linux")]
    #[serde(alias = "unix")]
    Wsl,

    #[serde(alias = "win")]
    #[serde(alias = "windows")]
    Windows,
}

#[derive(Debug, Deserialize)]
pub struct SymlinkRule {
    pub link: String,
    pub target: String,
    pub platform: Option<Platform>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}
