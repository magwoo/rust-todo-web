use once_cell::sync::Lazy;
use serde_json::from_slice as deserialize;
use serde_json::to_vec as serialize;
use sled::Db;

use crate::prelude::*;

static DB: Lazy<Db> = Lazy::new(|| sled::open("database").unwrap());

impl Task {
    pub fn insert_db(&self) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let tree = DB.open_tree("tasks")?;
        let data = serialize(self)?;
        let previous = tree.insert(&self.title, data)?;
        match previous {
            Some(data) => Ok(Some(deserialize::<Task>(&data)?)),
            None => Ok(None),
        }
    }

    pub fn get_all_db() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let tree = DB.open_tree("tasks")?;
        let mut result = Vec::new();
        for data in tree.iter() {
            let task = deserialize::<Task>(&data?.1)?;
            result.push(task);
        }
        Ok(result)
    }
}
