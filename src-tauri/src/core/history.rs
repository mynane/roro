use anyhow::bail;
use anyhow::Ok;
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::utils::{config, dirs};
use anyhow::Result;

macro_rules! patch {
    ($lv: expr, $rv: expr, $key: tt) => {
        if ($rv.$key).is_some() {
            $lv.$key = $rv.$key;
        }
    };
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct History {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<FileHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<FileHistory>>,
}

impl History {
    pub fn read_file() -> Self {
        let mut history = config::read_yaml::<Self>(dirs::history_path());

        if history.items.is_none() {
            history.items = Some(vec![]);
        }

        let file_history = FileHistory {
            itype: Some("file".into()),
            name: Some("test".into()),
            path: Some("./test.js".into()),
            ..FileHistory::default()
        };

        history.items.as_mut().map(|items| items.push(file_history));

        history
    }

    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(
            dirs::history_path(),
            self,
            Some("# History Config for yoyo\n\n"),
        )
    }

    pub fn async_file(&mut self) -> Result<()> {
        let mut history = Self::read_file();
        if history.items.is_none() {
            history.items = Some(vec![]);
        }
        self.items = history.items;

        Ok(())
    }

    pub fn append_history(&mut self, file_history: FileHistory) -> Result<()> {
        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        self.items.as_mut().map(|items| items.push(file_history));

        self.save_file()
    }

    /// find the item by the uid
    pub fn get_item(&mut self, path: String) -> Result<&FileHistory> {
        if self.items.is_some() {
            let items = self.items.as_mut().unwrap();
            let some_path = Some(path.clone());

            for each in items.iter() {
                if each.path == some_path {
                    return Ok(each);
                }
            }
        }
        bail!("failed to get the item by \"{}\"", path);
    }

    pub fn patch_item(&mut self, path: String, item: FileHistory) -> Result<()> {
        let mut items = self.items.take().unwrap_or(vec![]);

        for mut each in items.iter_mut() {
            if each.path == Some(path.clone()) {
                patch!(each, item, itype);
                patch!(each, item, name);
                patch!(each, item, path);
                patch!(each, item, create_at);
                patch!(each, item, update_at);

                self.items = Some(items);

                return self.save_file();
            }
        }

        self.items = Some(items);

        bail!("failed to found the path \"{path}\"")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileHistory {
    /// enum value: file | folder
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub itype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_at: Option<String>,
}

impl Default for FileHistory {
    fn default() -> Self {
        FileHistory {
            itype: None,
            name: None,
            path: None,
            create_at: Some(Local::now().to_string()),
            update_at: Some(Local::now().to_string()),
        }
    }
}
