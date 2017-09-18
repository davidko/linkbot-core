
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
    SignalState,
};
pub use protos::robot::{ Button, 
                         ButtonState, 
                         FormFactor,
                         Goal, 
                         Goal_Type, 
                         Goal_Controller, 
                         JointState};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
