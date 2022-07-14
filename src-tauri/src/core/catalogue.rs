use std::{fs, io::Write};

use crate::utils::{config, dirs, help};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClgItem {
    /// catalogue uid
    pub uid: Option<String>,

    /// catalogue name
    pub name: Option<String>,
}

impl Default for ClgItem {
    fn default() -> Self {
        ClgItem {
            uid: None,
            name: None,
        }
    }
}

impl ClgItem {}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Catalogues {
    /// same as PrfConfig.current
    current: Option<String>,

    /// profile list
    items: Option<Vec<ClgItem>>,
}

macro_rules! patch {
    ($lv: expr, $rv: expr, $key: tt) => {
        if ($rv.$key).is_some() {
            $lv.$key = $rv.$key;
        }
    };
}

impl Catalogues {
    /// read the config from the file
    pub fn read_file() -> Self {
        let mut catalogues = config::read_yaml::<Self>(dirs::catalogue_path());

        if catalogues.items.is_none() {
            catalogues.items = Some(vec![]);
        }

        catalogues.items.as_mut().map(|items| {
            for mut item in items.iter_mut() {
                if item.uid.is_none() {
                    item.uid = Some(help::get_uid("c"));
                }
            }
        });

        catalogues
    }

    /// save the config to the file
    pub fn save_file(&self) -> Result<()> {
        config::save_yaml(
            dirs::catalogue_path(),
            self,
            Some("# Catalogues Config for roro\n\n"),
        )
    }

    /// sync the config between file and memory
    pub fn sync_file(&mut self) -> Result<()> {
        let data = Self::read_file();
        if data.current.is_none() && data.items.is_none() {
            bail!("failed to read catalogue.yaml");
        }

        self.current = data.current;
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

        let items = self.items.as_ref().unwrap();
        let some_uid = Some(uid.clone());

        for each in items.iter() {
            if each.uid == some_uid {
                self.current = some_uid;
                return self.save_file();
            }
        }

        bail!("invalid uid \"{uid}\"");
    }

    /// find the item by the uid
    pub fn get_item(&self, uid: &String) -> Result<&ClgItem> {
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
    pub fn append_item(&mut self, mut item: ClgItem) -> Result<()> {
        if item.uid.is_none() {
            item.uid = Some(help::get_uid("ci"));
            // bail!("the uid should not be null");
        }

        // save the file data
        // move the field value after save
        // if let Some(file_data) = item.file_data.take() {
        //     if item.file.is_none() {
        //         bail!("the file should not be null");
        //     }

        //     let file = item.file.clone().unwrap();
        //     let path = dirs::app_profiles_dir().join(&file);

        //     fs::File::create(path)
        //         .context(format!("failed to create file \"{}\"", file))?
        //         .write(file_data.as_bytes())
        //         .context(format!("failed to write to file \"{}\"", file))?;
        // }

        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        self.items.as_mut().map(|items| items.insert(0, item));
        self.save_file()
    }

    /// update the item's value
    pub fn patch_item(&mut self, uid: String, item: ClgItem) -> Result<()> {
        let mut items = self.items.take().unwrap_or(vec![]);

        for mut each in items.iter_mut() {
            if each.uid == Some(uid.clone()) {
                patch!(each, item, name);

                self.items = Some(items);
                return self.save_file();
            }
        }

        self.items = Some(items);
        bail!("failed to found the uid \"{uid}\"")
    }

    /// be used to update the remote item
    /// only patch `updated` `extra` `file_data`
    // pub fn update_item(&mut self, uid: String, mut item: ClgItem) -> Result<()> {
    //     if self.items.is_none() {
    //         self.items = Some(vec![]);
    //     }

    //     // find the item
    //     let _ = self.get_item(&uid)?;

    //     self.items.as_mut().map(|items| {
    //         let some_uid = Some(uid.clone());

    //         for mut each in items.iter_mut() {
    //             if each.uid == some_uid {
    //                 each.extra = item.extra;
    //                 each.updated = item.updated;

    //                 // save the file data
    //                 // move the field value after save
    //                 if let Some(file_data) = item.file_data.take() {
    //                     let file = each.file.take();
    //                     let file =
    //                         file.unwrap_or(item.file.take().unwrap_or(format!("{}.yaml", &uid)));

    //                     // the file must exists
    //                     each.file = Some(file.clone());

    //                     let path = dirs::app_profiles_dir().join(&file);

    //                     fs::File::create(path)
    //                         .unwrap()
    //                         .write(file_data.as_bytes())
    //                         .unwrap();
    //                 }

    //                 break;
    //             }
    //         }
    //     });

    //     self.save_file()
    // }

    /// delete item
    /// if delete the current then return true
    pub fn delete_item(&mut self, uid: String) -> Result<bool> {
        let current = self.current.as_ref().unwrap_or(&uid);
        let current = current.clone();

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

        // delete the original uid
        if current == uid {
            self.current = match items.len() > 0 {
                true => items[0].uid.clone(),
                false => None,
            };
        }

        self.items = Some(items);
        self.save_file()?;
        Ok(current == uid)
    }

    /// only generate config mapping
    pub fn gen_activate(&self) -> Result<Mapping> {
        let config = Mapping::new();

        if self.current.is_none() || self.items.is_none() {
            return Ok(config);
        }

        let current = self.current.clone().unwrap();

        // for item in self.items.as_ref().unwrap().iter() {
        //     if item.uid == Some(current.clone()) {
        //         let file_path = match item.file.clone() {
        //             Some(file) => dirs::app_profiles_dir().join(file),
        //             None => bail!("failed to get the file field"),
        //         };

        //         if !file_path.exists() {
        //             bail!("failed to read the file \"{}\"", file_path.display());
        //         }

        //         return Ok(config::read_yaml::<Mapping>(file_path.clone()));
        //     }
        // }

        bail!("failed to found the uid \"{current}\"");
    }
}
