pub mod pipeline;
use crate::pipeline::{events::*, Pipeline};

pub fn read_config(path: String) -> Result<Pipeline, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let pipeline: Pipeline = serde_yaml::from_reader(reader)?;
    Ok(pipeline)
}

pub fn write_config(path: String, pipeline: Pipeline) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_yaml::to_writer(writer, &pipeline)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pipeline::{events::OnAction, metadata::Metadata, station::Station, Validate};

    use super::*;
    #[test]
    fn test_read_config() {
        let pipeline = read_config("../docs/pipelines/example.yaml".to_string()).unwrap();

        assert_eq!(
            pipeline.metadata.name.unwrap(),
            "This is my very descriptive name"
        );
        assert_eq!(pipeline.stations.len(), 1);

        let station = &pipeline.stations[0];

        assert_eq!(station.steps.len(), 2);
    }

    #[test]
    fn test_validate_config() {
        let mut pipeline = read_config("../docs/pipelines/example.yaml".to_string()).unwrap();
        assert!(pipeline.validate().is_ok());
    }
}
