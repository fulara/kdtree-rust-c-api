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

#[no_mangle]
pub extern "C" fn kdtree3_create(array_pointer: *mut Point3WithId, size: libc::size_t) -> *mut libc::c_void {
    unsafe {
        let mut tree = Box::new(::kdtree::kdtree::Kdtree::new(std::slice::from_raw_parts_mut(array_pointer, size as usize)));
        let ptr: *mut ::kdtree::kdtree::Kdtree<Point3WithId> = &mut *tree;
        ::std::mem::forget(tree);

        ptr as *mut ::libc::c_void
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_free(ptr: *mut libc::c_void) {
    let obj: Box<::kdtree::kdtree::Kdtree<Point3WithId>> = unsafe { ::std::mem::transmute(ptr) };
}

#[no_mangle]
pub extern "C" fn kdtree3_nearest_search(ptr: *mut libc::c_void, searched_for : *mut Point3WithId) -> Point3WithId {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point3WithId>;
        (*tree).nearest_search(&(*searched_for))
    }
}


#[no_mangle]
pub extern "C" fn kdtree3_has_neighbor_in_range(ptr: *mut libc::c_void,searched_for : *mut Point3WithId, range :f64 ) -> bool {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point3WithId>;
        (*tree).has_neighbor_in_range(&(*searched_for), range)
    }
}


#[no_mangle]
pub extern "C" fn kdtree3_distance_squared_to_nearest(ptr: *mut libc::c_void,searched_for : *mut Point3WithId ) -> f64 {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point3WithId>;
        (*tree).distance_squared_to_nearest(&(*searched_for))
    }
}


#[no_mangle]
pub extern "C" fn kdtree3_insert_node(ptr: *mut libc::c_void,node : *mut Point3WithId) {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point3WithId>;
        (*tree).insert_node(*node);
    }
}

#[no_mangle]
pub extern "C" fn kdtree3_insert_nodes(ptr: *mut libc::c_void,array_pointer: *mut Point3WithId, size: libc::size_t) {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point3WithId>;
        (*tree).insert_nodes_and_rebuild(std::slice::from_raw_parts_mut(array_pointer, size as usize));
    }
}


#[no_mangle]
pub extern "C" fn kdtree2_create(array_pointer: *mut Point2WithId, size: libc::size_t) -> *mut libc::c_void {
    unsafe {
        let mut tree = Box::new(::kdtree::kdtree::Kdtree::new(std::slice::from_raw_parts_mut(array_pointer, size as usize)));
        let ptr: *mut ::kdtree::kdtree::Kdtree<Point2WithId> = &mut *tree;
        ::std::mem::forget(tree);

        ptr as *mut ::libc::c_void
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_free(ptr: *mut libc::c_void) {
    let obj: Box<::kdtree::kdtree::Kdtree<Point2WithId>> = unsafe { ::std::mem::transmute(ptr) };
}

#[no_mangle]
pub extern "C" fn kdtree2_nearest_search(ptr: *mut libc::c_void, searched_for : *mut Point2WithId) -> Point2WithId {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point2WithId>;
        (*tree).nearest_search(&(*searched_for))
    }
}


#[no_mangle]
pub extern "C" fn kdtree2_has_neighbor_in_range(ptr: *mut libc::c_void,searched_for : *mut Point2WithId, range :f64 ) -> bool {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point2WithId>;
        (*tree).has_neighbor_in_range(&(*searched_for), range)
    }
}


#[no_mangle]
pub extern "C" fn kdtree2_distance_squared_to_nearest(ptr: *mut libc::c_void,searched_for : *mut Point2WithId ) -> f64 {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point2WithId>;
        (*tree).distance_squared_to_nearest(&(*searched_for))
    }
}


#[no_mangle]
pub extern "C" fn kdtree2_insert_node(ptr: *mut libc::c_void,node : *mut Point2WithId) {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point2WithId>;
        (*tree).insert_node(*node);
    }
}

#[no_mangle]
pub extern "C" fn kdtree2_insert_nodes(ptr: *mut libc::c_void,array_pointer: *mut Point2WithId, size: libc::size_t) {
    unsafe {
        let tree = ptr as *mut ::kdtree::kdtree::Kdtree<Point2WithId>;
        (*tree).insert_nodes_and_rebuild(std::slice::from_raw_parts_mut(array_pointer, size as usize));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
