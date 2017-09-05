
#[macro_use] extern crate log;
extern crate protobuf;
extern crate rand;

mod protos;
mod daemon;
mod robot;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;

pub use daemon::{DaemonProxy};
pub use robot::{Robot};

#[no_mangle]
pub extern fn daemon_new() -> *mut DaemonProxy {
    //! Create and return a handle to a daemon proxy
    let d = DaemonProxy::new();
    Box::into_raw( Box::new(d) )
}

#[no_mangle]
pub extern fn daemon_set_write_callback(daemon: *mut DaemonProxy, 
                                        cb: extern fn(Vec<u8>)) {
    //! Set the function the daemon will use to send messages to the Daemon-Server.
    let mut d = unsafe {
        Box::from_raw(daemon)
    };

    d.set_write_callback( move |buf| {
        cb(buf);
    });

    Box::into_raw(d);
}

#[no_mangle]
pub extern fn daemon_deliver(daemon: *mut DaemonProxy, 
                             buffer: *mut u8, 
                             len: usize) {
    //! Pass messages coming from the Daemon-Server to this function
    let mut d = unsafe {
        Box::from_raw(daemon)
    };

    let vec = unsafe {
        Vec::from_raw_parts(buffer, len, len)
    };

    if let Err(msg) = d.deliver(&vec) {
        println!("Error delivering bytes to daemon proxy: {}", msg);
    }

    mem::forget(vec);
    Box::into_raw(d);
}

#[no_mangle]
pub extern fn daemon_get_robot(daemon: *mut DaemonProxy, 
                               serial_id: *mut c_char) -> *mut Robot {
    //! Get a handle to a robot object from the daemon proxy
    let mut d = unsafe {
        Box::from_raw(daemon)
    };

    let robot = unsafe {
        let cstring = CString::from_raw(serial_id);
        println!("Getting robot with id: '{}'", cstring.to_str().unwrap());
        let robot = d.get_robot(cstring.to_str().unwrap());
        println!("Getting robot with id: '{}' done", cstring.to_str().unwrap());
        mem::forget(cstring);
        robot
    };

    Box::into_raw(d);
    println!("Returning robot pointer...");
    Box::into_raw( Box::new(robot) )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
