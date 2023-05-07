pub fn allocate<T>(data: T) -> *mut T {
    // Allocate the memory using box
    let _box = Box::new(data);
    // Convert the box into a raw pointer
    Box::into_raw(_box)
}

pub(crate) unsafe fn alloc<T>(capacity: usize) -> *mut T {
    let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
    let ptr = std::alloc::alloc(layout) as *mut T;
    ptr
}

pub(crate) unsafe fn dealloc<T>(ptr: *mut T, capacity: usize) {
    let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
    std::alloc::dealloc(ptr as *mut u8, layout);
}