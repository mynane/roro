use tauri::PackageInfo;

use super::{dirs, tmpl};
use std::{fs, io::Write, path::PathBuf};

/// Initialize all the files from resources
fn init_config(app_dir: &PathBuf) -> std::io::Result<()> {
    // target path
    let clash_path = app_dir.join("config.yaml");
    let verge_path = app_dir.join("verge.yaml");
    let profile_path = app_dir.join("profiles.yaml");

    if !clash_path.exists() {
        fs::File::create(clash_path)?.write(tmpl::CLASH_CONFIG)?;
    }
    if !verge_path.exists() {
        fs::File::create(verge_path)?.write(tmpl::VERGE_CONFIG)?;
    }
    if !profile_path.exists() {
        fs::File::create(profile_path)?.write(tmpl::PROFILES_CONFIG)?;
    }
    Ok(())
}

/// initialize app
pub fn init_app(_package_info: &PackageInfo) {
    // create app dir
    let app_dir = dirs::app_home_dir();
    let log_dir = dirs::app_logs_dir();
    let profiles_dir = dirs::app_profiles_dir();

    // let res_dir = dirs::app_resources_dir(package_info);

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).unwrap();
    }
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir).unwrap();
    }

    // init_log(&log_dir);
    if let Err(err) = init_config(&app_dir) {
        log::error!("{err}");
    }

    // copy the resource file
    // let mmdb_path = app_dir.join("Country.mmdb");
    // let mmdb_tmpl = res_dir.join("Country.mmdb");
    // if !mmdb_path.exists() && mmdb_tmpl.exists() {
    //     fs::copy(mmdb_tmpl, mmdb_path).unwrap();
    // }

    // // copy the wintun.dll
    // #[cfg(target_os = "windows")]
    // {
    //     let wintun_path = app_dir.join("wintun.dll");
    //     let wintun_tmpl = res_dir.join("wintun.dll");
    //     if !wintun_path.exists() && wintun_tmpl.exists() {
    //         fs::copy(wintun_tmpl, wintun_path).unwrap();
    //     }
    // }
}
