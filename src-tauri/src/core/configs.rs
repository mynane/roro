use crate::utils::{config, dirs};
use anyhow::Result;
use serde::{Deserialize, Serialize};

macro_rules! patch {
    ($lv: expr, $rv: expr, $key: tt) => {
        if ($rv.$key).is_some() {
            $lv.$key = $rv.$key;
        }
    };
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Configs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<bool>,
}

impl Configs {
    pub fn new() -> Configs {
        Self::read_file()
    }

    pub fn read_file() -> Self {
        let confifs = config::read_yaml::<Self>(dirs::yoyo_path());
        confifs
    }

    pub fn async_file(&mut self) -> Result<()> {
        let confifs = Self::read_file();

        self.width = confifs.width;
        self.mode = confifs.mode;

        Ok(())
    }

    pub fn save_file(&self) -> Result<()> {
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

    pub fn set_mode(&mut self, mode: bool) -> Result<()> {
        self.mode = Some(mode);
        self.save_file()
    }
}
