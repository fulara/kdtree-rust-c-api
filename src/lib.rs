#![feature(drop_types_in_const)]

extern crate libc;

extern crate kdtree;

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Point3WithId {
    pointer: usize,
    dims: [f64;3],
}

impl ::kdtree::kdtree::KdtreePointTrait for Point3WithId {
    fn dims(&self) -> &[f64] {
        &self.dims
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Point2WithId {
    pointer: usize,
    dims: [f64;2],
}

impl ::kdtree::kdtree::KdtreePointTrait for Point2WithId {
    fn dims(&self) -> &[f64] {
        &self.dims
    }
}

static mut KDTREE3: Option<kdtree::kdtree::Kdtree<Point3WithId>> = None;
static mut KDTREE2: Option<kdtree::kdtree::Kdtree<Point2WithId>> = None;

#[no_mangle]
pub extern "C" fn kdtree3_create(array_pointer: *mut Point3WithId, size: libc::size_t) {
    unsafe {
        KDTREE3 = Some(::kdtree::kdtree::Kdtree::new(std::slice::from_raw_parts_mut(array_pointer, size as usize)));
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_nearest_search(searched_for : *mut Point3WithId) -> Point3WithId {
    unsafe {
        KDTREE3.as_ref().unwrap().nearest_search(&(*searched_for))
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_has_neighbor_in_range(searched_for : *mut Point3WithId, range :f64 ) -> bool {
    unsafe {
        KDTREE3.as_ref().unwrap().has_neighbor_in_range(&(*searched_for), range)
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_distance_squared_to_nearest(searched_for : *mut Point3WithId ) -> f64 {
    unsafe {
        KDTREE3.as_ref().unwrap().distance_squared_to_nearest(&(*searched_for))
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_insert_node(node : *mut Point3WithId) {
    unsafe {
        KDTREE3.as_mut().unwrap().insert_node(*node);
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_create(array_pointer: *mut Point2WithId, size: libc::size_t) {
    unsafe {
        KDTREE2 = Some(::kdtree::kdtree::Kdtree::new(std::slice::from_raw_parts_mut(array_pointer, size as usize)));
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_nearest_search(searched_for : *mut Point2WithId) -> Point2WithId {
    unsafe {
        KDTREE2.as_ref().unwrap().nearest_search(&(*searched_for))
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_has_neighbor_in_range(searched_for : *mut Point2WithId, range :f64 ) -> bool {
    unsafe {
        KDTREE2.as_ref().unwrap().has_neighbor_in_range(&(*searched_for), range)
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_distance_squared_to_nearest(searched_for : *mut Point2WithId ) -> f64 {
    unsafe {
        KDTREE2.as_ref().unwrap().distance_squared_to_nearest(&(*searched_for))
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_insert_node(node : *mut Point2WithId) {
    unsafe {
        KDTREE2.as_mut().unwrap().insert_node(*node);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
