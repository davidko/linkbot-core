
use rand;
use std::cell::RefCell;
use std::rc::Rc;
use super::daemon;

#[derive(Clone)]
pub struct Robot {
    inner: Rc<RefCell<Inner>>,
}

impl Robot {
    pub fn new_from_daemon(serial_id: String, d: &daemon::DaemonClient) -> Robot {
        Robot{ inner: Rc::new( RefCell::new( 
                    Inner::new_from_daemon(serial_id, d)
                    )),
        }
    }
}

struct Inner
{
    serial_id: String,
    daemon: daemon::DaemonClient,
    seq: u32,
}

impl Inner {
    fn new_from_daemon(serial_id: String, d: &daemon::DaemonClient) -> Inner {
        Inner{ serial_id: serial_id,
               daemon: d.clone(),
               seq: rand::random(),
        }
    }
}
