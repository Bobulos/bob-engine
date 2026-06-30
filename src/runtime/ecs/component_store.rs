use std::any::Any;

/// Stores components of type `T` densely indexed by entity ID.
pub struct ComponentStore<T: Any + Default> {
    components: Vec<Option<T>>,
}

impl<T: Any + Default> ComponentStore<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity_id: usize, component: T) {
        if entity_id >= self.components.len() {
            self.components.resize_with(entity_id + 1, || None);
        }
        self.components[entity_id] = Some(component);
    }

    pub fn remove(&mut self, entity_id: usize) {
        if let Some(slot) = self.components.get_mut(entity_id) {
            *slot = None;
        }
    }

    pub fn get(&self, entity_id: usize) -> Option<&T> {
        self.components.get(entity_id)?.as_ref()
    }

    pub fn get_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        self.components.get_mut(entity_id)?.as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.components
            .iter()
            .enumerate()
            .filter_map(|(id, c)| c.as_ref().map(|c| (id, c)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.components
            .iter_mut()
            .enumerate()
            .filter_map(|(id, c)| c.as_mut().map(|c| (id, c)))
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }
}
pub trait AnyComponentStore: Any + Send + Sync {
    fn insert_default(&mut self, entity: usize);
    fn remove(&mut self, entity: usize);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Send + Sync> AnyComponentStore for ComponentStore<T>
where
    T: Default + Any + Send + Sync + 'static,
{
    fn remove(&mut self, entity: usize) {
        self.remove(entity);
    }
    fn insert_default(&mut self, entity: usize) {
        self.insert(entity, T::default());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
