
extern crate protobuf;
extern crate rand;

mod protos;
mod daemon;

use protos::robot as robot_pb;

pub struct RobotKernel {
    daemon: daemon::DaemonClient,
    seq: u32,
}

impl RobotKernel {
    pub fn new(daemon: daemon::DaemonClient) -> RobotKernel {
        RobotKernel{
            daemon: daemon,
            seq: rand::random(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
