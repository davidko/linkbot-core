
use rand;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use super::daemon;

use protos::commontypes as common_pb;
use protos::daemon as daemon_pb;
use protos::robot as robot_pb;

pub use self::robot_pb::{Goal, Goal_Type, Goal_Controller, JointState};

pub type SignalState = self::robot_pb::enableEncoderEvent_In_SignalState;

/// Callback arguments: (timestamp, x, y, z) where "x,y,z" are in units of "G's"
pub type AccelerometerEventHandler = FnMut(u32, f32, f32, f32);

/// Callback arguments: (timestamp, button, buttonstate).
///
/// The button indexes of 0, 1, and 2 correspond to the power, A, and B buttons on the robot,
/// respectively. 
///
/// The button state is "1" for button down and "0" for button up.
pub type ButtonEventHandler = FnMut(u32, u32, u32);

/// Callback arguments: (timestamp, mask, values)
pub type EncoderEventHandler = FnMut(u32, u32, Vec<f32>);

pub type ConnectEventHandler = FnMut(u32);

/// Callback arguments: (timestamp, joint, JointState, angle)
pub type JointEventHandler = FnMut(u32, u32, JointState, f32);

#[derive(Clone)]
pub struct Robot {
    inner: Arc<Mutex<Inner>>,
}

unsafe impl Send for Robot {}
unsafe impl Sync for Robot {}

impl Robot {
    pub fn new_from_daemon(serial_id: String, d: &daemon::DaemonProxy) -> Robot {
        Robot{ inner: Arc::new( Mutex::new( 
                    Inner::new_from_daemon(serial_id, d)
                    )),
        }
    }

    pub fn deliver(&mut self, payload: robot_pb::RobotToClient) -> Result<(), String> {
        self.inner.lock().unwrap().deliver(payload)
    }

