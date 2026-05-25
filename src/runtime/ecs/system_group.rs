use std::sync::{Arc, RwLock};
use std::thread;

use crate::runtime::entities::{DynamicWorld, SystemBase};

const MAX_SYSTEMS_PER_GROUP: usize = 256;
pub struct SystemGroup {
    //systems: RwLock<Vec<Box<dyn SystemBase>>>,
    threading: SystemGroupThreading,
    systems: Arc<RwLock<Vec<Box<dyn SystemBase>>>>,

    /// Lower numbers go first and higher ones go last
    system_update_order: [i32; MAX_SYSTEMS_PER_GROUP],
    world: Arc<DynamicWorld>,
}

/// All systems registered to a system group run on the same thread
/// Multiple system groups can share a world
/// Avoid haveing too many system group because each one takes a thread
impl SystemGroup {
    pub fn new(world: Arc<DynamicWorld>, threading: SystemGroupThreading) -> Self {
        Self {
            threading: threading,
            system_update_order: [i32::MAX; MAX_SYSTEMS_PER_GROUP],
            systems: Arc::new(RwLock::new(Vec::new())),
            world,
        }
    }
    pub fn update(&mut self) {
        match self.threading {
            SystemGroupThreading::Main => self.run_systems(),
            SystemGroupThreading::Parallel => self.run_systems_parrallel(),
        }
    }
    fn order_systems(&mut self) {
        let mut systems = self.systems.write().unwrap();
        let len = systems.len();
        let mut indices: Vec<usize> = (0..len).collect();
        indices.sort_by_key(|&i| self.system_update_order[i]);
        let mut visited = [false; MAX_SYSTEMS_PER_GROUP];
        for start in 0..len {
            if visited[start] || indices[start] == start {
                visited[start] = true;
                continue;
            }
            let current = start;
            while !visited[current] {
                visited[current] = true;
                let next = indices[current];
                if next != current {
                    systems.swap(current, next);
                    self.system_update_order.swap(current, next);
                    indices[current] = indices[next];
                    indices[next] = next;
                }
            }
        }
    }
    /// Registers a system and returns it's index in the group
    /// The systems will run in the order registered
    /// Calls on_start() for the system
    pub fn register_system(
        &mut self,
        system: Box<dyn SystemBase + Send + Sync>,
        order: i32,
    ) -> usize {
        let mut systems = self.systems.write().unwrap();
        let index = systems.len();
        systems.push(system);
        self.system_update_order[index] = order;
        index
    }
    pub fn destroy_system(&mut self, system_index: usize) {
        let mut systems = self.systems.write().unwrap();
        systems[system_index].on_destroy(&self.world);
        systems.remove(system_index);
    }

    pub fn start_systems(&mut self) {
        self.order_systems();
        for system in self.systems.write().unwrap().iter_mut() {
            system.on_start(&self.world);
        }
    }
    pub fn destroy_systems(&mut self) {
        self.order_systems();
        for system in self.systems.write().unwrap().iter_mut() {
            system.on_destroy(&self.world);
        }
    }
    /// Runs system on the main thread
    pub fn run_systems(&mut self) {
        //println!("Running system group with {} systems", self.systems.read().unwrap().len());
        for system in self.systems.write().unwrap().iter_mut() {
            system.on_update(&self.world);
        }
    }
    /// Runs system on a worker thread
    pub fn run_systems_parrallel(&self) {
        let systems = Arc::clone(&self.systems);
        let world = Arc::clone(&self.world);

        thread::spawn(move || {
            let mut systems = systems.write().unwrap();
            for system in systems.iter_mut() {
                system.on_update(&world);
            }
        });
    }
}

pub enum SystemGroupThreading {
    Main,
    Parallel,
}
