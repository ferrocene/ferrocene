//@ run-pass
//@ edition:2021

const LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED: i32 = 0x01;
const LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT: i32 = 0x02;

pub fn hotplug_callback(event: i32) {
    let _ = || {
        match event {
            LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED => (),
            LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT => (),
            _ => (),
        };
    };
}

fn main() {
    hotplug_callback(1);
}

// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
