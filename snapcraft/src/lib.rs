use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// Checks to see whether we are running within a snap, and if so return the real home directory for the current user.
pub fn check_snap_home() -> (bool, Option<PathBuf>) {
    if in_snap() {
        (true, snap_real_home())
    } else {
        (false, None)
    }
}

/// Checks whether we are running within a snap
pub fn in_snap() -> bool {
    snap().is_some()
}

/// Directory where the snap is mounted. This is where all the files in your snap are visible in the filesystem.
/// All of the data in the snap is read-only and cannot be changed.
///
/// Typical value: `/snap/hello-world/27`
pub fn snap() -> Option<String> {
    if let Ok(snap) = env::var("SNAP") {
        Some(snap)
    } else {
        None
    }
}

/// CPU architecture of the running system.
/// Typical value `amd64`
///
/// Other values are: `i386`, `armhf`, `arm64`.
pub fn snap_arch() -> Option<String> {
    if let Ok(snap_arch) = env::var("SNAP_ARCH") {
        Some(snap_arch)
    } else {
        None
    }
}

/// Directory for system data that is common across revisions of a snap.
/// This directory is owned and writable by root and is meant to be used by background applications (daemons, services).
/// Unlike `SNAP_DATA` this directory is not backed up and restored across snap refresh and revert operations.
///
/// Typical value: `/var/snap/hello-world/common`
pub fn snap_common() -> Option<PathBuf> {
    if let Ok(snap_common) = env::var("SNAP_COMMON") {
        Some(Path::new(snap_common.as_str()).to_path_buf())
    } else {
        None
    }
}

/// Directory for system data of a snap.
/// This directory is owned and writable by root and is meant to be used by background applications (daemons, services).
/// Unlike `SNAP_COMMON` this directory is backed up and restored across snap refresh and snap revert operations.
///
/// Typical value `/var/snap/hello-world/27`
pub fn snap_data() -> Option<PathBuf> {
    if let Ok(snap_data) = env::var("SNAP_DATA") {
        Some(Path::new(snap_data.as_str()).to_path_buf())
    } else {
        None
    }
}

/// The name of snap instance, including instance key if one is set (snapd 2.36+).
/// For example snap `hello-world` with instance key `foo` has instance name equal to `hello-world_foo`.
///
/// Typical value: `hello-world`
pub fn snap_instance_name() -> Option<String> {
    if let Ok(snap_instance_name) = env::var("SNAP_INSTANCE_NAME") {
        Some(snap_instance_name)
    } else {
        None
    }
}

/// Instance key if one was set during installation or empty (snapd 2.36+).
/// For example instance `hello-world_foo` has an instance key `foo`.
///
/// Typical value: none
pub fn snap_instance_key() -> Option<String> {
    if let Ok(snap_instance_key) = env::var("SNAP_INSTANCE_KEY") {
        Some(snap_instance_key)
    } else {
        None
    }
}

/// Directory with additional system libraries. This variable is used internally by snapcraft.
/// The value is always `/var/lib/snapd/lib/gl:` Please note the colon at the end of that value,
/// the variable is a colon-separated list.
///
/// The referenced directory is typically empty unless Nvidia proprietary drivers are in use.
pub fn snap_library_path() -> Option<Vec<PathBuf>> {
    if let Ok(snap_real_home) = env::var("SNAP_LIBRARY_PATH") {
        let snap_lib_paths: Vec<PathBuf> = snap_real_home
            .split(':')
            .filter(|x| !x.is_empty())
            .map(|x| Path::new(x).to_path_buf())
            .collect();
        Some(snap_lib_paths)
    } else {
        None
    }
}

/// The name of the snap as specified in the `snapcraft.yaml` file.
///
/// Typical value: `hello-world`
pub fn snap_name() -> Option<String> {
    if let Ok(snap_name) = env::var("SNAP_NAME") {
        Some(snap_name)
    } else {
        None
    }
}

/// The vanilla `HOME` environment variable before snapd-induced remapping, refer
/// [Any way to acquire the originally set HOME environment variable? - snapcraft snapcraft.io](https://forum.snapcraft.io/t/any-way-to-acquire-the-originally-set-home-environment-variable/19475)
/// for more info.
///
/// Available since `snapd 2.46`.
pub fn snap_real_home() -> Option<PathBuf> {
    if let Ok(snap_real_home) = env::var("SNAP_REAL_HOME") {
        Some(Path::new(snap_real_home.as_str()).to_path_buf())
    } else {
        None
    }
}

