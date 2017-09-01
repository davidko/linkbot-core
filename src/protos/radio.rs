// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ConnectSession {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectSession {}

impl ConnectSession {
    pub fn new() -> ConnectSession {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectSession {
        static mut instance: ::protobuf::lazy::Lazy<ConnectSession> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectSession,
        };
        unsafe {
            instance.get(ConnectSession::new)
        }
    }
}

impl ::protobuf::Message for ConnectSession {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectSession {
    fn new() -> ConnectSession {
        ConnectSession::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectSession>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ConnectSession>(
                    "ConnectSession",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectSession {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectSession {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectSession {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DisconnectSession {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DisconnectSession {}

impl DisconnectSession {
    pub fn new() -> DisconnectSession {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DisconnectSession {
        static mut instance: ::protobuf::lazy::Lazy<DisconnectSession> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DisconnectSession,
        };
        unsafe {
            instance.get(DisconnectSession::new)
        }
    }
}

impl ::protobuf::Message for DisconnectSession {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DisconnectSession {
    fn new() -> DisconnectSession {
        DisconnectSession::new()
    }

    fn descriptor_static(_: ::std::option::Option<DisconnectSession>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DisconnectSession>(
                    "DisconnectSession",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DisconnectSession {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DisconnectSession {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisconnectSession {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PowerOnEvent {
    // message fields
    version: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PowerOnEvent {}

impl PowerOnEvent {
    pub fn new() -> PowerOnEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PowerOnEvent {
        static mut instance: ::protobuf::lazy::Lazy<PowerOnEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PowerOnEvent,
        };
        unsafe {
            instance.get(PowerOnEvent::new)
        }
    }

    // optional string version = 1;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }
}

impl ::protobuf::Message for PowerOnEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PowerOnEvent {
    fn new() -> PowerOnEvent {
        PowerOnEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<PowerOnEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    PowerOnEvent::get_version_for_reflect,
                    PowerOnEvent::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PowerOnEvent>(
                    "PowerOnEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PowerOnEvent {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PowerOnEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PowerOnEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RadioMessage {
    // message oneof groups
    arg: ::std::option::Option<RadioMessage_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RadioMessage {}

#[derive(Clone,PartialEq)]
pub enum RadioMessage_oneof_arg {
    clientToRobot(super::robot::ClientToRobot),
    clientToRobotBroadcast(super::robot::ClientToRobotBroadcast),
    robotToClient(super::robot::RobotToClient),
    bumpConnect(super::bumpconnect::RobotToRobot),
    connectSession(ConnectSession),
    disconnectSession(DisconnectSession),
    powerOnEvent(PowerOnEvent),
}

impl RadioMessage {
    pub fn new() -> RadioMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RadioMessage {
        static mut instance: ::protobuf::lazy::Lazy<RadioMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RadioMessage,
        };
        unsafe {
            instance.get(RadioMessage::new)
        }
    }

    // optional .linkbot.robot.ClientToRobot clientToRobot = 1;

    pub fn clear_clientToRobot(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_clientToRobot(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clientToRobot(&mut self, v: super::robot::ClientToRobot) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(v))
    }

    // Mutable pointer to the field.
    pub fn mut_clientToRobot(&mut self) -> &mut super::robot::ClientToRobot {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(super::robot::ClientToRobot::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_clientToRobot(&mut self) -> super::robot::ClientToRobot {
        if self.has_clientToRobot() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(v)) => v,
                _ => panic!(),
            }
        } else {
            super::robot::ClientToRobot::new()
        }
    }

    pub fn get_clientToRobot(&self) -> &super::robot::ClientToRobot {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(ref v)) => v,
            _ => super::robot::ClientToRobot::default_instance(),
        }
    }

    // optional .linkbot.robot.ClientToRobotBroadcast clientToRobotBroadcast = 2;

    pub fn clear_clientToRobotBroadcast(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_clientToRobotBroadcast(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clientToRobotBroadcast(&mut self, v: super::robot::ClientToRobotBroadcast) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(v))
    }

    // Mutable pointer to the field.
    pub fn mut_clientToRobotBroadcast(&mut self) -> &mut super::robot::ClientToRobotBroadcast {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(super::robot::ClientToRobotBroadcast::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_clientToRobotBroadcast(&mut self) -> super::robot::ClientToRobotBroadcast {
        if self.has_clientToRobotBroadcast() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(v)) => v,
                _ => panic!(),
            }
        } else {
            super::robot::ClientToRobotBroadcast::new()
        }
    }

    pub fn get_clientToRobotBroadcast(&self) -> &super::robot::ClientToRobotBroadcast {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(ref v)) => v,
            _ => super::robot::ClientToRobotBroadcast::default_instance(),
        }
    }

    // optional .linkbot.robot.RobotToClient robotToClient = 3;

    pub fn clear_robotToClient(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_robotToClient(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_robotToClient(&mut self, v: super::robot::RobotToClient) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(v))
    }

    // Mutable pointer to the field.
    pub fn mut_robotToClient(&mut self) -> &mut super::robot::RobotToClient {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(super::robot::RobotToClient::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_robotToClient(&mut self) -> super::robot::RobotToClient {
        if self.has_robotToClient() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(v)) => v,
                _ => panic!(),
            }
        } else {
            super::robot::RobotToClient::new()
        }
    }

    pub fn get_robotToClient(&self) -> &super::robot::RobotToClient {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(ref v)) => v,
            _ => super::robot::RobotToClient::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.RobotToRobot bumpConnect = 4;

    pub fn clear_bumpConnect(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_bumpConnect(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bumpConnect(&mut self, v: super::bumpconnect::RobotToRobot) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bumpConnect(&mut self) -> &mut super::bumpconnect::RobotToRobot {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(super::bumpconnect::RobotToRobot::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bumpConnect(&mut self) -> super::bumpconnect::RobotToRobot {
        if self.has_bumpConnect() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::bumpconnect::RobotToRobot::new()
        }
    }

    pub fn get_bumpConnect(&self) -> &super::bumpconnect::RobotToRobot {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(ref v)) => v,
            _ => super::bumpconnect::RobotToRobot::default_instance(),
        }
    }

    // optional .linkbot.radio.ConnectSession connectSession = 5;

    pub fn clear_connectSession(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_connectSession(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_connectSession(&mut self, v: ConnectSession) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(v))
    }

    // Mutable pointer to the field.
    pub fn mut_connectSession(&mut self) -> &mut ConnectSession {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(ConnectSession::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_connectSession(&mut self) -> ConnectSession {
        if self.has_connectSession() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(v)) => v,
                _ => panic!(),
            }
        } else {
            ConnectSession::new()
        }
    }

    pub fn get_connectSession(&self) -> &ConnectSession {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(ref v)) => v,
            _ => ConnectSession::default_instance(),
        }
    }

    // optional .linkbot.radio.DisconnectSession disconnectSession = 6;

    pub fn clear_disconnectSession(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_disconnectSession(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnectSession(&mut self, v: DisconnectSession) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(v))
    }

    // Mutable pointer to the field.
    pub fn mut_disconnectSession(&mut self) -> &mut DisconnectSession {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(DisconnectSession::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnectSession(&mut self) -> DisconnectSession {
        if self.has_disconnectSession() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(v)) => v,
                _ => panic!(),
            }
        } else {
            DisconnectSession::new()
        }
    }

    pub fn get_disconnectSession(&self) -> &DisconnectSession {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(ref v)) => v,
            _ => DisconnectSession::default_instance(),
        }
    }

    // optional .linkbot.radio.PowerOnEvent powerOnEvent = 7;

    pub fn clear_powerOnEvent(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_powerOnEvent(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_powerOnEvent(&mut self, v: PowerOnEvent) {
        self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(v))
    }

    // Mutable pointer to the field.
    pub fn mut_powerOnEvent(&mut self) -> &mut PowerOnEvent {
        if let ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(PowerOnEvent::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_powerOnEvent(&mut self) -> PowerOnEvent {
        if self.has_powerOnEvent() {
            match self.arg.take() {
                ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(v)) => v,
                _ => panic!(),
            }
        } else {
            PowerOnEvent::new()
        }
    }

    pub fn get_powerOnEvent(&self) -> &PowerOnEvent {
        match self.arg {
            ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(ref v)) => v,
            _ => PowerOnEvent::default_instance(),
        }
    }
}

impl ::protobuf::Message for RadioMessage {
    fn is_initialized(&self) -> bool {
        if let Some(RadioMessage_oneof_arg::clientToRobot(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::clientToRobotBroadcast(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::robotToClient(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::bumpConnect(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::connectSession(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::disconnectSession(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RadioMessage_oneof_arg::powerOnEvent(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobot(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::clientToRobotBroadcast(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::robotToClient(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::bumpConnect(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::connectSession(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::disconnectSession(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RadioMessage_oneof_arg::powerOnEvent(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RadioMessage_oneof_arg::clientToRobot(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::clientToRobotBroadcast(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::robotToClient(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::bumpConnect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::connectSession(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::disconnectSession(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RadioMessage_oneof_arg::powerOnEvent(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RadioMessage_oneof_arg::clientToRobot(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::clientToRobotBroadcast(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::robotToClient(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::bumpConnect(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::connectSession(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::disconnectSession(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RadioMessage_oneof_arg::powerOnEvent(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RadioMessage {
    fn new() -> RadioMessage {
        RadioMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RadioMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::robot::ClientToRobot>(
                    "clientToRobot",
                    RadioMessage::has_clientToRobot,
                    RadioMessage::get_clientToRobot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::robot::ClientToRobotBroadcast>(
                    "clientToRobotBroadcast",
                    RadioMessage::has_clientToRobotBroadcast,
                    RadioMessage::get_clientToRobotBroadcast,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::robot::RobotToClient>(
                    "robotToClient",
                    RadioMessage::has_robotToClient,
                    RadioMessage::get_robotToClient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::bumpconnect::RobotToRobot>(
                    "bumpConnect",
                    RadioMessage::has_bumpConnect,
                    RadioMessage::get_bumpConnect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ConnectSession>(
                    "connectSession",
                    RadioMessage::has_connectSession,
                    RadioMessage::get_connectSession,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DisconnectSession>(
                    "disconnectSession",
                    RadioMessage::has_disconnectSession,
                    RadioMessage::get_disconnectSession,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PowerOnEvent>(
                    "powerOnEvent",
                    RadioMessage::has_powerOnEvent,
                    RadioMessage::get_powerOnEvent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RadioMessage>(
                    "RadioMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RadioMessage {
    fn clear(&mut self) {
        self.clear_clientToRobot();
        self.clear_clientToRobotBroadcast();
        self.clear_robotToClient();
        self.clear_bumpConnect();
        self.clear_connectSession();
        self.clear_disconnectSession();
        self.clear_powerOnEvent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RadioMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RadioMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoutedRadioMessage {
    // message fields
    destinations: ::protobuf::RepeatedField<super::commontypes::SerialId>,
    sessionId: ::std::option::Option<u32>,
    payload: ::protobuf::SingularPtrField<RadioMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoutedRadioMessage {}

impl RoutedRadioMessage {
    pub fn new() -> RoutedRadioMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoutedRadioMessage {
        static mut instance: ::protobuf::lazy::Lazy<RoutedRadioMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoutedRadioMessage,
        };
        unsafe {
            instance.get(RoutedRadioMessage::new)
        }
    }

    // repeated .linkbot.SerialId destinations = 1;

    pub fn clear_destinations(&mut self) {
        self.destinations.clear();
    }

    // Param is passed by value, moved
    pub fn set_destinations(&mut self, v: ::protobuf::RepeatedField<super::commontypes::SerialId>) {
        self.destinations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_destinations(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.destinations
    }

    // Take field
    pub fn take_destinations(&mut self) -> ::protobuf::RepeatedField<super::commontypes::SerialId> {
        ::std::mem::replace(&mut self.destinations, ::protobuf::RepeatedField::new())
    }

    pub fn get_destinations(&self) -> &[super::commontypes::SerialId] {
        &self.destinations
    }

    fn get_destinations_for_reflect(&self) -> &::protobuf::RepeatedField<super::commontypes::SerialId> {
        &self.destinations
    }

    fn mut_destinations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.destinations
    }

    // optional uint32 sessionId = 2;

    pub fn clear_sessionId(&mut self) {
        self.sessionId = ::std::option::Option::None;
    }

    pub fn has_sessionId(&self) -> bool {
        self.sessionId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sessionId(&mut self, v: u32) {
        self.sessionId = ::std::option::Option::Some(v);
    }

    pub fn get_sessionId(&self) -> u32 {
        self.sessionId.unwrap_or(0)
    }

    fn get_sessionId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sessionId
    }

    fn mut_sessionId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sessionId
    }

    // optional .linkbot.radio.RadioMessage payload = 3;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: RadioMessage) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut RadioMessage {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> RadioMessage {
        self.payload.take().unwrap_or_else(|| RadioMessage::new())
    }

    pub fn get_payload(&self) -> &RadioMessage {
        self.payload.as_ref().unwrap_or_else(|| RadioMessage::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<RadioMessage> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RadioMessage> {
        &mut self.payload
    }
}

impl ::protobuf::Message for RoutedRadioMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.destinations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.payload {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.destinations)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sessionId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.sessionId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.destinations {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.sessionId {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RoutedRadioMessage {
    fn new() -> RoutedRadioMessage {
        RoutedRadioMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoutedRadioMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "destinations",
                    RoutedRadioMessage::get_destinations_for_reflect,
                    RoutedRadioMessage::mut_destinations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sessionId",
                    RoutedRadioMessage::get_sessionId_for_reflect,
                    RoutedRadioMessage::mut_sessionId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RadioMessage>>(
                    "payload",
                    RoutedRadioMessage::get_payload_for_reflect,
                    RoutedRadioMessage::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoutedRadioMessage>(
                    "RoutedRadioMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoutedRadioMessage {
    fn clear(&mut self) {
        self.clear_destinations();
        self.clear_sessionId();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoutedRadioMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoutedRadioMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bradio.proto\x12\rlinkbot.radio\x1a\x0cnanopb.proto\x1a\x0brobot.pr\
    oto\x1a\x11bumpconnect.proto\x1a\x11commontypes.proto\"\x10\n\x0eConnect\
    Session\"\x13\n\x11DisconnectSession\"0\n\x0cPowerOnEvent\x12\x20\n\x07v\
    ersion\x18\x01\x20\x01(\tR\x07versionB\x06\x92?\x03\x08\x80\x01\"\xa7\
    \x04\n\x0cRadioMessage\x12D\n\rclientToRobot\x18\x01\x20\x01(\x0b2\x1c.l\
    inkbot.robot.ClientToRobotH\0R\rclientToRobot\x12_\n\x16clientToRobotBro\
    adcast\x18\x02\x20\x01(\x0b2%.linkbot.robot.ClientToRobotBroadcastH\0R\
    \x16clientToRobotBroadcast\x12D\n\rrobotToClient\x18\x03\x20\x01(\x0b2\
    \x1c.linkbot.robot.RobotToClientH\0R\rrobotToClient\x12E\n\x0bbumpConnec\
    t\x18\x04\x20\x01(\x0b2!.linkbot.bumpconnect.RobotToRobotH\0R\x0bbumpCon\
    nect\x12G\n\x0econnectSession\x18\x05\x20\x01(\x0b2\x1d.linkbot.radio.Co\
    nnectSessionH\0R\x0econnectSession\x12P\n\x11disconnectSession\x18\x06\
    \x20\x01(\x0b2\x20.linkbot.radio.DisconnectSessionH\0R\x11disconnectSess\
    ion\x12A\n\x0cpowerOnEvent\x18\x07\x20\x01(\x0b2\x1b.linkbot.radio.Power\
    OnEventH\0R\x0cpowerOnEventB\x05\n\x03arg\"\xa7\x01\n\x12RoutedRadioMess\
    age\x12<\n\x0cdestinations\x18\x01\x20\x03(\x0b2\x11.linkbot.SerialIdR\
    \x0cdestinationsB\x05\x92?\x02\x10\x08\x12\x1c\n\tsessionId\x18\x02\x20\
    \x01(\rR\tsessionId\x125\n\x07payload\x18\x03\x20\x01(\x0b2\x1b.linkbot.\
    radio.RadioMessageR\x07payload\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
