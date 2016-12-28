#![feature(drop_types_in_const)]

extern crate libc;

extern crate kdtree_rust;

#[repr(C)]
pub struct Point3WithId {
    num: i32,
    dims: [f64;3],
}

impl ::kdtree_rust::kdtree::KdtreePointTrait for Point3WithId {
    fn dims(&self) -> &[f64] {
        &self.dims
    }
}

static mut KDTREE: Option<kdtree_rust::kdtree::Kdtree<Point3WithId>> = None;

#[no_mangle]
pub extern "C" fn create_tree() -> i32 {
    let v : Vec<Point3WithId> = vec![];
    unsafe {
        KDTREE = Some(::kdtree_rust::kdtree::Kdtree::new(v));
    }
    3
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
