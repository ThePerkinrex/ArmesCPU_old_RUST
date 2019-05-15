mod config;
pub use config::Config;

pub fn run_cpu(config: Config){
    println!("Initializing {}", config.name)
}