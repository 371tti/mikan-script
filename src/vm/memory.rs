use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};


/// キャッシュサイズ
/// !!! 2^n しか許可しません
pub const HEEP_PTR_CACHE_SIZE: usize = 16;

pub struct Memory {
    pub data: Vec<Heep>,
    pub reuse_list: Vec<u64>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: Vec::new(),
            reuse_list: Vec::new(),
        }
    }

    /// 新しいHeepとそのid
    #[inline(always)]
    pub fn alloc_heep(&mut self, size: usize) -> u64 {
        if let Some(id) = self.reuse_list.pop() {
            let heep = &mut self.data[id as usize];
            heep.alloc(size);
            return id;
        } else {
            let id = self.data.len() as u64;
            let heep = Heep::new(size);
            self.data.push(heep);
            return id;
        }
    }

    #[inline(always)]
    pub fn realloc_heep(&mut self, id: u64, new_size: usize) {
        if let Some(heep) = self.data.get_mut(id as usize) {
            heep.realloc(new_size);
        } else {
            std::process::exit(-9998);
        }
    }

    #[inline(always)]
    pub fn dealloc_heep(&mut self, id: u64) {
        if let Some(heep) = self.data.get_mut(id as usize) {
            heep.dealloc();
            self.reuse_list.push(id);
        } else {
            std::process::exit(-9998);
        }
    }

    #[inline(always)]
    pub fn heep(&self, id: u64) -> &Heep {
        if let Some(heep) = self.data.get(id as usize) {
            heep
        } else {
            std::process::exit(-9998);
        }
    }

    #[inline(always)]
    pub fn head_ptr(&mut self, id: u64) -> usize {
        if let Some(heep) = self.data.get(id as usize) {
            let ptr = heep.ptr();
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
    fn alloc(&mut self, size: usize) {
        let layout = alloc::Layout::from_size_align(size, Self::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::alloc(layout) };
        if uncheck_ptr.is_null() {
            oom();
        }
        self.ptr = unsafe { NonNull::new_unchecked(uncheck_ptr) };
        self.size = size;
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
