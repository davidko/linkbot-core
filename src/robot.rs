
use rand;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use super::daemon;

use protos::commontypes as common_pb;
use protos::daemon as daemon_pb;
use protos::robot as robot_pb;

#[derive(Clone)]
pub struct Robot {
    inner: Rc<RefCell<Inner>>,
}

impl Robot {
    pub fn new_from_daemon(serial_id: String, d: &daemon::DaemonProxy) -> Robot {
        Robot{ inner: Rc::new( RefCell::new( 
                    Inner::new_from_daemon(serial_id, d)
                    )),
        }
    }

    pub fn deliver(&mut self, payload: robot_pb::RobotToClient) -> Result<(), String> {
        self.inner.borrow_mut().deliver(payload)
    }

    pub fn get_accelerometer_data<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32, f32, f32),
              F: 'static
    {
        self.inner.borrow_mut().get_accelerometer_data(cb)
    }

    pub fn get_battery_voltage<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        self.inner.borrow_mut().get_battery_voltage(cb)
    }

    pub fn get_button_state<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32),
              F: 'static
    {
        self.inner.borrow_mut().get_button_state(cb)
    }

    pub fn get_buzzer_frequency<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        self.inner.borrow_mut().get_buzzer_frequency(cb)
    }

    pub fn get_encoder_values<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<f32>),
              F: 'static
    {
        self.inner.borrow_mut().get_encoder_values(cb)
    }

    pub fn get_firmware_version_string<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(String),
              F: 'static
    {
        self.inner.borrow_mut().get_firmware_version_string(cb)
    }

    pub fn get_form_factor<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(robot_pb::FormFactor),
              F: 'static
    {
        self.inner.borrow_mut().get_form_factor(cb)
    }

    pub fn get_joint_states<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<robot_pb::JointState>),
              F: 'static
    {
        self.inner.borrow_mut().get_joint_states(cb)
    }

    pub fn get_led_color<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u8, u8, u8),
              F: 'static
    {
        self.inner.borrow_mut().get_led_color(cb)
    }

    pub fn set_led_color<F>(&mut self, 
                            red: u8, 
                            green: u8, 
                            blue: u8,
                            cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.borrow_mut().set_led_color(red,green,blue,cb)
    }
}

struct Inner
{
    serial_id: String,
    daemon: daemon::DaemonProxy,
    seq: u32,
    requests: HashMap<u32, Box<FnMut(robot_pb::RpcReply)>>,
}

impl Inner {
    fn new_from_daemon(serial_id: String, d: &daemon::DaemonProxy) -> Inner {
        Inner{ serial_id: serial_id,
               daemon: d.clone(),
               seq: rand::random(),
               requests: HashMap::new(),
        }
    }

    fn deliver(&mut self, mut payload: robot_pb::RobotToClient) -> Result<(), String> {
        if payload.has_rpcReply() {
            self.handle_rpc_reply(payload.take_rpcReply())
        } else {
            unimplemented!()
        }
    }

    fn handle_rpc_reply(&mut self, reply: robot_pb::RpcReply) -> Result<(), String> {
        // See if we have a matching request
        let request_id = reply.get_requestId();
        if let Some(mut cb) = self.requests.remove(&request_id) {
            cb(reply);
            Ok(())
        } else {
            Err(format!("Robot {} received unsolicited reply with request ID: {}.", self.serial_id, request_id))
        }
    }

    fn rpc_request<F>(&mut self,
                      mut request: robot_pb::RpcRequest,
                      cb: F) -> Result<(), String>
        where F: FnMut(robot_pb::RpcReply),
              F: 'static
    {
        request.set_requestId(self.seq);
        self.requests.insert(self.seq, Box::new(cb));
        self.seq += 1;

        let mut c_to_r = robot_pb::ClientToRobot::new();
        c_to_r.set_rpcRequest(request);

        let mut transmit = daemon_pb::transmit_In::new();
        let mut serial_id = common_pb::SerialId::new();
        serial_id.set_value( self.serial_id.clone() );
        transmit.set_destination(serial_id);
        transmit.set_payload(c_to_r);
        self.daemon.transmit(transmit, |status| {
            if status != daemon_pb::Status::OK {
                warn!("transmit received error status: {}", status);
            }
        })
    }

