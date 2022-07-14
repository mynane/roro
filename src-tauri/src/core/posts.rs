use std::{fs, io::Write};

use crate::{
    utils::{config, dirs, help},
    wrap_err,
};
use anyhow::{bail, Context, Ok, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostItem {
    /// post uid
    pub uid: Option<String>,

    /// catalogue uid
    pub cuid: Option<String>,

    /// post name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl Default for PostItem {
    fn default() -> Self {
        PostItem {
            uid: None,
            cuid: None,
            content: None,
        }
    }
}

impl PostItem {}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Posts {
    /// profile list
    items: Option<Vec<PostItem>>,
}

macro_rules! patch {
    ($lv: expr, $rv: expr, $key: tt) => {
        if ($rv.$key).is_some() {
            $lv.$key = $rv.$key;
        }
    };
}

impl Posts {
    /// read the config from the file
    pub fn read_file() -> Self {
        let mut catalogues = config::read_yaml::<Self>(dirs::post_path());

        if catalogues.items.is_none() {
            catalogues.items = Some(vec![]);
        }

        catalogues.items.as_mut().map(|items| {
            for mut item in items.iter_mut() {
                if item.uid.is_none() {
                    item.uid = Some(help::get_uid("p"));
                }
            }
        });

        catalogues
    }

    /// save the config to the file
    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(dirs::post_path(), self, Some("# Posts Config for roro\n\n"))
    }

    /// sync the config between file and memory
    pub fn sync_file(&mut self) -> Result<()> {
        let data = Self::read_file();
        if data.items.is_none() {
            bail!("failed to read post.yaml");
        }

        self.items = data.items;
        Ok(())
    }

    /// find the item by the uid
    pub fn get_item(&self, uid: &String) -> Result<&PostItem> {
        if self.items.is_some() {
            let items = self.items.as_ref().unwrap();
            let some_uid = Some(uid.clone());

            for each in items.iter() {
                if each.uid == some_uid {
                    return Ok(each);
                }
            }
        }

        bail!("failed to get the item by \"{}\"", uid);
    }

    /// append new item
    /// if the file_data is some
    /// then should save the data to file
    pub fn append_item(&mut self, mut item: PostItem) -> Result<()> {
        self.save_file()?;
        if item.cuid.is_none() {
            bail!("cuid is required");
        }

        if item.uid.is_none() {
            item.uid = Some(help::get_uid("pi"));
        }

        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        self.items.as_mut().map(|items| items.insert(0, item));

        self.save_file()
    }

    /// update the item's value
    pub fn patch_item(&mut self, uid: String, item: PostItem) -> Result<()> {
        let mut items = self.items.take().unwrap_or(vec![]);

        for mut each in items.iter_mut() {
            if each.uid == Some(uid.clone()) {
                patch!(each, item, content);

                self.items = Some(items);
                return self.save_file();
            }
        }

        self.items = Some(items);
        bail!("failed to found the uid \"{uid}\"")
    }

    /// delete item
    /// if delete the current then return true
    pub fn delete_item(&mut self, uid: String) -> Result<()> {
        let mut items = self.items.take().unwrap_or(vec![]);

        let mut index = None;

        // get the index
        for i in 0..items.len() {
            if items[i].uid == Some(uid.clone()) {
                index = Some(i);
                break;
            }
        }

        if let Some(index) = index {
            items.remove(index);
        }

        self.items = Some(items);
        self.save_file()?;
        Ok(())
    }

    /// get all by cuid
    pub fn get_items_by_cuid(&mut self, cuid: String) -> Result<Vec<PostItem>> {
        let items = self.items.clone().take().unwrap_or(vec![]);

        let result = items
            .into_iter()
            .filter(|x| x.cuid == Some(cuid.clone()))
            .collect();

        Ok(result)
    }

    /// delete item by cuid
    pub fn delete_items_by_cuid(&mut self, cuid: String) -> Result<()> {
        let mut items = self.items.clone().take().unwrap_or(vec![]);

        items.retain(|x| x.cuid != Some(cuid.clone()));

        self.items = Some(items);
        self.save_file()?;

        Ok(())
    }
}
