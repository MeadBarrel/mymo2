pub mod model;
pub mod error;

pub use indexmap;
pub use enum_map;
pub use strum;

use model::Skill;

pub fn load_skills(path: &str) -> Result<indexmap::IndexMap<String, Skill>, error::Error> {
    let file = std::fs::File::open(path)
        .map_err(|_| error::Error::LoadingFailed("failed to open file".to_owned()))?;
    let vec: Vec<Skill> = serde_yaml::from_reader(file)
        .map_err(|_| error::Error::LoadingFailed("failed to deserialize".to_owned()))?;
    Ok(vec.into_iter().map(|x| (x.name.clone(), x)).collect())
}