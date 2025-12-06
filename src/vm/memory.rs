use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// 仮想ポインタ
/// 上位24bit: heep id
/// 下位40bit: heep内オフセット
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VPtr(pub u64);

impl VPtr {
    #[inline(always)]
    pub fn from_heep_id(id: usize) -> Self {
        VPtr((id as u64) << 40)
    }

    #[inline(always)]
    pub fn heep_id(&self) -> usize {
        (self.0 >> 40) as usize
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        (self.0 & 0x0000_00FF_FFFF_FFFF) as usize
    }
}

impl From<u64> for VPtr {
    #[inline(always)]
    fn from(v: u64) -> Self {
        VPtr(v)
    }
}

impl Deref for VPtr {
    type Target = u64;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 仮想メモリ
#[derive(Clone, Debug)]
pub struct Memory {
    pub data: Vec<Heep>,
    pub reuse_list: Vec<usize>,
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
    pub fn alloc_heep(&mut self, size: usize) -> VPtr {
        if let Some(id) = self.reuse_list.pop() {
            let heep = &self.data[id as usize];
            heep.alloc(size);
            return VPtr::from_heep_id(id);
        } else {
            let id = self.data.len();
            let heep = Heep::new(size);
            self.data.push(heep);
            return VPtr::from_heep_id(id);
        }
    }

    /// サイズ再確保
    #[inline(always)]
    pub fn realloc_heep(&mut self, ptr: VPtr, new_size: usize) {
        if let Some(heep) = self.data.get(ptr.heep_id() as usize) {
            heep.realloc(new_size);
        } else {
            std::process::exit(-9998);
        }
    }

    /// 解放
    #[inline(always)]
    pub fn dealloc_heep(&mut self, ptr: VPtr) {
        if let Some(heep) = self.data.get(ptr.heep_id() as usize) {
            heep.dealloc();
            self.reuse_list.push(ptr.heep_id() as usize);
        } else {
            std::process::exit(-9998);
        }
    }

    /// 実ポインタへ変換
    #[inline(always)]
    pub fn as_ptr(&self, ptr: VPtr) -> *mut u8 {
        if let Some(heep) = self.data.get(ptr.heep_id() as usize) {
            let ptr = heep.ptr();
            ptr
        } else {
            std::process::exit(-9998);
        }
    }

    /// 全Heepの合計サイズを取得
    pub fn total_memory_size(&self) -> usize {
        let mut total_size = 0;
        for heep in &self.data {
            total_size += heep.size;
        }
        total_size
    }

    pub fn static_data(&mut self, data: &[u8]) -> VPtr {
        let size = data.len();
        let vptr = self.alloc_heep(size);
        let heep_ptr = self.as_ptr(vptr);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), heep_ptr, size);
        }
        vptr
    }
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
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
    fn alloc(&self, size: usize) {
        let layout = alloc::Layout::from_size_align(size, Self::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::alloc(layout) };
        if uncheck_ptr.is_null() {
            oom();
        }
        unsafe {
            let this = self as *const Self as *mut Self;
            (*this).ptr = NonNull::new_unchecked(uncheck_ptr);
            (*this).size = size;
        }
    }

    #[inline(always)]
    fn realloc(&self, new_size: usize) {
        let layout = alloc::Layout::from_size_align(self.size, Self::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::realloc(self.ptr(), layout, new_size) };
        if uncheck_ptr.is_null() {
            oom();
        }
        unsafe {
            let this = self as *const Self as *mut Self;
            (*this).ptr = NonNull::new_unchecked(uncheck_ptr);
            (*this).size = new_size;
        }
    }

    #[inline(always)]
    fn dealloc(&self) {
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