    pub fn get_accelerometer_data<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32, f32, f32),
              F: 'static
    {
        self.inner.lock().unwrap().get_accelerometer_data(cb)
    }

    pub fn get_battery_voltage<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        self.inner.lock().unwrap().get_battery_voltage(cb)
    }

    pub fn get_button_state<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32),
              F: 'static
    {
        self.inner.lock().unwrap().get_button_state(cb)
    }

    pub fn get_buzzer_frequency<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(f32),
              F: 'static
    {
        self.inner.lock().unwrap().get_buzzer_frequency(cb)
    }

    pub fn get_encoder_values<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_encoder_values(cb)
    }

    pub fn get_firmware_version_string<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(String),
              F: 'static
    {
        self.inner.lock().unwrap().get_firmware_version_string(cb)
    }

    pub fn get_form_factor<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(robot_pb::FormFactor),
              F: 'static
    {
        self.inner.lock().unwrap().get_form_factor(cb)
    }

    pub fn get_joint_states<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u32, Vec<robot_pb::JointState>),
              F: 'static
    {
        self.inner.lock().unwrap().get_joint_states(cb)
    }

    pub fn get_led_color<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(u8, u8, u8),
              F: 'static
    {
        self.inner.lock().unwrap().get_led_color(cb)
    }

    pub fn get_motor_controller_omega<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_omega(cb)
    }

    pub fn get_motor_controller_alpha_i<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_alpha_i(cb)
    }

    pub fn get_motor_controller_alpha_f<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_alpha_f(cb)
    }

    pub fn get_motor_controller_proportional_gain<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_proportional_gain(cb)
    }

    pub fn get_motor_controller_integrator_gain<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_integrator_gain(cb)
    }

    pub fn get_motor_controller_derivative_gain<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_derivative_gain(cb)
    }

    pub fn get_motor_controller_safety_threshold<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<u32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_safety_threshold(cb)
    }

    pub fn get_motor_controller_safety_angle<F>(&mut self, cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        self.inner.lock().unwrap().get_motor_controller_safety_angle(cb)
    }

    pub fn set_motor_controller_omega<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_omega(mask, values, cb)
    }

    pub fn set_motor_controller_alpha_i<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_alpha_i(mask, values, cb)
    }

    pub fn set_motor_controller_alpha_f<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_alpha_f(mask, values, cb)
    }

    pub fn set_motor_controller_proportional_gain<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_proportional_gain(mask, values, cb)
    }

    pub fn set_motor_controller_derivative_gain<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_derivative_gain(mask, values, cb)
    }

    pub fn set_motor_controller_safety_threshold<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<u32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_safety_threshold(mask, values, cb)
    }

    pub fn set_motor_controller_safety_angle<F>(&mut self, 
                                         mask: u32, 
                                         values: Vec<f32>, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_motor_controller_safety_angle(mask, values, cb)
    }

    pub fn set_reset_on_disconnect<F>(&mut self, 
                                         mask: u32, 
                                         peripheral_mask: u32, 
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_reset_on_disconnect(mask, peripheral_mask, cb)
    }

    pub fn reset_encoder_revs<F>(&mut self, 
                                 cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().reset_encoder_revs(cb)
    }

    pub fn set_buzzer_frequency<F>(&mut self, 
                                   value: f32, 
                                   cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_buzzer_frequency(value, cb)
    }

    pub fn set_led_color<F>(&mut self, 
                            red: u8, 
                            green: u8, 
                            blue: u8,
                            cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().set_led_color(red,green,blue,cb)
    }

    pub fn stop<F>(&mut self, 
                   mask: Option<u32>,
                   cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().stop(mask, cb)
    }

    pub fn robot_move<F>(&mut self, 
                        motor1: Option<Goal>,
                        motor2: Option<Goal>,
                        motor3: Option<Goal>,
                        cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().robot_move(motor1, motor2, motor3, cb)
    }

    // Robot Events

    pub fn enable_accelerometer_event<F>(&mut self, 
                                         enable: bool,
                                         granularity: Option<f32>,
                                         cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().enable_accelerometer_event(enable, granularity, cb)
    }

    pub fn enable_button_event<F>(&mut self, 
                                  enable: bool,
                                  cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().enable_button_event(enable, cb)
    }

    pub fn enable_encoder_event<F>(&mut self, 
                                   encoder1: Option<SignalState>,
                                   encoder2: Option<SignalState>,
                                   encoder3: Option<SignalState>,
                                   cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().enable_encoder_event(encoder1,
                                                     encoder2,
                                                     encoder3,
                                                     cb)
    }

    pub fn enable_joint_event<F>(&mut self, 
                                 enable: bool,
                                 cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().enable_joint_event(enable, cb)
    }

    pub fn set_button_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, robot_pb::Button, robot_pb::ButtonState),
              F: 'static
    {
        self.inner.lock().unwrap().set_button_event_handler(handler);
    }

    pub fn set_connect_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32),
              F: 'static
    {
        self.inner.lock().unwrap().set_connect_event_handler(handler);
    }

    pub fn set_encoder_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, u32, Vec<f32>), // (timestamp, mask, values)
              F: 'static
    {
        self.inner.lock().unwrap().set_encoder_event_handler(handler);
    }

    pub fn set_joint_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, u32, robot_pb::JointState, f32),
              F: 'static
    {
        self.inner.lock().unwrap().set_joint_event_handler(handler);
    }


    // Miscellaneous functions

    pub fn write_twi<F>(&mut self, 
                        address: u32,
                        data: Vec<u8>,
                        cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        self.inner.lock().unwrap().write_twi(address, data, cb)
    }

    pub fn read_twi<F>(&mut self, 
                        address: u32,
                        recvsize: u32,
                        cb: F) -> Result<(), String>
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        self.inner.lock().unwrap().read_twi(address, recvsize, cb)
    }

    pub fn write_read_twi<F>(&mut self, 
                        address: u32,
                        recvsize: u32,
                        data: Vec<u8>,
                        cb: F) -> Result<(), String>
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        self.inner.lock().unwrap().write_read_twi(address, recvsize, data, cb)
    }

}

struct Inner
{
    serial_id: String,
    daemon: daemon::DaemonProxy,
    seq: u32,
    requests: HashMap<u32, Box<FnMut(robot_pb::RpcReply)>>,
    button_handler: Option<Box<FnMut(u32, robot_pb::Button, robot_pb::ButtonState)>>,
    connect_handler: Option<Box<ConnectEventHandler>>,
    joint_handler: Option<Box<JointEventHandler>>,
    encoder_handler: Option<Box<EncoderEventHandler>>,
}