/// Revision of the snap, as allocated by the Snap Store on upload or as allocated by snapd for locally installed snaps.
/// The Snap Store assigns monotonic revisions to each upload of a given snap.
/// Snapd uses Snap Store revisions if accompanying assertions are available or uses a locally generated number.
/// Locally generated numbers are prefixed with `x` to distinguish them from Snap Store uploads.
///
/// Typical value: `27` or `x1`
pub fn snap_revision() -> Option<String> {
    if let Ok(snap_revision) = env::var("SNAP_REVISION") {
        Some(snap_revision)
    } else {
        None
    }
}

/// This variable is only exposed on [Ubuntu Core](https://snapcraft.io/docs/glossary#heading--ubuntu-core) systems, and
/// was introduced with snapd 2.57.   It points to a snap-specific location on the ubuntu-save partition where the snap
/// is allowed to store persistent files (like certificates or configuration files) that will survive a
/// [factory reset](https://ubuntu.com/core/docs/recovery-modes#heading--factory) of the Ubuntu Core device.
///
/// See [ubuntu-save](https://ubuntu.com/core/docs/storage-layout#heading--save) in the Ubuntu Core documentation for
/// more details on storage layout with this specific partition.
pub fn snap_save_data() -> Option<PathBuf> {
    if let Ok(snap_save_data) = env::var("SNAP_SAVE_DATA") {
        Some(Path::new(snap_save_data.as_str()).to_path_buf())
    } else {
        None
    }
}

/// Directory for user data that is common across revisions of a snap.   Unlike `SNAP_DATA`, data present in this
/// directory is not backed up or restored across snap refresh and snap revert operations. The directory is suitable for
/// large data that the application can access even if it was made or modified by a future version of a snap.
///
/// Typical value `/home/zyga/snap/hello-world/common`
pub fn snap_user_common() -> Option<PathBuf> {
    if let Ok(snap_user_common) = env::var("SNAP_USER_COMMON") {
        Some(Path::new(snap_user_common.as_str()).to_path_buf())
    } else {
        None
    }
}

/// Directory for user data.
/// This directory is backed up and restored across `snap refresh` and `snap revert` operations.
/// Typical value: `/home/zyga/snap/hello-world/27`
///
/// The final number there is `$SNAP_REVISION`.
pub fn snap_user_data() -> Option<PathBuf> {
    if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
        Some(Path::new(snap_user_data.as_str()).to_path_buf())
    } else {
        None
    }
}

/// The version string as specified in the `snapcraft.yaml`
///
/// Typical value `6.3`
pub fn snap_version() -> Option<String> {
    if let Ok(snap_version) = env::var("SNAP_VERSION") {
        Some(snap_version)
    } else {
        None
    }
}

