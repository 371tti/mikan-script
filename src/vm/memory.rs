use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use rustc_hash::FxHashMap;

/// キャッシュサイズ
/// !!! 2^n しか許可しません
pub const HEEP_PTR_CACHE_SIZE: usize = 16;

pub struct Memory {
    pub data: FxHashMap<u64, Heep>,
    pub counter: u64,
    cache_ids: [u64; HEEP_PTR_CACHE_SIZE],
    cache_ptrs: [usize; HEEP_PTR_CACHE_SIZE],
    cache_rotate_index: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: FxHashMap::default(),
            counter: 0,
            // 無効なエントリとして u64::MAX を使う
            // 単カウントでMAXに到達することがないと考えられるため安全
            cache_ids: [u64::MAX; HEEP_PTR_CACHE_SIZE],
            cache_ptrs: [0usize; HEEP_PTR_CACHE_SIZE],
            // 下位4bitだけきりおとしてindexにする
            cache_rotate_index: 0,
        }
    }

    #[inline(always)]
    fn cache_push_front(&mut self, id: u64, ptr: usize) {
        self.cache_rotate_index = self.cache_rotate_index.wrapping_sub(1);
        let cache_index = self.cache_rotate_index as usize & (HEEP_PTR_CACHE_SIZE - 1);
        self.cache_ids[cache_index] = id;
        self.cache_ptrs[cache_index] = ptr;
    }

    #[inline(always)]
    fn cache_search(&self, id: u64) -> Option<usize> {
        let front_index = self.cache_rotate_index;
        for i in front_index..front_index.wrapping_add(HEEP_PTR_CACHE_SIZE) {
            let i = i & (HEEP_PTR_CACHE_SIZE - 1);
            if self.cache_ids[i] == id {
                return Some(self.cache_ptrs[i]);
            }
        }
        None
    }

    #[inline(always)]
    fn cache_entry_front(&mut self, id: u64, ptr: usize) {
        if let Some(cache_index) = self.cache_search(id) {
            self.cache_ptrs[cache_index] = ptr;
        } else {
            self.cache_push_front(id, ptr);
        }
    }

    fn chache_remove(&mut self, id: u64) {
        let front_index = self.cache_rotate_index;
        for i in front_index..front_index.wrapping_add(HEEP_PTR_CACHE_SIZE) {
            let i = i & (HEEP_PTR_CACHE_SIZE - 1);
            if self.cache_ids[i] == id {
                self.cache_ids[i] = u64::MAX;
                return;
            }
        }
    }

    /// 新しいHeepとそのid
    #[inline(always)]
    pub fn alloc_heep(&mut self, size: usize) -> u64 {
        let heep = Heep::new(size);
        let id = self.counter;
        self.cache_push_front(id, heep.ptr());
        self.data.insert(id, heep);
        self.counter += 1;
        id
    }

    #[inline(always)]
    pub fn realloc_heep(&mut self, id: u64, new_size: usize) {
        if let Some(heep) = self.data.get_mut(&id) {
            heep.realloc(new_size);
            let ptr = heep.ptr();
            self.cache_entry_front(id, ptr);
        } else {
            std::process::exit(-9998);
        }
    }

    #[inline(always)]
    pub fn dealloc_heep(&mut self, id: u64) {
        self.data.remove(&id);
        self.chache_remove(id);
    }

    #[inline(always)]
    pub fn heep(&self, id: u64) -> &Heep {
        if let Some(heep) = self.data.get(&id) {
            heep
        } else {
            std::process::exit(-9998);
        }
    }

    #[inline(always)]
    pub fn head_ptr(&mut self, id: u64) -> usize {
        if let Some(ptr) = self.cache_search(id) {
            return ptr;
        }
        if let Some(heep) = self.data.get(&id) {
            let ptr = heep.ptr();
            self.cache_push_front(id, ptr);
            ptr
        } else {
            std::process::exit(-9998);
        }
    }
}

pub struct Heep {
    pub raw: RawHeep,
}

impl Heep {
    #[inline(always)]
    pub fn new(size: usize) -> Self {
        Heep {
            raw: RawHeep::new(size),
        }
    }

    #[inline(always)]
    pub fn ptr(&self) -> usize {
        self.raw.ptr() as usize
    }
}

unsafe impl Send for Heep {}
unsafe impl Sync for Heep {}

impl Deref for Heep {
    type Target = RawHeep;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for Heep {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

pub struct RawHeep {
    pub ptr: NonNull<u8>,
    pub size: usize,
}

impl RawHeep {
    const ALIGN: usize = 64;

    #[inline(always)]
    fn new(size: usize) -> Self {
        let layout = alloc::Layout::from_size_align(size, Self::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::alloc(layout) };
        if uncheck_ptr.is_null() {
            oom();
        }
        let ptr = unsafe { NonNull::new_unchecked(uncheck_ptr) };

        RawHeep { ptr, size }
    }

    #[inline(always)]
    fn ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    fn realloc(&mut self, new_size: usize) {
        let layout = alloc::Layout::from_size_align(self.size, Self::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::realloc(self.ptr(), layout, new_size) };
        if uncheck_ptr.is_null() {
            oom();
        }
        self.ptr = unsafe { NonNull::new_unchecked(uncheck_ptr) };
        self.size = new_size;
    }

    #[inline(always)]
    fn dealloc(&mut self) {
        let layout = alloc::Layout::from_size_align(self.size, Self::ALIGN).unwrap();
        unsafe {
            alloc::dealloc(self.ptr(), layout);
        }
    }

    #[inline(always)]
    fn deep_copy(&self) -> Self {
        let new_struct = RawHeep::new(self.size);
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr(), new_struct.ptr(), self.size);
        }
        new_struct
    }
}

impl Clone for RawHeep {
    #[inline(always)]
    fn clone(&self) -> Self {
        self.deep_copy()
    }
}

impl Drop for RawHeep {
    #[inline(always)]
    fn drop(&mut self) {
        self.dealloc();
    }
}

#[cold]
fn oom() {
    ::std::process::exit(-9999);
}