    fn get_accelerometer_data<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(f32, f32, f32),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getAccelerometerData( 
            robot_pb::getAccelerometerData_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getAccelerometerData() {
                warn!("Reply has no accelerometer data.");
                return;
            }
            let data = reply.take_getAccelerometerData();
            cb( data.get_x(),
                data.get_y(),
                data.get_z() );
        })
    }

    fn get_battery_voltage<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getBatteryVoltage( 
            robot_pb::getBatteryVoltage_In::new()
            );
        self.rpc_request(request, move |reply| {
            if ! reply.has_getBatteryVoltage() {
                warn!("Reply has no battery voltage data.");
                return;
            }
            cb(reply.get_getBatteryVoltage().get_v());
        })
    }

    fn get_button_state<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(u32),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getButtonState( 
            robot_pb::getButtonState_In::new()
            );
        self.rpc_request(request, move |reply| {
            if ! reply.has_getButtonState() {
                warn!("Reply has no button state data.");
                return;
            }
            cb(reply.get_getButtonState().get_mask());
        })
    }

    fn get_buzzer_frequency<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getBuzzerFrequency( 
            robot_pb::getBuzzerFrequency_In::new()
            );
        self.rpc_request(request, move |reply| {
            if ! reply.has_getBuzzerFrequency() {
                warn!("Reply has no buzzer frequency data.");
                return;
            }
            cb(reply.get_getBuzzerFrequency().get_value());
        })
    }

    fn get_encoder_values<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getEncoderValues( 
            robot_pb::getEncoderValues_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getEncoderValues() {
                warn!("Reply has no encoder data.");
                return;
            }
            let mut data = reply.take_getEncoderValues();
            cb(data.get_timestamp(), data.take_values());
        })
    }

    fn get_firmware_version_string<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(String),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getFirmwareVersionString( 
            robot_pb::getFirmwareVersionString_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getFirmwareVersionString() {
                warn!("Reply has no firmware version string data.");
                return;
            }
            let mut data = reply.take_getFirmwareVersionString();
            cb(data.take_value());
        })
    }

    fn get_form_factor<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(robot_pb::FormFactor),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getFormFactor( 
            robot_pb::getFormFactor_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getFormFactor() {
                warn!("Reply has no form factor data.");
                return;
            }
            let data = reply.take_getFormFactor();
            cb(data.get_value());
        })
    }

    fn get_joint_states<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<robot_pb::JointState>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getJointStates( 
            robot_pb::getJointStates_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getJointStates() {
                warn!("Reply has no form factor data.");
                return;
            }
            let mut data = reply.take_getJointStates();
            cb(data.get_timestamp(), data.take_values());
        })
    }

    fn get_led_color<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(u8, u8, u8),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getLedColor( 
            robot_pb::getLedColor_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getLedColor() {
                warn!("Reply has no form factor data.");
                return;
            }
            let data = reply.take_getLedColor();
            let value = data.get_value();
            cb(
                ((value >> 16)&0xff) as u8,
                ((value >> 8)&0xff) as u8,
                (value&0xff) as u8
                );
        })
    }

    fn set_led_color<F>(&mut self, 
                            red: u8, 
                            green: u8, 
                            blue: u8,
                            mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut led_message = robot_pb::setLedColor_In::new();
        let value:u32 = ((red as u32)<<16) | 
                        ((green as u32)<<8) |
                        blue as u32;
        led_message.set_value(value);

        let mut request = robot_pb::RpcRequest::new();
        request.set_setLedColor(led_message);
        
        self.rpc_request(request, move |_| { cb(); })
    }
}

impl fmt::Display for daemon_pb::Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            daemon_pb::Status::OK => write!(f, "Ok."),
            daemon_pb::Status::CANNOT_OPEN_DONGLE => write!(f, "Cannot open dongle."),
            daemon_pb::Status::DONGLE_NOT_FOUND => write!(f, "Dongle not found."),
            daemon_pb::Status::PORT_OUT_OF_RANGE => write!(f, "Port out of range."),
            daemon_pb::Status::UNREGISTERED_SERIALID => write!(f, "Unregistered robot serial ID."),
            daemon_pb::Status::INVALID_SERIALID => write!(f, "Invalid robot serial ID."),
            daemon_pb::Status::DAEMON_UNAVAILABLE => write!(f, "Daemon unavailable."),
            daemon_pb::Status::STRANGE_DONGLE=> write!(f, "Strange dongle."),
            daemon_pb::Status::INCOMPATIBLE_FIRMWARE => write!(f, "Incompatible firmware."),
            daemon_pb::Status::BUFFER_OVERFLOW => write!(f, "Buffer overflow."),
            daemon_pb::Status::OTHER_ERROR => write!(f, "Unknown error."),
        }
    }
}
