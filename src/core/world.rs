//! World and Entity Component Store.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// Unique identifier for any biological entity in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u64);

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

/// Generic container for entities of type `T` keyed by `EntityId`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityStore<T> {
    entities: BTreeMap<EntityId, T>,
}

impl<T> Default for EntityStore<T> {
    fn default() -> Self {
        Self {
            entities: BTreeMap::new(),
        }
    }
}

impl<T> EntityStore<T> {
    /// Creates a new empty entity store.
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::new(),
        }
    }

    /// Inserts an entity with given ID.
    pub fn insert(&mut self, id: EntityId, entity: T) -> Option<T> {
        self.entities.insert(id, entity)
    }

    /// Retrieves a reference to an entity.
    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.entities.get(&id)
    }

    /// Retrieves a mutable reference to an entity.
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    /// Removes an entity by ID.
    pub fn remove(&mut self, id: EntityId) -> Option<T> {
        self.entities.remove(&id)
    }

    /// Returns iterator over all entities.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.entities.values()
    }

    /// Returns mutable iterator over all entities.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.entities.values_mut()
    }

    /// Returns iterator over (EntityId, &T) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.entities.iter().map(|(&k, v)| (k, v))
    }

    /// Returns mutable iterator over (EntityId, &mut T) pairs.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.entities.iter_mut().map(|(&k, v)| (k, v))
    }

    /// Returns the number of entities in the store.
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

/// World container managing entity identity allocation and lifecycle.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct World {
    next_id: u64,
}

impl World {
    /// Creates a new empty world.
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Allocates a new unique `EntityId`.
    pub fn next_entity_id(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_id_allocation() {
        let mut world = World::new();
        let e1 = world.next_entity_id();
        let e2 = world.next_entity_id();
        assert_eq!(e1, EntityId(1));
        assert_eq!(e2, EntityId(2));
        assert_ne!(e1, e2);
    }

    #[test]
    fn test_entity_store() {
        let mut store = EntityStore::new();
        let id = EntityId(42);
        store.insert(id, "CellAlpha");
        assert_eq!(store.get(id), Some(&"CellAlpha"));
        assert_eq!(store.len(), 1);
        store.remove(id);
        assert_eq!(store.len(), 0);
    }
}
