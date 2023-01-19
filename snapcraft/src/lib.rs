use std::env;
use std::path::{Path, PathBuf};

/// Checks to see whether we are operating within a snap
pub fn check_snap_home() -> (bool, Option<PathBuf>) {
    if in_snap() {
        (true, snap_real_home())
    } else {
        (false, None)
    }
}

/// Checks whether it thinks this is running within a snap
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
/// The value is always `/var/lib/snapd/lib/gl`: Please note the colon at the end of that value,
/// the variable is a colon-separated list.
///
/// The referenced directory is typically empty unless Nvidia proprietary drivers are in use.
pub fn snap_library_path() -> Option<PathBuf> {
    if let Ok(snap_real_home) = env::var("SNAP_LIBRARY_PATH") {
        Some(Path::new(snap_real_home.as_str()).to_path_buf())
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

/// The vanilla `HOME` environment variable before snapd-induced remapping, refer [Any way to acquire the originally set HOME environment variable? - snapcraft - snapcraft.io](https://forum.snapcraft.io/t/any-way-to-acquire-the-originally-set-home-environment-variable/19475)
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

/// This variable is only exposed on [Ubuntu Core](https://snapcraft.io/docs/glossary#heading--ubuntu-core) systems, and was introduced with snapd 2.57.
/// It points to a snap-specific location on the ubuntu-save partition where the snap is allowed to store persistent files (like certificates or configuration files) that will survive a [factory reset](https://ubuntu.com/core/docs/recovery-modes#heading--factory) of the Ubuntu Core device.
///
/// See [ubuntu-save](https://ubuntu.com/core/docs/storage-layout#heading--save) in the Ubuntu Core documentation for more details on storage layout with this specific partition.
pub fn snap_save_data() -> Option<String> {
    if let Ok(snap_save_data) = env::var("SNAP_SAVE_DATA") {
        Some(snap_save_data)
    } else {
        None
    }
}

/// Directory for user data that is common across revisions of a snap.
/// Unlike `SNAP_DATA`, data present in this directory is not backed up or restored across snap refresh and snap revert operations. The directory is suitable for large data that the application can access even if it was made or modified by a future version of a snap.
///
/// Typical value `/home/zyga/snap/hello-world/common`
pub fn snap_user_common() -> Option<String> {
    if let Ok(snap_user_common) = env::var("SNAP_USER_COMMON") {
        Some(snap_user_common)
    } else {
        None
    }
}

/// Directory for user data.
/// This directory is backed up and restored across `snap refresh` and `snap revert` operations.
/// Typical value: `/home/zyga/snap/hello-world/27`
///
/// The final number there is `$SNAP_REVISION`.
pub fn snap_user_data() -> Option<String> {
    if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
        Some(snap_user_data)
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
