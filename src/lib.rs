
#[macro_use] extern crate log;
extern crate protobuf;
extern crate rand;

mod protos;
mod daemon;
mod robot;

pub use daemon::{DaemonProxy};
pub use robot::{Robot, 
    AccelerometerEventHandler, 
    ButtonEventHandler, 
    EncoderEventHandler,
    ConnectEventHandler,
};
pub use protos::robot::{Button, ButtonState};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
