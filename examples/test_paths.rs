use kodegen_config::KodegenConfig;

fn main() {
    env_logger::init();
    
    println!("Testing platform path resolution with validation...\n");
    
    match KodegenConfig::user_config_dir() {
        Ok(path) => println!("config_dir: {}", path.display()),
        Err(e) => println!("config_dir ERROR: {}", e),
    }
    
    match KodegenConfig::state_dir() {
        Ok(path) => println!("state_dir: {}", path.display()),
        Err(e) => println!("state_dir ERROR: {}", e),
    }
    
    match KodegenConfig::data_dir() {
        Ok(path) => println!("data_dir: {}", path.display()),
        Err(e) => println!("data_dir ERROR: {}", e),
    }
}
