use std::env::temp_dir;
use std::path::{Path, PathBuf};
use tauri::api::path::{home_dir, resource_dir};
use tauri::{Env, PackageInfo};

static APP_DIR: &str = "roro";

#[allow(dead_code)]
static RORO_CONFIG: &str = "config.yaml";

#[allow(dead_code)]
static PROFILE_YAML: &str = "profiles.yaml";

#[allow(dead_code)]
static PROFILE_TEMP: &str = "yoyo-runtime.yaml";

static HISTORY_TEMP: &str = "history.yaml";

static CATALOGUE_YAML: &str = "catalogues.yaml";

static POST_YAML: &str = "posts.yaml";

/// get the yoyo app home dir
pub fn app_home_dir() -> PathBuf {
    home_dir()
        .unwrap()
        .join(Path::new(".config"))
        .join(Path::new(APP_DIR))
}

#[allow(dead_code)]
/// get the resources dir
pub fn app_resources_dir(package_info: &PackageInfo) -> PathBuf {
    resource_dir(package_info, &Env::default())
        .unwrap()
        .join("resources")
}

#[allow(dead_code)]
/// profiles dir
pub fn app_profiles_dir() -> PathBuf {
    app_home_dir().join("profiles")
}

#[allow(dead_code)]
/// logs dir
pub fn app_logs_dir() -> PathBuf {
    app_home_dir().join("logs")
}

/// history dir
pub fn app_hostory_dir() -> PathBuf {
    app_home_dir().join("histoy")
}

/// workspace dir
pub fn app_workspace_dir() -> PathBuf {
    app_home_dir().join("workspace")
}

#[allow(dead_code)]
pub fn yoyo_path() -> PathBuf {
    app_home_dir().join(RORO_CONFIG)
}

#[allow(dead_code)]
pub fn profiles_path() -> PathBuf {
    app_home_dir().join(PROFILE_YAML)
}

#[allow(dead_code)]
pub fn profiles_temp_path() -> PathBuf {
    temp_dir().join(PROFILE_TEMP)
}

#[allow(dead_code)]
pub fn history_path() -> PathBuf {
    app_hostory_dir().join(HISTORY_TEMP)
}

#[allow(dead_code)]
pub fn catalogue_path() -> PathBuf {
    app_workspace_dir().join(CATALOGUE_YAML)
}

#[allow(dead_code)]
pub fn post_path() -> PathBuf {
    app_workspace_dir().join(POST_YAML)
}
