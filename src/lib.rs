#![feature(drop_types_in_const)]

extern crate libc;

extern crate kdtree;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point3WithId {
    num: i32,
    dims: [f64;3],
}

impl ::kdtree::kdtree::KdtreePointTrait for Point3WithId {
    fn dims(&self) -> &[f64] {
        &self.dims
    }
}

static mut KDTREE: Option<kdtree::kdtree::Kdtree<Point3WithId>> = None;

#[no_mangle]
pub extern "C" fn kdtree_create(array_pointer: *mut Point3WithId, size: libc::size_t) {
    unsafe {
        KDTREE = Some(::kdtree::kdtree::Kdtree::new(std::slice::from_raw_parts_mut(array_pointer, size as usize)));
    }
}

#[no_mangle]
pub extern "C" fn kdtree_nearest_search(searched_for : *mut Point3WithId) -> Point3WithId {
    unsafe {
        KDTREE.as_ref().unwrap().nearest_search(&(*searched_for))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
