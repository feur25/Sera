use std::mem::MaybeUninit;

pub struct AdaptiveBuf<T: Copy, const N: usize> {
    inline: [MaybeUninit<T>; N],
    len: usize,
    overflow: Option<Vec<T>>,
}

impl<T: Copy, const N: usize> AdaptiveBuf<T, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inline: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
            len: 0,
            overflow: None,
        }
    }

    #[inline]
    pub fn push(&mut self, val: T) {
        match self.overflow {
            Some(ref mut v) => v.push(val),
            None => {
                if self.len < N {
                    unsafe {
                        self.inline[self.len].as_mut_ptr().write(val);
                    }
                    self.len += 1;
                } else {
                    let mut v: Vec<T> = Vec::with_capacity(N * 2);
                    for i in 0..N {
                        v.push(unsafe { self.inline[i].assume_init() });
                    }
                    v.push(val);
                    self.overflow = Some(v);
                }
            }
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, slice: &[T]) {
        for &v in slice {
            self.push(v);
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        match self.overflow {
            Some(ref v) => v.as_slice(),
            None => unsafe {
                std::slice::from_raw_parts(self.inline.as_ptr() as *const T, self.len)
            },
        }
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx < self.len() {
            Some(self.as_slice()[idx])
        } else {
            None
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self.overflow {
            Some(ref v) => v.len(),
            None => self.len,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut v) = self.overflow {
            v.clear();
        }
        self.len = 0;
        if self
            .overflow
            .as_ref()
            .map(|v| v.is_empty())
            .unwrap_or(false)
        {
            self.overflow = None;
        }
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Copy, const N: usize> Default for AdaptiveBuf<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