impl Inner {
    fn new_from_daemon(serial_id: String, d: &daemon::DaemonProxy) -> Inner {
        Inner{ serial_id: serial_id,
               daemon: d.clone(),
               seq: rand::random(),
               requests: HashMap::new(),
               button_handler: None,
               connect_handler: None,
               joint_handler: None,
               encoder_handler: None,
        }
    }

    fn deliver(&mut self, mut payload: robot_pb::RobotToClient) -> Result<(), String> {
        if payload.has_rpcReply() {
            self.handle_rpc_reply(payload.take_rpcReply())
        } else if payload.has_buttonEvent() {
            self.handle_button_event(payload.take_buttonEvent())
        } else if payload.has_connectEvent() {
            self.handle_connect_event(payload.take_connectEvent())
        } else if payload.has_jointEvent() {
            self.handle_joint_event(payload.take_jointEvent())
        } else if payload.has_encoderEvent() {
            self.handle_encoder_event(payload.take_encoderEvent())
        } else {
            warn!("Robot message handler unimplemented!");
            Ok(())
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

    fn handle_button_event(&mut self, event: robot_pb::ButtonEvent) -> Result<(), String> {
        if let Some(ref mut cb) = self.button_handler {
            cb(event.get_timestamp(), event.get_button(), event.get_state());
            Ok(())
        } else {
            Err(format!("Robot received button event but there is no event handler set."))
        }
    }

    fn handle_connect_event(&mut self, event: robot_pb::ConnectEvent) -> Result<(), String> {
        if let Some(ref mut cb) = self.connect_handler {
            cb(event.get_timestamp());
        }
        Ok(())
    }

    fn handle_joint_event(&mut self, event: robot_pb::JointEvent) -> Result<(), String> {
        if let Some(ref mut cb) = self.joint_handler {
            cb( event.get_timestamp(),
                event.get_joint(),
                event.get_event(),
                event.get_angle()
            );
        }
        Ok(())
    }

    fn handle_encoder_event(&mut self, mut event: robot_pb::EncoderEvent) -> Result<(), String> {
        if let Some(ref mut cb) = self.encoder_handler {
            cb( event.get_timestamp(),
                event.get_mask(),
                event.take_values()
            );
        }
        Ok(())
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

    fn get_motor_controller_omega<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerOmega( 
            robot_pb::getMotorControllerOmega_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerOmega() {
                warn!("Reply has no form factor data.");
                return;
            }
            let mut data = reply.take_getMotorControllerOmega();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_alpha_i<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerAlphaI( 
            robot_pb::getMotorControllerAlphaI_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerAlphaI() {
                warn!("Reply has no getMotorControllerAlphaI data.");
                return;
            }
            let mut data = reply.take_getMotorControllerAlphaI();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_alpha_f<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerAlphaF( 
            robot_pb::getMotorControllerAlphaF_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerAlphaF() {
                warn!("Reply has no getMotorControllerAlphaF data.");
                return;
            }
            let mut data = reply.take_getMotorControllerAlphaF();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_proportional_gain<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerAlphaF( 
            robot_pb::getMotorControllerAlphaF_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerAlphaF() {
                warn!("Reply has no getMotorControllerAlphaF data.");
                return;
            }
            let mut data = reply.take_getMotorControllerAlphaF();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_integrator_gain<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerIntegratorGain( 
            robot_pb::getMotorControllerIntegratorGain_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerIntegratorGain() {
                warn!("Reply has no getMotorControllerIntegratorGain data.");
                return;
            }
            let mut data = reply.take_getMotorControllerIntegratorGain();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_derivative_gain<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerDerivativeGain( 
            robot_pb::getMotorControllerDerivativeGain_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerDerivativeGain() {
                warn!("Reply has no getMotorControllerDerivativeGain data.");
                return;
            }
            let mut data = reply.take_getMotorControllerDerivativeGain();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_safety_threshold<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<u32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerSafetyThreshold( 
            robot_pb::getMotorControllerSafetyThreshold_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerSafetyThreshold() {
                warn!("Reply has no getMotorControllerSafetyThreshold data.");
                return;
            }
            let mut data = reply.take_getMotorControllerSafetyThreshold();
            cb(data.take_values());
        })
    }

    fn get_motor_controller_safety_angle<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<f32>),
              F: 'static
    {
        let mut request = robot_pb::RpcRequest::new();
        request.set_getMotorControllerSafetyAngle( 
            robot_pb::getMotorControllerSafetyAngle_In::new()
            );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_getMotorControllerSafetyAngle() {
                warn!("Reply has no getMotorControllerSafetyAngle data.");
                return;
            }
            let mut data = reply.take_getMotorControllerSafetyAngle();
            cb(data.take_values());
        })
    }

    fn set_motor_controller_omega<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerOmega_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerOmega( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerOmega() {
                warn!("Reply has no setMotorControllerOmega data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_alpha_i<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerAlphaI_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerAlphaI( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerAlphaI() {
                warn!("Reply has no setMotorControllerAlphaI data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_alpha_f<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerAlphaF_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerAlphaF( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerAlphaF() {
                warn!("Reply has no setMotorControllerAlphaF data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_proportional_gain<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerProportionalGain_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerProportionalGain( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerProportionalGain() {
                warn!("Reply has no setMotorControllerProportionalGain data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_derivative_gain<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerDerivativeGain_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerDerivativeGain( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerDerivativeGain() {
                warn!("Reply has no setMotorControllerDerivativeGain data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_safety_threshold<F>(&mut self, mask: u32, values: Vec<u32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerSafetyThreshold_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerSafetyThreshold( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerSafetyThreshold() {
                warn!("Reply has no setMotorControllerSafetyThreshold data.");
                return;
            }
            cb();
        })
    }

    fn set_motor_controller_safety_angle<F>(&mut self, mask: u32, values: Vec<f32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setMotorControllerSafetyAngle_In::new();
        message.set_mask(mask);
        message.set_values(values);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setMotorControllerSafetyAngle( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setMotorControllerSafetyAngle() {
                warn!("Reply has no setMotorControllerSafetyAngle data.");
                return;
            }
            cb();
        })
    }

    fn set_reset_on_disconnect<F>(&mut self, mask: u32, peripheral_mask: u32, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setResetOnDisconnect_In::new();
        message.set_mask(mask);
        message.set_peripheralResetMask(peripheral_mask);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setResetOnDisconnect( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setResetOnDisconnect() {
                warn!("Reply has no setResetOnDisconnect data.");
                return;
            }
            cb();
        })
    }

    fn reset_encoder_revs<F>(&mut self, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let message = robot_pb::resetEncoderRevs_In::new();
        let mut request = robot_pb::RpcRequest::new();
        request.set_resetEncoderRevs( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_resetEncoderRevs() {
                warn!("Reply has no resetEncoderRevs data.");
                return;
            }
            cb();
        })
    }

    fn set_buzzer_frequency<F>(&mut self, value: f32, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::setBuzzerFrequency_In::new();
        message.set_value(value);
        let mut request = robot_pb::RpcRequest::new();
        request.set_setBuzzerFrequency( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_setBuzzerFrequency() {
                warn!("Reply has no setBuzzerFrequency data.");
                return;
            }
            cb();
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

    fn stop<F>(&mut self, mask: Option<u32>, mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::stop_In::new();
        if let Some(mask) = mask {
            message.set_mask(mask);
        }
        let mut request = robot_pb::RpcRequest::new();
        request.set_stop( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_stop() {
                warn!("Reply has no stop data.");
                return;
            }
            cb();
        })
    }

    fn robot_move<F>(&mut self, 
                    motor1: Option<Goal>, 
                    motor2: Option<Goal>,
                    motor3: Option<Goal>,
                    mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::robotMove_In::new();
        if let Some(goal) = motor1 {
            message.set_motorOneGoal(goal);
        }
        if let Some(goal) = motor2 {
            message.set_motorTwoGoal(goal);
        }
        if let Some(goal) = motor3 {
            message.set_motorThreeGoal(goal);
        }
        let mut request = robot_pb::RpcRequest::new();
        request.set_robotMove( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_robotMove() {
                warn!("Reply has no robotMove data.");
                return;
            }
            cb();
        })
    }

    fn enable_accelerometer_event<F>(&mut self, 
                                     enable: bool,
                                     granularity: Option<f32>, 
                                     mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::enableAccelerometerEvent_In::new();
        message.set_enable(enable);
        if let Some(g) = granularity {
            message.set_granularity(g);
        }
        let mut request = robot_pb::RpcRequest::new();
        request.set_enableAccelerometerEvent( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_enableAccelerometerEvent() {
                warn!("Reply has no enableAccelerometerEvent data.");
                return;
            }
            cb();
        })
    }

    fn enable_button_event<F>(&mut self, 
                                     enable: bool,
                                     mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::enableButtonEvent_In::new();
        message.set_enable(enable);
        let mut request = robot_pb::RpcRequest::new();
        request.set_enableButtonEvent( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_enableButtonEvent() {
                warn!("Reply has no enableButtonEvent data.");
                return;
            }
            cb();
        })
    }

    fn enable_encoder_event<F>(&mut self, 
                               encoder1: Option<SignalState>,
                               encoder2: Option<SignalState>,
                               encoder3: Option<SignalState>,
                               mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::enableEncoderEvent_In::new();
        if let Some(e) = encoder1 {
            message.set_encoderOne(e);
        }
        if let Some(e) = encoder2 {
            message.set_encoderTwo(e);
        }
        if let Some(e) = encoder3 {
            message.set_encoderThree(e);
        }
        let mut request = robot_pb::RpcRequest::new();
        request.set_enableEncoderEvent( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_enableEncoderEvent() {
                warn!("Reply has no enableEncoderEvent data.");
                return;
            }
            cb();
        })
    }

    fn enable_joint_event<F>(&mut self, 
                             enable: bool,
                             mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::enableJointEvent_In::new();
        message.set_enable(enable);
        let mut request = robot_pb::RpcRequest::new();
        request.set_enableJointEvent( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_enableJointEvent() {
                warn!("Reply has no enableJointEvent data.");
                return;
            }
            cb();
        })
    }


