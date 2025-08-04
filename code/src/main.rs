mod config;
mod get_files;
mod qc;

fn main() {
    // Initialize the base path based on the system
    let system = std::env::args().nth(1).expect("Please provide the system type as an argument");
    config::init(&system);
    
    }
}
