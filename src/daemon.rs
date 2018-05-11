
use rand;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use protobuf::Message;
use protos::daemon as daemon_pb;
use protos::commontypes as common_pb;
use protos::robot as robot_pb;
use super::robot;

//static DAEMON_CLIENT: Option<DaemonProxy> = None;

#[derive(Clone)]
pub struct DaemonProxy {
    inner: Arc<Mutex<Inner>>
}

unsafe impl Send for DaemonProxy {}
unsafe impl Sync for DaemonProxy {}

impl DaemonProxy {
    pub fn new() -> DaemonProxy {
        // Instantiate a ghostly daemon_pb::DaemonToClient() to avoid lazy initialization later.
        daemon_pb::DaemonToClient::new();
        {
            let mut req = robot_pb::RpcRequest::new();
            let bytes = req.write_to_bytes().unwrap();
            req.merge_from_bytes(&bytes.as_slice()).unwrap();
        }
        {
            let mut reply = robot_pb::RpcReply::new();
            let bytes = reply.write_to_bytes().unwrap();
            reply.merge_from_bytes(&bytes.as_slice()).unwrap();
        }
        DaemonProxy{inner: Arc::new( Mutex::new( Inner::new() ) )}
    }

    /// The "write callback" is the function the daemon proxy will use to try and send data back to
    /// the Daemon Server. 
    pub fn set_write_callback<F>(&mut self, cb: F) 
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        self.inner.lock().unwrap().set_write_callback(cb)
    }

    /// Deliver bytes received from the Daemon Server to this function.
    pub fn deliver(&mut self, buf: &Vec<u8>) -> Result<(), String> {
        self.inner.lock().unwrap().deliver(buf)
    }

    /// Get the version string from the Daemon Server
    pub fn get_version_string<F>(&mut self, cb: F) -> Result<(), String> 
        where F: FnMut(String),
              F: 'static
    {
        self.inner.lock().unwrap().get_version_string(cb)
    }

    /// Get a proxy to a robot. The daemon proxy will ask the server to create a robot reference
    /// for this robot, but the daemon server will not issue any "connect" commands to the robot.
    /// Use the connect_robot() function to do that.
    pub fn get_robot(&mut self, serial_id: &str) -> robot::Robot {
        self.inner.lock().unwrap().get_robot(serial_id, self.clone())
    }

    pub fn acquire_robot<F>(&self, cb: F) -> Result<(), String>
        where F: FnMut(Option<String>),
              F: 'static
    {
        self.inner.lock().unwrap().acquire_robot(cb)
    }

    /// Instruct the daemon server to send a ConnectSession message to a robot.
    pub fn connect_robot(&mut self, serial_id: &str) -> Result<(), String> {
        self.inner.lock().unwrap().connect_robot(serial_id)
    }

    /// Stop all robots connected to the daemon.
    pub fn stop_all_robots<F>(&mut self, cb: F) -> Result<(), String> 
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().stop_all_robots(cb)
    }

    /// Instruct the daemon server to transmit a message
    pub fn transmit<F>(&mut self, 
                       payload: daemon_pb::transmit_In,
                       cb: F) -> Result<(), String>
        where F: FnMut(daemon_pb::Status),
              F: 'static
    {
        self.inner.lock().unwrap().transmit(payload, cb)
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

    pub fn deliver(&mut self, buf: &Vec<u8>) -> Result<(), String> {
        // The message better be a DaemonToClient message
        let mut d_to_c = daemon_pb::DaemonToClient::new();
        if d_to_c.merge_from_bytes(buf.as_slice()).is_err() {
            Err(String::from("Could not parse DaemonToClient message."))
        } else if d_to_c.has_rpcReply() {
            self.handle_rpc_reply( d_to_c.take_rpcReply() )
        } else if d_to_c.has_receive() {
            self.handle_receive(d_to_c.take_receive())
        } else if d_to_c.has_dongleEvent() {
            self.handle_dongle_event(d_to_c.take_dongleEvent())
        } else if d_to_c.has_robotEvent() {
            self.handle_robot_event(d_to_c.take_robotEvent())
        } else {
            Err(String::from("Unknown DaemonToClient arg."))
        }
    }

    pub fn get_version_string<F>(&mut self, mut cb: F) -> Result<(), String> 
        where F: FnMut(String),
              F: 'static
    {
        let msg = daemon_pb::getDaemonVersionString_In::new();
        let mut request = daemon_pb::RpcRequest::new();
        request.set_getDaemonVersionString(msg);
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getDaemonVersionString() {
                warn!("Warning: Got version string reply with no data");
            } else {
                cb( reply.take_getDaemonVersionString().take_value() );
            }
        })
    }

    pub fn connect_robot(&mut self, serial_id: &str) -> Result<(), String>
    {
        self.add_robot_refs(vec![ String::from(serial_id), ], 
                            || { })
    }

    pub fn stop_all_robots<F>(&mut self, mut cb: F) -> Result<(), String> 
        where F: FnMut(),
              F: 'static
    {
        let mut reset_peripherals = robot_pb::ResetPeripherals::new();
        reset_peripherals.set_peripheralResetMask(0xffff);
        let mut payload = robot_pb::ClientToRobotBroadcast::new();
        payload.set_resetPeripherals(reset_peripherals);
        let mut broadcast = daemon_pb::transmitBroadcast_In::new();
        broadcast.set_broadcastMethod(daemon_pb::BroadcastMethod::MULTICAST_LOCAL);
        broadcast.set_payload(payload);
        let mut rpc_request = daemon_pb::RpcRequest::new();
        rpc_request.set_transmitBroadcast(broadcast);
        self.rpc_request(rpc_request, move |_| {
            cb();
        })
    }

    pub fn get_robot(&mut self, serial_id: &str, daemon: DaemonProxy) -> robot::Robot {
        // See if there is a robot in our map first
        if let Some(ref r) = self.robots.get(serial_id) {
            return (*r).clone();
        } 
        
        // Create a new robot object
        let r = robot::Robot::new_from_daemon(String::from(serial_id), &daemon);
        self.robots.insert(String::from(serial_id), r.clone());
        r
    }

    /// Get the serial ID of an un-acquired robot. If there are no robots or no acquireable
    /// robots, the callback is called with "None".
    pub fn acquire_robot<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Option<String>),
              F: 'static
    {
        debug!("Acquiring robot id...");
        let acquire = daemon_pb::acquireRobotRef_In::new();
        let mut rpc_request = daemon_pb::RpcRequest::new();
        rpc_request.set_acquireRobotRef(acquire);
        self.rpc_request(rpc_request, move |mut reply| {
            let mut acquire_out = reply.take_acquireRobotRef();
            if acquire_out.has_serialId() {
                debug!("Acquiring robot id... Got an ID");
                cb(Some(acquire_out.take_serialId().take_value()));
            } else {
                debug!("Acquiring robot id... Got None");
                cb(None);
            }
        })
    }

    pub fn add_robot_refs<F>(&mut self, serial_ids: Vec<String>, mut cb: F) -> Result<(), String>
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
        self.rpc_request(request, move |_| { cb(); })
    }

    fn rpc_request<F>(&mut self, 
                      mut request: daemon_pb::RpcRequest,
                      cb: F) -> Result<(), String> 
        where F: FnMut(daemon_pb::RpcReply),
              F: 'static
    {
        // Build a ClientToDaemon message
        request.set_requestId(self.seq);
        let mut msg = daemon_pb::ClientToDaemon::new();
        msg.set_rpcRequest(request);
        // Encode it and send it to the write callback
        if let Some(ref mut write_cb) = self.write_cb {
            if let Ok(data) = msg.write_to_bytes() {
                self.requests.insert(self.seq, Box::new(cb));
                self.seq += 1;
                write_cb(data);
                Ok(())
            } else {
                Err(String::from("daemon::rpc_request: Could not encode ClientToDaemon message"))
            }
        } else {
            Err(String::from("Daemon client write callback not set."))
        }
    }

    pub fn transmit<F>(&mut self, 
                       payload: daemon_pb::transmit_In,
                       mut cb: F) -> Result<(), String>
        where F: FnMut(daemon_pb::Status),
              F: 'static
    {
        let mut request = daemon_pb::RpcRequest::new();
        request.set_transmit(payload);
        self.rpc_request(request, move|mut reply| {
            if reply.has_transmit() {
                cb(reply.take_transmit().get_status());
            } else {
                warn!("Expected status in 'transmit()' reply.");
            }
        })
    }

    fn handle_rpc_reply(&mut self, reply: daemon_pb::RpcReply) -> Result<(),String> {
        // See if we have a matching request waiting
        let request_id = reply.get_requestId();
        if let Some(mut cb) = self.requests.remove(&request_id) {
            cb(reply);
            Ok(())
        } else {
            Err(format!("Received unsolicited RpcReply with ID {}.", request_id))
        }
    }

    fn handle_receive(&mut self, mut receive: daemon_pb::ReceiveTransmission) -> Result<(), String> {
        // Pass this message along to the correct robot
        // First, lets figure out which robot this message is addressed to.
        if ! receive.has_serialId() {
            Err(String::from("Received transmission with no destination serial id."))
        } else {
            if ! receive.has_payload() {
                Err(String::from(
                        "Received transmission with no payload." ))
            } else {
                let payload = receive.take_payload();
                let serial_id = receive.take_serialId().take_value();
                if let Some(ref mut robot) = self.robots.get_mut(&serial_id) {
                    robot.deliver(payload)
                } else {
                    Err(String::from(
                            format!("Received transmission for unknown robot: {}",
                                    serial_id) ))
                }
            }
        }
    }

    fn handle_dongle_event(&mut self, _: daemon_pb::DongleEvent) -> Result<(), String> {
        unimplemented!();
    }

    fn handle_robot_event(&mut self, mut event: daemon_pb::RobotEvent) -> Result<(), String> {
        if ! event.has_serialId() {
            warn!("Received robot event with no serial id. Ignoring...");
            Ok(())
        } else {
            // See if we have the robot in our list of robots
            let serial_id = event.take_serialId().take_value();
            if let Some(ref mut robot) = self.robots.get_mut(&serial_id) {
                robot.call_connect_handler();
            }
            Ok(())
        }
    }
}
