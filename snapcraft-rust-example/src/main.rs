use snapcraft;

fn main() {
    if snapcraft::in_snap() { 
        println!("Running within a snap");
    } else {
        println!("Not running within a snap");
    }
    println!("SNAP:                 {:?}", snapcraft::snap());
    println!("SNAP_ARCH:            {:?}", snapcraft::snap_arch());
    println!("SNAP_COMMON:          {:?}", snapcraft::snap_common());
    println!("SNAP_DATA:            {:?}", snapcraft::snap_data());
    println!("SNAP_INSTANCE_NAME:   {:?}", snapcraft::snap_instance_name());
    println!("SNAP_INSTANCE_KEY:    {:?}", snapcraft::snap_instance_key());
    println!("SNAP_LIBRARY_PATH:    {:?}", snapcraft::snap_library_path());
    println!("SNAP_NAME:            {:?}", snapcraft::snap_name());
    println!("SNAP_REAL_HOME:       {:?}", snapcraft::snap_real_home());
    println!("SNAP_REVISION:        {:?}", snapcraft::snap_revision());
    println!("SNAP_SAVE_DATA:       {:?}", snapcraft::snap_save_data());
    println!("SNAP_USER_COMMON:     {:?}", snapcraft::snap_user_common());
    println!("SNAP_USER_DATA:       {:?}", snapcraft::snap_user_data());
    println!("SNAP_VERSION:         {:?}", snapcraft::snap_version());    
}
