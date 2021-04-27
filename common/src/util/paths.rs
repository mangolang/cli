use ::std::env;
use ::std::env::temp_dir;
use ::std::fs::create_dir_all;
use ::std::path::PathBuf;

use ::dirs::cache_dir;
use ::dirs::config_dir;
use ::dirs::home_dir;

//TODO @mark: cache all these things (what about changing environment?)

/// Get user cache directory for Mango (multi-project).
/// Documented at https://docs.mangocode.org/en/latest/setup_guide/paths.html
pub fn mango_user_cache_dir() -> PathBuf {
    let pth = env::var("MANGO_USER_CACHE_PATH")
        .map(PathBuf::from)
        .ok()
        .or_else(|| {
            cache_dir().map(|mut p| {
                p.push("mango");
                p
            })
        })
        .or_else(|| {
            home_dir().map(|mut p| {
                p.push(".mango");
                p.push("cache");
                p
            })
        })
        .unwrap_or_else(|| {
            let mut p = temp_dir();
            p.push("mango");
            p
        });
    create_dir_all(&pth).expect("could not create mango cache directory");
    pth
}

/// Get user configuration directory for Mango (multi-project).
/// Documented at https://docs.mangocode.org/en/latest/setup_guide/paths.html
pub fn mango_user_config_dir() -> PathBuf {
    let pth = env::var("MANGO_USER_CONFIG_PATH")
        .map(PathBuf::from)
        .ok()
        .or_else(|| {
            config_dir().map(|mut p| {
                p.push("mango");
                p
            })
        })
        .or_else(|| {
            home_dir().map(|mut p| {
                p.push(".mango");
                p.push("config");
                p
            })
        })
        .expect("could not find any configuration directory; set MANGO_USER_CONFIG_PATH to provide one");
    create_dir_all(&pth).expect("could not create mango config directory");
    pth
}

/// The root directory of the current Mango project.
pub fn mango_project_root_dir() -> PathBuf {
    panic!("start at current dir and go up until project file is found?"); //TODO @mark
                                                                           //TODO @mark: also implement unit test, including for mango_project_build_dir
                                                                           //let pth = env::current_dir();
                                                                           //create_dir_all(&pth)
                                                                           //    .expect("could not create mango config directory");
}

/// Get project build output directory for Mango (single-user, single-project).
/// Documented at https://docs.mangocode.org/en/latest/setup_guide/paths.html
#[allow(dead_code)] //TODO @mark: TEMPORARY! REMOVE THIS!
pub fn mango_project_build_dir() -> PathBuf {
    let pth = env::var("MANGO_TARGET_DIR").map(PathBuf::from).ok().unwrap_or_else(|| {
        eprintln!("MANGO_TARGET_DIR is empty but alternative is not yet implemented"); //TODO @mark: TEMPORARY! REMOVE THIS!
        let mut p = mango_project_root_dir();
        p.push("target");
        p
    });
    create_dir_all(&pth)
        .expect("could not create mango build directory for the project; you can change the location with MANGO_TARGET_DIR");
    pth
}

pub fn mangod_lock_file_path() -> PathBuf {
    let mut pth = mango_user_cache_dir();
    pth.push("mangod.lock");
    pth
}

#[cfg(test)]
mod tests {
    use ::serial_test::serial;
    use ::tempdir::TempDir;

    use super::*;

    #[serial]
    #[test]
    fn user_cache_env() {
        let dir = TempDir::new("mango_user_cache").unwrap();
        let env_pth = dir.path().to_string_lossy().into_owned();
        env::set_var("MANGO_USER_CACHE_PATH", &env_pth);
        let cache_pth = mango_user_cache_dir();
        assert_eq!(cache_pth.to_string_lossy(), env_pth)
    }

    #[serial]
    #[test]
    fn user_cache_no_env() {
        env::remove_var("MANGO_USER_CACHE_PATH");
        let cache_pth = mango_user_cache_dir();
        assert!(cache_pth.is_dir());
        assert!(cache_pth.to_string_lossy().contains("mango"));
    }

    #[serial]
    #[test]
    fn user_config_env() {
        let dir = TempDir::new("mango_user_config").unwrap();
        let env_pth = dir.path().to_string_lossy().into_owned();
        env::set_var("MANGO_USER_CONFIG_PATH", &env_pth);
        let conf_pth = mango_user_config_dir();
        assert_eq!(conf_pth.to_string_lossy(), env_pth)
    }

    #[serial]
    #[test]
    fn user_config_no_env() {
        env::remove_var("MANGO_USER_CONFIG_PATH");
        let conf_pth = mango_user_config_dir();
        assert!(conf_pth.is_dir());
        assert!(conf_pth.to_string_lossy().contains("mango"));
    }

    #[serial]
    #[test]
    fn project_build_env() {
        let dir = TempDir::new("mango_project_build").unwrap();
        let env_pth = dir.path().to_string_lossy().into_owned();
        env::set_var("MANGO_TARGET_DIR", &env_pth);
        let conf_pth = mango_project_build_dir();
        assert_eq!(conf_pth.to_string_lossy(), env_pth)
    }
}
