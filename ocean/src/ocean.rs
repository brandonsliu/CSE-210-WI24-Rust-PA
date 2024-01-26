use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    beaches: Vec<Beach>,
    reefs: Vec<Rc<RefCell<Reef>>>,

}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean {
            beaches: Vec::new(),
            reefs: Vec::new(),
        }
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let mut new_reef = Reef::new();
        for n in 0..n_minnows {
            let new_prey = Box::new(Minnow::new(25));
            new_reef.add_prey(new_prey);
        }
        for n in 0..n_shrimp {
            let new_prey = Box::new(Shrimp::new(1));
            new_reef.add_prey(new_prey);
        }
        for n in 0..n_clams {
            let new_prey = Box::new(Clam::new());
            new_reef.add_prey(new_prey);
        }
        for n in 0..n_algae {
            let new_prey = Box::new(Algae::new());
            new_reef.add_prey(new_prey);
        }
        let ref_cell = RefCell::new(new_reef);
        let new_rc = Rc::new(ref_cell);
        let second_rc = Rc::clone(&new_rc);
        self.reefs.push(second_rc);
        return new_rc
    }
}