/// A map of all of the environment variables that start with `SNAP_`
pub fn snap_env() -> Option<HashMap<String, String>> {
    let snap_map: HashMap<String, String> = env::vars()
        .into_iter()
        .filter(|(k, _)| k.starts_with("SNAP"))
        .collect();
    if snap_map.is_empty() {
        None
    } else {
        Some(snap_map)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::{env, path::Path};

    fn setup() {
        env::set_var("SNAP", "/snap/hello-world/27");
        env::set_var("SNAP_ARCH", "armhf");
        env::set_var("SNAP_COMMON", "/var/snap/hello-world/common");
        env::set_var("SNAP_DATA", "/var/snap/hello-world/27");
        env::set_var("SNAP_INSTANCE_NAME", "hello-world");
        env::set_var("SNAP_INSTANCE_KEY", "foo");
        env::set_var("SNAP_LIBRARY_PATH", "/var/lib/snapd/lib/gl:");
        env::set_var("SNAP_NAME", "hello-world");
        env::set_var("SNAP_REAL_HOME", "/home/user");
        env::set_var("SNAP_REVISION", "27");
        env::set_var("SNAP_SAVE_DATA", "/snap/hello-world/27/save");
        env::set_var("SNAP_USER_COMMON", "/home/zyga/snap/hello-world/common");
        env::set_var("SNAP_USER_DATA", "/home/zyga/snap/hello-world/27");
        env::set_var("SNAP_VERSION", "v1.0.0");
    }

    fn unsetup() {
        env::vars()
            .into_iter()
            .filter(|(k, _)| k.starts_with("SNAP"))
            .for_each(|(k, _)| env::remove_var(k));
    }

    #[test]
    #[serial]
    fn snap() {
        setup();
        let val = crate::snap();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "/snap/hello-world/27");
        unsetup();
    }

    #[test]
    #[serial]
    fn in_snap() {
        setup();
        let val = crate::in_snap();
        assert!(val);
        unsetup();
    }

    #[test]
    #[serial]
    fn not_in_snap() {
        let val = crate::in_snap();
        assert!(!val);
    }

    #[test]
    #[serial]
    fn check_snap_home() {
        setup();
        let val = crate::check_snap_home();
        assert!(val.0);
        assert!(val.1.is_some());
        assert_eq!(val.1.unwrap(), Path::new("/home/user").to_path_buf());
        unsetup();
    }

    #[test]
    #[serial]
    fn not_check_snap_home() {
        let val = crate::check_snap_home();
        assert!(!val.0);
        assert!(val.1.is_none());
    }

    #[test]
    #[serial]
    fn snap_arch() {
        setup();
        let val = crate::snap_arch();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "armhf");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_data() {
        setup();
        let val = crate::snap_data();
        assert!(val.is_some());
        assert_eq!(
            val.unwrap(),
            Path::new("/var/snap/hello-world/27").to_path_buf()
        );
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_instance_name() {
        setup();
        let val = crate::snap_instance_name();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "hello-world");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_instance_key() {
        setup();
        let val = crate::snap_instance_key();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "foo");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_library_path() {
        setup();
        let val = crate::snap_library_path();
        assert!(val.is_some());
        let val = val.unwrap();
        assert_eq!(val.len(), 1);
        assert_eq!(
            val.first().unwrap().to_owned(),
            Path::new("/var/lib/snapd/lib/gl").to_path_buf()
        );
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_name() {
        setup();
        let val = crate::snap_name();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "hello-world");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_real_home() {
        setup();
        let val = crate::snap_real_home();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), Path::new("/home/user").to_path_buf());
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_revision() {
        setup();
        let val = crate::snap_revision();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "27");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_save_data() {
        setup();
        let val = crate::snap_save_data();
        assert!(val.is_some());
        assert_eq!(
            val.unwrap(),
            Path::new("/snap/hello-world/27/save").to_path_buf()
        );
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_user_common() {
        setup();
        let val = crate::snap_user_common();
        assert!(val.is_some());
        assert_eq!(
            val.unwrap(),
            Path::new("/home/zyga/snap/hello-world/common").to_path_buf()
        );
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_user_data() {
        setup();
        let val = crate::snap_user_data();
        assert!(val.is_some());
        assert_eq!(
            val.unwrap(),
            Path::new("/home/zyga/snap/hello-world/27").to_path_buf()
        );
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_version() {
        setup();
        let val = crate::snap_version();
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "v1.0.0");
        unsetup();
    }

    #[test]
    #[serial]
    fn snap_env() {
        setup();
        let val = crate::snap_env();
        assert!(val.is_some());
        if let Some(val) = val {
            assert_eq!(val.len(), 14);
            assert_eq!(val.get("SNAP").unwrap(), "/snap/hello-world/27");
            assert_eq!(val.get("SNAP_ARCH").unwrap(), "armhf");
            assert_eq!(
                val.get("SNAP_COMMON").unwrap(),
                "/var/snap/hello-world/common"
            );
            assert_eq!(val.get("SNAP_DATA").unwrap(), "/var/snap/hello-world/27");
            assert_eq!(val.get("SNAP_INSTANCE_NAME").unwrap(), "hello-world");
            assert_eq!(val.get("SNAP_INSTANCE_KEY").unwrap(), "foo");
            assert_eq!(
                val.get("SNAP_LIBRARY_PATH").unwrap(),
                "/var/lib/snapd/lib/gl:"
            );
            assert_eq!(val.get("SNAP_NAME").unwrap(), "hello-world");
            assert_eq!(val.get("SNAP_REAL_HOME").unwrap(), "/home/user");
            assert_eq!(val.get("SNAP_REVISION").unwrap(), "27");
            assert_eq!(
                val.get("SNAP_SAVE_DATA").unwrap(),
                "/snap/hello-world/27/save"
            );
            assert_eq!(
                val.get("SNAP_USER_COMMON").unwrap(),
                "/home/zyga/snap/hello-world/common"
            );
            assert_eq!(
                val.get("SNAP_USER_DATA").unwrap(),
                "/home/zyga/snap/hello-world/27"
            );
            assert_eq!(val.get("SNAP_VERSION").unwrap(), "v1.0.0");
        }
        unsetup();
    }

    #[test]
    #[serial]
    fn no_snap_env() {
        unsetup();
        let val = crate::snap_env();
        assert!(val.is_none());
    }
}
