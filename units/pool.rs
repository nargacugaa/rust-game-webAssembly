use std::mem;
use std::num::NonZeroU32;

/// 世代句柄, 包含索引和世代数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle {
    index: u32,
    generation: NonZeroU32,
}

impl Handle {
    pub const NONE: Self = Self {
        index: u32::MAX,
        generation: unsafe { NonZeroU32::new_unchecked(1) },
    };

    pub fn is_some(&self) -> bool {
        *self != Self::NONE
    }

    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn generation(&self) -> NonZeroU32 {
        self.generation
    }
}

struct Entry<T> {
    generation: NonZeroU32,
    value: Option<T>,
}

pub struct Pool<T> {
    entries: Vec<Entry<T>>,
    free_indices: Vec<u32>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    /// 向池中添加一个值，返回其句柄
    pub fn spawn(&mut self, value: T) -> Handle {
        if let Some(index) = self.free_indices.pop() {
            let entry = &mut self.entries[index as usize];
            let generation = entry.generation;
            entry.value = Some(value);

            Handle { index, generation }
        } else {
            let index = self.entries.len() as u32;

            // 从1开始，因为0不是有效的NonZeroU32
            let generation = NonZeroU32::new(1).unwrap();

            self.entries.push(Entry {
                generation,
                value: Some(value),
            });

            Handle { index, generation }
        }
    }

    /// 销毁池中的值，使其句柄失效
    pub fn despawn(&mut self, handle: Handle) -> Option<T> {
        if handle.is_none() {
            return None;
        }

        let index = handle.index as usize;
        if index >= self.entries.len() {
            return None;
        }

        let entry = &mut self.entries[index];
        if entry.generation != handle.generation {
            return None;
        }

        let result = entry.value.take();
        if result.is_some() {
            entry.generation =
                NonZeroU32::new(entry.generation.get().wrapping_add(1).max(1)).unwrap();

            // 将索引添加到空闲列表
            self.free_indices.push(handle.index);
        }

        result
    }

    /// 借用池中的值
    pub fn borrow(&self, handle: Handle) -> Option<&T> {
        if handle.is_none() {
            return None;
        }

        let index = handle.index as usize;
        if index >= self.entries.len() {
            return None;
        }

        let entry = &self.entries[index];
        if entry.generation != handle.generation {
            return None;
        }

        entry.value.as_ref()
    }

    /// 可变的借用池中的值
    pub fn borrow_mut(&mut self, handle: Handle) -> Option<&mut T> {
        if handle.is_none() {
            return None;
        }

        let index = handle.index as usize;
        if index >= self.entries.len() {
            return None;
        }

        let entry = &mut self.entries[index];
        if entry.generation != handle.generation {
            return None;
        }

        entry.value.as_mut()
    }

    /// 迭代池中的所有值
    pub fn iter(&self) -> impl Iterator<Item = (Handle, &T)> {
        self.entries.iter().enumerate().filter_map(|(i, entry)| {
            entry.value.as_ref().map(|value| {
                (
                    Handle {
                        index: i as u32,
                        generation: entry.generation,
                    },
                    value,
                )
            })
        })
    }

    /// 可变迭代池中的所有值
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Handle, &mut T)> {
        self.entries.iter_mut().enumerate().filter_map(|(i, entry)| {
            entry.value.as_mut().map(|value| {
                (
                    Handle {
                        index: i as u32,
                        generation: entry.generation,
                    },
                    value,
                )
            })
        })
    }
}
