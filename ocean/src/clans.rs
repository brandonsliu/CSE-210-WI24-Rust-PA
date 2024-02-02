use crate::crab::Crab;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    pub clans: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {
            clans: HashMap::new(),
        }
    }

    pub fn add_clan_member(&mut self, clan_id: &str, crab_name: &str) {
        let entry = self.clans.entry(clan_id.to_string()).or_insert(vec![]);
        entry.push(crab_name.to_string());
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        match self.clans.get(clan_id) {
            Some(vec) => {
                return vec.clone()
            }
            _ => {
                return Vec::new()
            }
        }
    }


    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        match self.clans.get(clan_id) {
            Some(vec) => {
                return vec.len()
            }
            None => {
                return 0
            }
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest = 0;
        let mut largest_clan_id = None;
        for (id, vec) in self.clans.iter(){
            if vec.len() > largest {
                largest = vec.len();
                largest_clan_id = Some(id.clone());
            }
            else if vec.len() == largest {
                largest_clan_id = None;
            }
        }
        largest_clan_id
    }
}