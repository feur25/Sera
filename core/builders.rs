use std::sync::Mutex;

pub trait Builder<T> {
    fn build(self) -> T;
}

pub struct MutexState<T>(pub Mutex<T>);

impl<T: Copy + Default> MutexState<T> {
    pub fn new(default: T) -> Self {
        Self(Mutex::new(default))
    }

    pub fn get(&self, default: T) -> T {
        self.0.lock().map(|g| *g).unwrap_or(default)
    }

    pub fn set(&self, value: T) {
        if let Ok(mut guard) = self.0.lock() {
            *guard = value;
        }
    }

    pub fn compute<F: FnOnce(T) -> T>(&self, f: F, default: T) {
        if let Ok(mut guard) = self.0.lock() {
            *guard = f(*guard);
        }
    }
}

pub fn merge_option<T>(opt: Option<T>, default: T) -> T {
    opt.unwrap_or(default)
}

pub fn try_map<T, U, F>(opt: Option<T>, f: F) -> Option<U>
where
    F: FnOnce(T) -> U,
{
    opt.map(f)
}

pub trait TupleBuilder<T> {
    fn into_parts(self) -> T;
}

#[macro_export]
macro_rules! fluent_builder {
    ($name:ident, $build:ty, $($field:ident: $typ:ty),+) => {
        pub struct $name {
            $($field: $typ),+
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($field: Default::default()),+
                }
            }

            $(
                pub fn $field(mut self, value: $typ) -> Self {
                    self.$field = value;
                    self
                }
            )+
        }

        impl $crate::core::builders::Builder<$build> for $name {
            fn build(self) -> $build {
                $build {
                    $($field: self.$field),+
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

pub fn safe_lock<T: Copy + Default>(mutex: &Mutex<T>, default: T) -> T {
    mutex.lock().map(|g| *g).unwrap_or(default)
}

pub fn compare_default<T: Copy + PartialEq + Default>(value: T) -> bool {
    value == T::default()
}

pub struct ChainableResult<T> {
    value: T,
}

impl<T> ChainableResult<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> ChainableResult<U> {
        ChainableResult { value: f(self.value) }
    }

    pub fn inspect<F: FnOnce(&T)>(self, f: F) -> ChainableResult<T> {
        f(&self.value);
        self
    }

    pub fn unwrap(self) -> T {
        self.value
    }
}
