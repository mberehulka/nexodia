use std::ffi::CStr;

use ash::Entry;

pub fn new() -> Entry {
    let entry = Entry::linked();
    for required in super::REQUIRED_LAYERS {
        let found = entry.enumerate_instance_layer_properties().unwrap()
            .iter().any(|layer| {
                let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
                let name = name.to_str().expect("Failed to get layer name pointer");
                *required == *name
            });
        if !found {
            panic!("Validation layer not supported: {}", required)
        }
    }
    entry
}