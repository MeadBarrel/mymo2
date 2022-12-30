use uuid::Uuid;
use eframe::egui::Id;

pub struct SuffixedId {
    uuid: Uuid,
    suffix: String,
}

impl SuffixedId {
    fn derive(&self, suffix: &str) -> Self {
        let mut new_suffix = self.suffix.clone();
        new_suffix.push_str(suffix);
        Self {
            uuid: self.uuid,
            suffix: new_suffix,
        }
    }

    fn id(&self) -> Id {
        Id::new((self.uuid, &self.suffix))
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