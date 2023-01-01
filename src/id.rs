use uuid::Uuid;
use eframe::egui::Id;

#[derive(Debug, Hash)]
pub struct SuffixedId {
    uuid: Uuid,
    suffix: String,
}

impl SuffixedId {
    pub fn derive(&self, suffix: &str) -> Self {
        let mut new_suffix = self.suffix.clone();
        new_suffix.push_str(suffix);
        Self {
            uuid: self.uuid,
            suffix: new_suffix,
        }
    }

    pub fn id(&self) -> Id {
        Id::new(self)
    }
}

impl Default for SuffixedId {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            suffix: "".to_string(),
        }
    }
}

impl From<SuffixedId> for Id {
    fn from(value: SuffixedId) -> Self {
        value.id()
    }
}