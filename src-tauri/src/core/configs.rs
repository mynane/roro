use crate::utils::{config, dirs};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Configs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl Configs {
    pub fn read_file() -> Self {
        let history = config::read_yaml::<Self>(dirs::yoyo_path());
        history
    }

    pub fn save_file(&self) -> Result<()> {
        println!("{:?}", self);
        config::save_yaml(
            dirs::yoyo_path(),
            self,
            Some("# History Config for roro\n\n"),
        )
    }

    pub fn set_width(&mut self, width: u32) -> Result<()> {
        self.width = Some(width);
        self.save_file()
    }

    pub fn set_mode(&mut self, mode: String) -> Result<()> {
        self.mode = Some(mode);
        self.save_file()
    }
}
