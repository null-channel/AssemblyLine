pub mod server;
use server::ServerConfig;

pub fn read_config(path: String) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let server: ServerConfig = serde_yaml::from_reader(reader)?;
    Ok(server)
}
