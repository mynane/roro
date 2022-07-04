use crate::utils::{config, dirs};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrfItem {
    /// profile item type
    /// enum value: remote | local | script | merge
    #[serde(rename = "type")]
    pub itype: Option<String>,

    /// profile name
    pub name: Option<String>,

    /// profile description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// profile file
    pub file: Option<String>,

    /// source url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// selected infomation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<Vec<PrfSelected>>,

    /// subscription user info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<PrfExtra>,

    /// updated time
    pub updated: Option<usize>,

    /// some options of the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<PrfOption>,

    /// the file data
    #[serde(skip)]
    pub file_data: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Profiles {
    /// same as PrfConfig.current
    current: Option<String>,
    /// same as PrfConfig.chain
    chain: Option<Vec<String>>,
    /// Profile list
    items: Option<Vec<String>>,
}

impl Profiles {
    /// read the config from the file
    pub fn read_file() -> Self {
        let mut profiles: Profiles = config::read_yaml::<Self>(dirs::profiles_path());

        if profiles.items.is_none() {
            profiles.items = Some(vec![]);
        }

        profiles
    }

    /// save the config to the file
    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(
            dirs::profiles_path(),
            self,
            Some("# Profiles Config for yoyo\n\n"),
        )
    }

    /// sync the config between file and memory
    pub fn sync_file(&mut self) -> Result<()> {
        let data: Profiles = Self::read_file();

        if data.current.is_none() && data.items.is_none() {
            bail!("fail to read profiles.yaml")
        }

        self.current = data.current;
        self.chain = data.chain;
        self.items = data.items;

        Ok(())
    }

    /// get the current uid
    pub fn get_current(&self) -> Option<String> {
        self.current.clone()
    }

    /// only change the main to the target id
    pub fn put_current(&mut self, uid: String) -> Result<()> {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        bail!("invalid uid \"{uid}\"");
    }
}
