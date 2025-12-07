use std::{
    alloc,
    ops::{Deref, DerefMut},
    ptr::NonNull, sync::{Arc, Mutex, RwLock},
};

pub trait MemoryManager: Send + Sync {
    fn with_capacity(cap: usize) -> Self
    where
        Self: Sized;
    fn alloc_heep(&self, size: usize, shard_hint: usize) -> VPtr;
    fn realloc_heep(&self, ptr: VPtr, new_size: usize);
    fn dealloc_heep(&self, ptr: VPtr, shard_hint: usize);
    fn as_ptr(&self, ptr: VPtr) -> *mut u8;
    fn total_memory_size_hint(&self) -> usize {
        0
    }
    fn static_data(&self, data: &[u8]) -> VPtr;
}

/// デフォルトのメモリマネージャ
#[cfg(not(feature = "unsafe-opt"))]
pub type Memory = DefaultMemoryManager;
/// らっぷされてないやつ たぶん早いけどあぶない？
#[cfg(feature = "unsafe-opt")]
pub type Memory = NoWrapMemoryManager;

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

/// 仮想メモリ
/// alloc/dealloc/realloc以外の安全性は保証しない
#[derive(Debug)]
pub struct DefaultMemoryManager {
    // 追加のみで削除、編集は禁止
    pub data: RwLock<Vec<Heep>>,
    pub reuse_list: Arc<Mutex<Vec<usize>>>,
}

impl MemoryManager for DefaultMemoryManager {
    fn with_capacity(cap: usize) -> Self {
        DefaultMemoryManager {
            data: RwLock::new(Vec::with_capacity(cap)),
            reuse_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 新しいHeepを確保
    /// これだけ
    #[inline(always)]
    fn alloc_heep(&self, size: usize, _shard_hint: usize) -> VPtr {
        if let Some(id) = self.reuse_list.lock().unwrap().pop() {
            self.data.write().unwrap().get_mut(id).unwrap().realloc(size);
            return VPtr::from_heep_id(id);
        } else {
            let heep = Heep::new(size);
            let mut data = self.data.write().unwrap();
            data.push(heep);
            let id = data.len() - 1;
            return VPtr::from_heep_id(id);
        }
    }

    /// サイズ再確保
    #[inline(always)]
    fn realloc_heep(&self, ptr: VPtr, new_size: usize) {
        if let Some(heep) = self.data.write().unwrap().get_mut(ptr.heep_id() as usize) {
            heep.realloc(new_size);
        } else {
            std::process::exit(-9998);
        }
    }

    /// 解放
    #[inline(always)]
    fn dealloc_heep(&self, ptr: VPtr, _shard_hint: usize) {
        if let Some(heep) = self.data.write().unwrap().get_mut(ptr.heep_id() as usize) {
            heep.dealloc();
            self.reuse_list.lock().unwrap().push(ptr.heep_id() as usize);
        } else {
            std::process::exit(-9998);
        }
    }

    /// 実ポインタへ変換
    #[inline(always)]
    fn as_ptr(&self, ptr: VPtr) -> *mut u8 {
        if let Some(heep) = self.data.read().unwrap().get(ptr.heep_id() as usize) {
            let ptr = heep.ptr();
            ptr
        } else {
            std::process::exit(-9998);
        }
    }

    /// 全Heepの合計サイズを取得
    fn total_memory_size_hint(&self) -> usize {
        let mut total_size = 0;
        for heep in self.data.read().unwrap().iter() {
            total_size += heep.size;
        }
        total_size
    }

    fn static_data(&self, data: &[u8]) -> VPtr {
        let size = data.len();
        let vptr = self.alloc_heep(size, 0);
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
        let null_self = RawHeep {
            ptr: NonNull::dangling(),
            size: 0,
        };
        null_self.alloc(size);
        null_self
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

pub struct NoWrapMemoryManager;

impl MemoryManager for NoWrapMemoryManager {
    fn with_capacity(_cap: usize) -> Self
    where
        Self: Sized,
    {
        NoWrapMemoryManager
    }

    fn alloc_heep(&self, size: usize, _shard_hint: usize) -> VPtr {
        let heap = Heep::new(size);
        let ptr = heap.ptr();
        let raw_ptr = ptr as u64;
        VPtr(raw_ptr) 
    }

    fn realloc_heep(&self, ptr: VPtr, new_size: usize) {
        let raw_ptr = ptr.0 as *mut u8;
        let layout = alloc::Layout::from_size_align(new_size, RawHeep::ALIGN).unwrap();
        let uncheck_ptr = unsafe { alloc::realloc(raw_ptr, layout, new_size) };
        if uncheck_ptr.is_null() {
            oom();
        }
    }

    fn dealloc_heep(&self, ptr: VPtr, shard_hint: usize) {
        let raw_ptr = ptr.0 as *mut u8;
        let layout = alloc::Layout::from_size_align(shard_hint, RawHeep::ALIGN).unwrap();
        unsafe {
            alloc::dealloc(raw_ptr, layout);
        }
    }

    fn as_ptr(&self, ptr: VPtr) -> *mut u8 {
        ptr.0 as *mut u8
    }

    fn static_data(&self, data: &[u8]) -> VPtr {
        let size = data.len();
        let heap = Heep::new(size);
        let ptr = heap.ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, size);
        }
        let raw_ptr = ptr as u64;
        VPtr(raw_ptr)
    }
}

#[cold]
fn oom() {
    ::std::process::exit(-9999);
}