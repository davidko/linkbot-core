
use rand;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use protobuf::Message;
use protos::daemon as daemon_pb;
use protos::commontypes as common_pb;
use super::robot;

//static DAEMON_CLIENT: Option<DaemonClient> = None;

#[derive(Clone)]
pub struct DaemonClient {
    inner: Rc<RefCell<Inner>>
}

impl DaemonClient {
    pub fn new() -> DaemonClient {
        DaemonClient{inner: Rc::new( RefCell::new( Inner::new() ) )}
    }

    pub fn set_write_callback<F>(&mut self, cb: F) 
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        self.inner.borrow_mut().set_write_callback(cb)
    }

    pub fn deliver(&mut self, buf: Vec<u8>) {
        self.inner.borrow_mut().deliver(buf)
    }

    pub fn get_version_string<F>(&mut self, cb: F) -> Result<(), String> 
        where F: FnMut(String),
              F: 'static
    {
        self.inner.borrow_mut().get_version_string(cb)
    }

    pub fn get_robot(&mut self, serial_id: String) -> robot::Robot {
        self.inner.borrow_mut().get_robot(serial_id, self.clone())
    }
}

struct Inner {
    write_cb: Option<Box<FnMut(Vec<u8>)>>,
    seq: u32,
    requests: HashMap<u32, Box<FnMut(daemon_pb::RpcReply)>>,
    robots: HashMap<String, robot::Robot>,
}

impl Inner{
    pub fn new() -> Inner {
        Inner{ write_cb: None,
               seq: rand::random(),
               requests: HashMap::new(),
               robots: HashMap::new(),
        }
    }

    pub fn set_write_callback<F>(&mut self, cb: F) 
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        //! Set the function which writes to the daemon server.
        self.write_cb = Some(Box::new(cb));
    }

    pub fn deliver(&mut self, buf: Vec<u8>) {
    }

    pub fn get_version_string<F>(&mut self, mut cb: F) -> Result<(), String> 
        where F: FnMut(String),
              F: 'static
    {
        self.requests.insert(self.seq, Box::new( move |mut reply| {
            if ! reply.has_getDaemonVersionString() {
                println!("Warning: Got version string reply with no data");
            } else {
                cb( reply.take_getDaemonVersionString().take_value() );
            }
        }));
        let msg = daemon_pb::getDaemonVersionString_In::new();
        let mut request = daemon_pb::RpcRequest::new();
        request.set_getDaemonVersionString(msg);
        self.rpc_request(request)
    }

    pub fn get_robot(&mut self, serial_id: String, daemon: DaemonClient) -> robot::Robot {
        // See if there is a robot in our map first
        if let Some(ref r) = self.robots.get(&serial_id) {
            return (*r).clone();
        } 
    
        let r = robot::Robot::new_from_daemon(serial_id.clone(), &daemon);
        self.robots.insert(serial_id, r.clone());
        r
    }

    pub fn add_robot_refs<F>(&mut self, serial_ids: Vec<String>, cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut msg = daemon_pb::addRobotRefs_In::new();
        for id in serial_ids.into_iter() {
            let mut _id = common_pb::SerialId::new();
            _id.set_value(id);
            msg.mut_serialIds().push( _id );
        }
        let mut request = daemon_pb::RpcRequest::new();
        request.set_addRobotRefs(msg);
        self.rpc_request(request)
    }

    fn rpc_request(&mut self, request: daemon_pb::RpcRequest) -> Result<(), String> {
        // Build a ClientToDaemon message
        let mut msg = daemon_pb::ClientToDaemon::new();
        msg.set_rpcRequest(request);
        // Encode it and send it to the write callback
        if let Some(ref mut cb) = self.write_cb {
            if let Ok(data) = msg.write_to_bytes() {
                cb(data);
                Ok(())
            } else {
                Err(String::from("daemon::rpc_request: Could not encode ClientToDaemon message"))
            }
        } else {
            Err(String::from("Daemon client write callback not set."))
        }
    }
}
