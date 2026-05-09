use std::marker::PhantomData;

pub trait BuilderOutput {
    type Output;
    fn build(self) -> Self::Output;
}

pub struct GenericBuilder<T, O> {
    data: T,
    _output: PhantomData<O>,
}

impl<T: Clone, O: BuilderOutput<Output = O>> GenericBuilder<T, O>
where
    T: Clone,
{
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            data,
            _output: PhantomData,
        }
    }

    #[inline]
    pub fn with<F: Fn(T) -> T>(mut self, f: F) -> Self {
        self.data = f(self.data);
        self
    }

    #[inline]
    pub fn get_ref(&self) -> &T {
        &self.data
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.data
    }
}

pub trait Buildable: Sized {
    fn builder() -> Self;
}

pub struct TypeRegistry<K: std::hash::Hash + Eq + Copy, V: Clone> {
    entries: std::collections::HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq + Copy, V: Clone> TypeRegistry<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    #[inline]
    pub fn register(&mut self, key: K, value: V) {
        self.entries.insert(key, value);
    }

    #[inline]
    pub fn get(&self, key: K) -> Option<V> {
        self.entries.get(&key).cloned()
    }

    #[inline]
    pub fn contains(&self, key: K) -> bool {
        self.entries.contains_key(&key)
    }

    #[inline]
    pub fn list(&self) -> Vec<(K, V)> {
        self.entries.iter().map(|(&k, v)| (k, v.clone())).collect()
    }
}

impl<K: std::hash::Hash + Eq + Copy, V: Clone> Default for TypeRegistry<K, V> {
    fn default() -> Self {
        Self::new()
    }
}


