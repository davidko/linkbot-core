
use rand;
use super::daemon;

pub struct Robot
{
    serial_id: String,
    daemon: daemon::DaemonClient,
    seq: u32,
}

impl Robot {
    pub new_from_daemon(serial_id: String, d: daemon::DaemonClient) -> Robot {
        Robot{ serial_id: serial_id,
               daemon: d.clone(),
               seq: rand.random(),
        }
    }
}
