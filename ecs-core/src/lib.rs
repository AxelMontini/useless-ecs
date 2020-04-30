use core::any::TypeId;
use core::mem;
use core::ops::{Index, IndexMut};
use std::collections::HashMap;

#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct Entity(u32);

/// Trait that components must implement. It does nothing for now, but it will do stuff later (probably) (maybe)
pub trait Component {}

/// Storage trait, stored in the World. Can be downcasted to a `StorageImpl<C>`
/// using `Storage::downcast`
pub trait Storage {
    fn type_id(&self) -> TypeId;
}

/// Actual storage type, holding a vector of `Option<T>`s. Will be downcasted from a Box<dyn Storage>
#[derive(Debug)]
#[doc(hidden)]
pub struct StorageImpl<C: Component> {
    list: Vec<Option<C>>,
}

impl<C: Component> Default for StorageImpl<C> {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl<C: Component> StorageImpl<C> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, entity: Entity, component: C) {
        if self.list.len() <= entity.0 as _ {
            self.list.resize_with((entity.0 + 1) as _, Default::default)
        }

        mem::replace(self.index_mut(entity), Some(component));
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        self.list
            .get(entity.0 as usize)
            .and_then(|maybe_component| maybe_component.as_ref())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.list
            .get_mut(entity.0 as usize)
            .and_then(|maybe_component| maybe_component.as_mut())
    }

    /// Remove component of given entity and replaces it with `None`. Then returns
    /// the removed component, if any.
    pub fn remove(&mut self, entity: Entity) -> Option<C> {
        mem::replace(self.index_mut(entity), None)
    }
}

impl<C: Component> Index<Entity> for StorageImpl<C> {
    type Output = Option<C>;

    fn index(&self, entity: Entity) -> &Self::Output {
        self.list.index(entity.0 as usize)
    }
}

impl<C: Component> IndexMut<Entity> for StorageImpl<C> {
    fn index_mut(&mut self, entity: Entity) -> &mut Self::Output {
        self.list.index_mut(entity.0 as usize)
    }
}

impl dyn Storage + 'static {
    fn is<S: Storage + 'static>(&self) -> bool {
        let type_of_param = TypeId::of::<S>();
        let type_of_self = self.type_id();

        println!("TYPEOF SELF: {:?}", type_of_self);
        println!("TYPEOF OTHER: {:?}", type_of_param);

        type_of_self == type_of_param
    }

    /// Tries to downcast this trait object to an immutable reference to a concrete storage.
    ///
    /// Pretty sure this is safe.
    ///
    /// We can't both immutably and mutably borrow.
    ///
    /// Also the reference lives just as long as self;
    #[allow(clippy::cast_ptr_alignment)]
    pub fn downcast_ref<T: Component + 'static>(&self) -> Option<&StorageImpl<T>> {
        if self.is::<StorageImpl<T>>() {
            unsafe { Some(&*(self as *const dyn Storage as *const StorageImpl<T>)) }
        } else {
            None
        }
    }

    /// Tries to downcast this trait object to a mutable reference to a concrete storage.
    ///
    /// Pretty sure this is safe.
    ///
    /// We cannot mutably borrow twice, thus there's no way to have dangling pointers in the vec.
    ///
    /// Also the reference lives just as long as self;
    #[allow(clippy::cast_ptr_alignment)]
    pub fn downcast_mut<T: Component + 'static>(&mut self) -> Option<&mut StorageImpl<T>> {
        if self.is::<StorageImpl<T>>() {
            unsafe { Some(&mut *(self as *mut dyn Storage as *mut StorageImpl<T>)) }
        } else {
            None
        }
    }
}

impl<C: Component + 'static> Storage for StorageImpl<C> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// World holds all storages and entities.
/// It also provides ways to create/remove/modify entities and components.
pub struct World {
    entities: Vec<Entity>,
    storages: HashMap<TypeId, Box<dyn Storage>>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            storages: HashMap::new(),
        }
    }
}

impl World {
    pub fn storage<C: Component + 'static>(&self) -> Option<&StorageImpl<C>> {
        self.storages
            .get(&TypeId::of::<C>())
            .and_then(|storage| storage.downcast_ref())
    }

    pub fn storage_mut<C: Component + 'static>(&mut self) -> Option<&mut StorageImpl<C>> {
        self.storages
            .get_mut(&TypeId::of::<C>())
            .and_then(|storage| storage.downcast_mut())
    }

    /// Returns true whether the storage of given type exists.
    pub fn has_storage<C: Component + 'static>(&self) -> bool {
        self.storages.contains_key(&TypeId::of::<C>())
    }

    /// Creates storage is it doesn't exist and then returns it.
    pub fn ensure_storage_exists<C: Component + 'static>(&mut self) -> &mut StorageImpl<C> {
        if !self.has_storage::<C>() {
            self.storages
                .insert(TypeId::of::<C>(), Box::new(StorageImpl::<C>::new()));
        }

        self.storage_mut::<C>().unwrap()
    }

    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new entity in the world and returns it.
    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity(self.entities.len() as _);
        self.entities.push(entity);

        entity
    }
}
