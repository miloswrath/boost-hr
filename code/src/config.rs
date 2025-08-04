use once_cell::sync::OnceCell;

static BASE_PATH: OnceCell<String> = OnceCell::new();

pub fn init(system: &str) {
    let path = match system {
        "linux" => "/mnt/lss/Projects/BOOST/".to_string(),
        "vosslnx" => "/mnt/nfs/lss/Projects/BOOST".to_string(),
        "argon" => "/Shared/vosslabhpc/Projects/BOOST".to_string(),
        _ => panic!("Unsupported system: {}", system),
    };
    BASE_PATH.set(base_path).expect("Failed to set base path");
}

pub fn get_base_path() -> &'static str {
    BASE_PATH.get().expect("Base path not initialized")
}
