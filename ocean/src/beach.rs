use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    size: usize,
    crab_collection: Vec<Crab>,
    clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach {
            size: 0,
            crab_collection: Vec::new(), 
            clan_system: ClanSystem::new(),
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.size
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crab_collection.push(crab);
        self.size += 1;
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &(self.crab_collection[index])
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crab_collection.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        let mut best_crab = None;
        let mut highest_speed = 0;
        for crab in &self.crab_collection {
            if crab.speed() > highest_speed {
                best_crab = Some(crab);
                highest_speed = crab.speed();
            }
        }
        best_crab

    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crab_vec = Vec::new();
        for crab in &self.crab_collection {
            if crab.name() == name {
                crab_vec.push(crab);
            }
        }
        crab_vec
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i < self.crab_collection.len() && j < self.crab_collection.len() {
            let new_crab = self.get_crab(i).breed(self.get_crab(j), name);
            self.add_crab(new_crab);
        }
        else {
            panic!("Index out of bounds");
        }
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        //Check if crab_name already exists in clan_system
        let already_contains = self.clan_system.clans.values().any(|l| l.contains(&crab_name.to_string()));
        if !already_contains {
            self.clan_system.add_clan_member(clan_id, crab_name);
        }
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let mut average_one = 0;
        let mut average_two = 0;
        let names_one = self.clan_system.get_clan_member_names(id1);
        let names_two = self.clan_system.get_clan_member_names(id2);
        for name in names_one {
            let crab_one = self.find_crabs_by_name(&name)[0];
            average_one += crab_one.speed();
        }
        match self.clan_system.get_clan_member_count(id1) {
            0 => {
                return Err("id1 doesn't exist".to_string())
            }
            val => {
                average_one = average_one / val as u32;

            }
        }
        for name in names_two {
            let crab_two = self.find_crabs_by_name(&name)[0];
            average_two += crab_two.speed();
        }
        match self.clan_system.get_clan_member_count(id2) {
            0 => {
                return Err("id2 doesn't exist".to_string())
            }
            val => {
                average_two = average_two / val as u32;

            }
        }
        if average_one > average_two {
            return Ok(Some(id1.to_string()))
        }
        if average_one < average_two {
            return Ok(Some(id2.to_string()))
        }
        return Ok(None)
    }
}