    fn set_button_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, robot_pb::Button, robot_pb::ButtonState),
              F: 'static
    {
        self.button_handler = Some( Box::new( handler ) );
    }

    fn set_encoder_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, u32, Vec<f32>), // (timestamp, mask, values)
              F: 'static
    {
        self.encoder_handler = Some( Box::new( handler ) );
    }

    fn set_connect_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32),
              F: 'static
    {
        self.connect_handler = Some( Box::new( handler ) );
    }

    fn set_joint_event_handler<F>(&mut self, handler: F)
        where F: FnMut(u32, u32, JointState, f32),
              F: 'static
    {
        self.joint_handler = Some( Box::new( handler ) );
    }

    fn write_twi<F>(&mut self, 
                    address: u32,
                    data: Vec<u8>,
                    mut cb: F) -> Result<(), String>
        where F: FnMut(),
              F: 'static
    {
        let mut message = robot_pb::writeTwi_In::new();
        message.set_address(address);
        message.set_data(data);
        let mut request = robot_pb::RpcRequest::new();
        request.set_writeTwi( message );
        self.rpc_request(request, move |reply| {
            if ! reply.has_writeTwi() {
                warn!("Reply has no writeTwi data.");
                return;
            }
            cb();
        })
    }

    fn read_twi<F>(&mut self, 
                    address: u32,
                    recvsize: u32,
                    mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        let mut message = robot_pb::readTwi_In::new();
        message.set_address(address);
        message.set_recvsize(recvsize);
        let mut request = robot_pb::RpcRequest::new();
        request.set_readTwi( message );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_readTwi() {
                warn!("Reply has no readTwi data.");
                return;
            }
            cb(reply.take_readTwi().take_data());
        })
    }

    fn write_read_twi<F>(&mut self, 
                    address: u32,
                    recvsize: u32,
                    data: Vec<u8>,
                    mut cb: F) -> Result<(), String>
        where F: FnMut(Vec<u8>),
              F: 'static
    {
        let mut message = robot_pb::writeReadTwi_In::new();
        message.set_address(address);
        message.set_recvsize(recvsize);
        message.set_data(data);
        let mut request = robot_pb::RpcRequest::new();
        request.set_writeReadTwi( message );
        self.rpc_request(request, move |mut reply| {
            if ! reply.has_writeReadTwi() {
                warn!("Reply has no writeReadTwi data.");
                return;
            }
            cb(reply.take_writeReadTwi().take_data());
        })
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
