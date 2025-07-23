use anyhow::Result;
use dashmap::DashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::NarrativeData;

#[derive(Clone)]
pub struct NarrativeStore {
    narratives: Arc<DashMap<Uuid, NarrativeData>>,
    storage_path: PathBuf,
}

impl NarrativeStore {
    pub fn new(storage_path: PathBuf) -> Self {
        let store = Self {
            narratives: Arc::new(DashMap::new()),
            storage_path,
        };

        // Load existing narratives
        let _ = store.load_from_disk();

        store
    }

    fn storage_file(&self) -> PathBuf {
        self.storage_path.join(".weaver-narratives.json")
    }

    fn save_to_disk(&self) -> Result<()> {
        let narratives: Vec<NarrativeData> = self.list();
        let json = serde_json::to_string_pretty(&narratives)?;
        fs::write(self.storage_file(), json)?;
        Ok(())
    }

    fn load_from_disk(&self) -> Result<()> {
        let path = self.storage_file();
        if !path.exists() {
            return Ok(());
        }

        let json = fs::read_to_string(path)?;
        let narratives: Vec<NarrativeData> = serde_json::from_str(&json)?;

        for narrative in narratives {
            self.narratives.insert(narrative.id, narrative);
        }

        Ok(())
    }

    pub fn create(&self, mut narrative: NarrativeData) -> Result<NarrativeData> {
        narrative.id = Uuid::new_v4();
        narrative.created_at = chrono::Utc::now();
        narrative.updated_at = chrono::Utc::now();

        self.narratives.insert(narrative.id, narrative.clone());
        self.save_to_disk()?;
        Ok(narrative)
    }

    pub fn get(&self, id: &Uuid) -> Option<NarrativeData> {
        self.narratives.get(id).map(|entry| entry.clone())
    }

    pub fn update(&self, id: &Uuid, mut narrative: NarrativeData) -> Result<NarrativeData> {
        narrative.id = *id;
        narrative.updated_at = chrono::Utc::now();

        self.narratives.insert(*id, narrative.clone());
        self.save_to_disk()?;
        Ok(narrative)
    }

    pub fn delete(&self, id: &Uuid) -> Result<()> {
        self.narratives.remove(id);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn list(&self) -> Vec<NarrativeData> {
        self.narratives
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}
