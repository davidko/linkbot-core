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
pub struct ArbitrateMaster {
    // message fields
    millisSinceShock: ::std::option::Option<u32>,
    millisSinceButton: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ArbitrateMaster {}

impl ArbitrateMaster {
    pub fn new() -> ArbitrateMaster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ArbitrateMaster {
        static mut instance: ::protobuf::lazy::Lazy<ArbitrateMaster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ArbitrateMaster,
        };
        unsafe {
            instance.get(ArbitrateMaster::new)
        }
    }

    // required uint32 millisSinceShock = 1;

    pub fn clear_millisSinceShock(&mut self) {
        self.millisSinceShock = ::std::option::Option::None;
    }

    pub fn has_millisSinceShock(&self) -> bool {
        self.millisSinceShock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_millisSinceShock(&mut self, v: u32) {
        self.millisSinceShock = ::std::option::Option::Some(v);
    }

    pub fn get_millisSinceShock(&self) -> u32 {
        self.millisSinceShock.unwrap_or(0)
    }

    fn get_millisSinceShock_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.millisSinceShock
    }

    fn mut_millisSinceShock_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.millisSinceShock
    }

    // required uint32 millisSinceButton = 2;

    pub fn clear_millisSinceButton(&mut self) {
        self.millisSinceButton = ::std::option::Option::None;
    }

    pub fn has_millisSinceButton(&self) -> bool {
        self.millisSinceButton.is_some()
    }

    // Param is passed by value, moved
    pub fn set_millisSinceButton(&mut self, v: u32) {
        self.millisSinceButton = ::std::option::Option::Some(v);
    }

    pub fn get_millisSinceButton(&self) -> u32 {
        self.millisSinceButton.unwrap_or(0)
    }

    fn get_millisSinceButton_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.millisSinceButton
    }

    fn mut_millisSinceButton_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.millisSinceButton
    }
}

impl ::protobuf::Message for ArbitrateMaster {
    fn is_initialized(&self) -> bool {
        if self.millisSinceShock.is_none() {
            return false;
        }
        if self.millisSinceButton.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.millisSinceShock = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.millisSinceButton = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.millisSinceShock {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.millisSinceButton {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.millisSinceShock {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.millisSinceButton {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for ArbitrateMaster {
    fn new() -> ArbitrateMaster {
        ArbitrateMaster::new()
    }

    fn descriptor_static(_: ::std::option::Option<ArbitrateMaster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "millisSinceShock",
                    ArbitrateMaster::get_millisSinceShock_for_reflect,
                    ArbitrateMaster::mut_millisSinceShock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "millisSinceButton",
                    ArbitrateMaster::get_millisSinceButton_for_reflect,
                    ArbitrateMaster::mut_millisSinceButton_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ArbitrateMaster>(
                    "ArbitrateMaster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ArbitrateMaster {
    fn clear(&mut self) {
        self.clear_millisSinceShock();
        self.clear_millisSinceButton();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ArbitrateMaster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ArbitrateMaster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMotorPower {
    // message fields
    m1: ::std::option::Option<i32>,
    m2: ::std::option::Option<i32>,
    m3: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMotorPower {}

impl SetMotorPower {
    pub fn new() -> SetMotorPower {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMotorPower {
        static mut instance: ::protobuf::lazy::Lazy<SetMotorPower> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMotorPower,
        };
        unsafe {
            instance.get(SetMotorPower::new)
        }
    }

    // required int32 m1 = 1;

    pub fn clear_m1(&mut self) {
        self.m1 = ::std::option::Option::None;
    }

    pub fn has_m1(&self) -> bool {
        self.m1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m1(&mut self, v: i32) {
        self.m1 = ::std::option::Option::Some(v);
    }

    pub fn get_m1(&self) -> i32 {
        self.m1.unwrap_or(0)
    }

    fn get_m1_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.m1
    }

    fn mut_m1_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.m1
    }

    // required int32 m2 = 2;

    pub fn clear_m2(&mut self) {
        self.m2 = ::std::option::Option::None;
    }

    pub fn has_m2(&self) -> bool {
        self.m2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m2(&mut self, v: i32) {
        self.m2 = ::std::option::Option::Some(v);
    }

    pub fn get_m2(&self) -> i32 {
        self.m2.unwrap_or(0)
    }

    fn get_m2_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.m2
    }

    fn mut_m2_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.m2
    }

    // required int32 m3 = 3;

    pub fn clear_m3(&mut self) {
        self.m3 = ::std::option::Option::None;
    }

    pub fn has_m3(&self) -> bool {
        self.m3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m3(&mut self, v: i32) {
        self.m3 = ::std::option::Option::Some(v);
    }

    pub fn get_m3(&self) -> i32 {
        self.m3.unwrap_or(0)
    }

    fn get_m3_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.m3
    }

    fn mut_m3_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.m3
    }
}

impl ::protobuf::Message for SetMotorPower {
    fn is_initialized(&self) -> bool {
        if self.m1.is_none() {
            return false;
        }
        if self.m2.is_none() {
            return false;
        }
        if self.m3.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.m1 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.m2 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.m3 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.m1 {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.m2 {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.m3 {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.m1 {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.m2 {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.m3 {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for SetMotorPower {
    fn new() -> SetMotorPower {
        SetMotorPower::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMotorPower>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "m1",
                    SetMotorPower::get_m1_for_reflect,
                    SetMotorPower::mut_m1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "m2",
                    SetMotorPower::get_m2_for_reflect,
                    SetMotorPower::mut_m2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "m3",
                    SetMotorPower::get_m3_for_reflect,
                    SetMotorPower::mut_m3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetMotorPower>(
                    "SetMotorPower",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMotorPower {
    fn clear(&mut self) {
        self.clear_m1();
        self.clear_m2();
        self.clear_m3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMotorPower {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMotorPower {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetJointPos {
    // message fields
    m1: ::std::option::Option<f32>,
    m2: ::std::option::Option<f32>,
    m3: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetJointPos {}

impl SetJointPos {
    pub fn new() -> SetJointPos {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetJointPos {
        static mut instance: ::protobuf::lazy::Lazy<SetJointPos> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetJointPos,
        };
        unsafe {
            instance.get(SetJointPos::new)
        }
    }

    // required float m1 = 1;

    pub fn clear_m1(&mut self) {
        self.m1 = ::std::option::Option::None;
    }

    pub fn has_m1(&self) -> bool {
        self.m1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m1(&mut self, v: f32) {
        self.m1 = ::std::option::Option::Some(v);
    }

    pub fn get_m1(&self) -> f32 {
        self.m1.unwrap_or(0.)
    }

    fn get_m1_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.m1
    }

    fn mut_m1_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.m1
    }

    // required float m2 = 2;

    pub fn clear_m2(&mut self) {
        self.m2 = ::std::option::Option::None;
    }

    pub fn has_m2(&self) -> bool {
        self.m2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m2(&mut self, v: f32) {
        self.m2 = ::std::option::Option::Some(v);
    }

    pub fn get_m2(&self) -> f32 {
        self.m2.unwrap_or(0.)
    }

    fn get_m2_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.m2
    }

    fn mut_m2_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.m2
    }

    // required float m3 = 3;

    pub fn clear_m3(&mut self) {
        self.m3 = ::std::option::Option::None;
    }

    pub fn has_m3(&self) -> bool {
        self.m3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_m3(&mut self, v: f32) {
        self.m3 = ::std::option::Option::Some(v);
    }

    pub fn get_m3(&self) -> f32 {
        self.m3.unwrap_or(0.)
    }

    fn get_m3_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.m3
    }

    fn mut_m3_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.m3
    }
}

impl ::protobuf::Message for SetJointPos {
    fn is_initialized(&self) -> bool {
        if self.m1.is_none() {
            return false;
        }
        if self.m2.is_none() {
            return false;
        }
        if self.m3.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.m1 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.m2 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.m3 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.m1 {
            my_size += 5;
        }
        if let Some(v) = self.m2 {
            my_size += 5;
        }
        if let Some(v) = self.m3 {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.m1 {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.m2 {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.m3 {
            os.write_float(3, v)?;
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

impl ::protobuf::MessageStatic for SetJointPos {
    fn new() -> SetJointPos {
        SetJointPos::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetJointPos>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "m1",
                    SetJointPos::get_m1_for_reflect,
                    SetJointPos::mut_m1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "m2",
                    SetJointPos::get_m2_for_reflect,
                    SetJointPos::mut_m2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "m3",
                    SetJointPos::get_m3_for_reflect,
                    SetJointPos::mut_m3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetJointPos>(
                    "SetJointPos",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetJointPos {
    fn clear(&mut self) {
        self.clear_m1();
        self.clear_m2();
        self.clear_m3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetJointPos {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetJointPos {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupInfo {
    // message fields
    groupId: ::std::option::Option<u32>,
    rgb: ::std::option::Option<u32>,
    masterId: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GroupInfo {}

impl GroupInfo {
    pub fn new() -> GroupInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GroupInfo {
        static mut instance: ::protobuf::lazy::Lazy<GroupInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupInfo,
        };
        unsafe {
            instance.get(GroupInfo::new)
        }
    }

    // required uint32 groupId = 1;

    pub fn clear_groupId(&mut self) {
        self.groupId = ::std::option::Option::None;
    }

    pub fn has_groupId(&self) -> bool {
        self.groupId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupId(&mut self, v: u32) {
        self.groupId = ::std::option::Option::Some(v);
    }

    pub fn get_groupId(&self) -> u32 {
        self.groupId.unwrap_or(0)
    }

    fn get_groupId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.groupId
    }

    fn mut_groupId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.groupId
    }

    // required uint32 rgb = 2;

    pub fn clear_rgb(&mut self) {
        self.rgb = ::std::option::Option::None;
    }

    pub fn has_rgb(&self) -> bool {
        self.rgb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rgb(&mut self, v: u32) {
        self.rgb = ::std::option::Option::Some(v);
    }

    pub fn get_rgb(&self) -> u32 {
        self.rgb.unwrap_or(0)
    }

    fn get_rgb_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rgb
    }

    fn mut_rgb_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rgb
    }

    // optional .linkbot.SerialId masterId = 3;

    pub fn clear_masterId(&mut self) {
        self.masterId.clear();
    }

    pub fn has_masterId(&self) -> bool {
        self.masterId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_masterId(&mut self, v: super::commontypes::SerialId) {
        self.masterId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_masterId(&mut self) -> &mut super::commontypes::SerialId {
        if self.masterId.is_none() {
            self.masterId.set_default();
        }
        self.masterId.as_mut().unwrap()
    }

    // Take field
    pub fn take_masterId(&mut self) -> super::commontypes::SerialId {
        self.masterId.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_masterId(&self) -> &super::commontypes::SerialId {
        self.masterId.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_masterId_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.masterId
    }

    fn mut_masterId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.masterId
    }
}

impl ::protobuf::Message for GroupInfo {
    fn is_initialized(&self) -> bool {
        if self.groupId.is_none() {
            return false;
        }
        if self.rgb.is_none() {
            return false;
        }
        for v in &self.masterId {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.groupId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.rgb = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.masterId)?;
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
        if let Some(v) = self.groupId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rgb {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.masterId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.groupId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.rgb {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.masterId.as_ref() {
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

impl ::protobuf::MessageStatic for GroupInfo {
    fn new() -> GroupInfo {
        GroupInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GroupInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "groupId",
                    GroupInfo::get_groupId_for_reflect,
                    GroupInfo::mut_groupId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rgb",
                    GroupInfo::get_rgb_for_reflect,
                    GroupInfo::mut_rgb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "masterId",
                    GroupInfo::get_masterId_for_reflect,
                    GroupInfo::mut_masterId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GroupInfo>(
                    "GroupInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GroupInfo {
    fn clear(&mut self) {
        self.clear_groupId();
        self.clear_rgb();
        self.clear_masterId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GroupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetSlave {
    // message fields
    groupInfo: ::protobuf::SingularPtrField<GroupInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetSlave {}

impl SetSlave {
    pub fn new() -> SetSlave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetSlave {
        static mut instance: ::protobuf::lazy::Lazy<SetSlave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetSlave,
        };
        unsafe {
            instance.get(SetSlave::new)
        }
    }

    // required .linkbot.bumpconnect.GroupInfo groupInfo = 1;

    pub fn clear_groupInfo(&mut self) {
        self.groupInfo.clear();
    }

    pub fn has_groupInfo(&self) -> bool {
        self.groupInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupInfo(&mut self, v: GroupInfo) {
        self.groupInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupInfo(&mut self) -> &mut GroupInfo {
        if self.groupInfo.is_none() {
            self.groupInfo.set_default();
        }
        self.groupInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupInfo(&mut self) -> GroupInfo {
        self.groupInfo.take().unwrap_or_else(|| GroupInfo::new())
    }

    pub fn get_groupInfo(&self) -> &GroupInfo {
        self.groupInfo.as_ref().unwrap_or_else(|| GroupInfo::default_instance())
    }

    fn get_groupInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<GroupInfo> {
        &self.groupInfo
    }

    fn mut_groupInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GroupInfo> {
        &mut self.groupInfo
    }
}

impl ::protobuf::Message for SetSlave {
    fn is_initialized(&self) -> bool {
        if self.groupInfo.is_none() {
            return false;
        }
        for v in &self.groupInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.groupInfo)?;
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
        if let Some(ref v) = self.groupInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.groupInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SetSlave {
    fn new() -> SetSlave {
        SetSlave::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetSlave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GroupInfo>>(
                    "groupInfo",
                    SetSlave::get_groupInfo_for_reflect,
                    SetSlave::mut_groupInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetSlave>(
                    "SetSlave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetSlave {
    fn clear(&mut self) {
        self.clear_groupInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetSlave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetSlave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetMaster {
    // message fields
    groupInfo: ::protobuf::SingularPtrField<GroupInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetMaster {}

impl SetMaster {
    pub fn new() -> SetMaster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetMaster {
        static mut instance: ::protobuf::lazy::Lazy<SetMaster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetMaster,
        };
        unsafe {
            instance.get(SetMaster::new)
        }
    }

    // required .linkbot.bumpconnect.GroupInfo groupInfo = 1;

    pub fn clear_groupInfo(&mut self) {
        self.groupInfo.clear();
    }

    pub fn has_groupInfo(&self) -> bool {
        self.groupInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupInfo(&mut self, v: GroupInfo) {
        self.groupInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupInfo(&mut self) -> &mut GroupInfo {
        if self.groupInfo.is_none() {
            self.groupInfo.set_default();
        }
        self.groupInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupInfo(&mut self) -> GroupInfo {
        self.groupInfo.take().unwrap_or_else(|| GroupInfo::new())
    }

    pub fn get_groupInfo(&self) -> &GroupInfo {
        self.groupInfo.as_ref().unwrap_or_else(|| GroupInfo::default_instance())
    }

    fn get_groupInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<GroupInfo> {
        &self.groupInfo
    }

    fn mut_groupInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GroupInfo> {
        &mut self.groupInfo
    }
}

impl ::protobuf::Message for SetMaster {
    fn is_initialized(&self) -> bool {
        if self.groupInfo.is_none() {
            return false;
        }
        for v in &self.groupInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.groupInfo)?;
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
        if let Some(ref v) = self.groupInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.groupInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SetMaster {
    fn new() -> SetMaster {
        SetMaster::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetMaster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GroupInfo>>(
                    "groupInfo",
                    SetMaster::get_groupInfo_for_reflect,
                    SetMaster::mut_groupInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetMaster>(
                    "SetMaster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetMaster {
    fn clear(&mut self) {
        self.clear_groupInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetMaster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetMaster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewSlave {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewSlave {}

impl NewSlave {
    pub fn new() -> NewSlave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewSlave {
        static mut instance: ::protobuf::lazy::Lazy<NewSlave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewSlave,
        };
        unsafe {
            instance.get(NewSlave::new)
        }
    }
}

impl ::protobuf::Message for NewSlave {
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

impl ::protobuf::MessageStatic for NewSlave {
    fn new() -> NewSlave {
        NewSlave::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewSlave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<NewSlave>(
                    "NewSlave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewSlave {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewSlave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewSlave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Unpair {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unpair {}

impl Unpair {
    pub fn new() -> Unpair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unpair {
        static mut instance: ::protobuf::lazy::Lazy<Unpair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unpair,
        };
        unsafe {
            instance.get(Unpair::new)
        }
    }
}

impl ::protobuf::Message for Unpair {
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

impl ::protobuf::MessageStatic for Unpair {
    fn new() -> Unpair {
        Unpair::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unpair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Unpair>(
                    "Unpair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unpair {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Unpair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Unpair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RecordPose {
    // message fields
    index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecordPose {}

impl RecordPose {
    pub fn new() -> RecordPose {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecordPose {
        static mut instance: ::protobuf::lazy::Lazy<RecordPose> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecordPose,
        };
        unsafe {
            instance.get(RecordPose::new)
        }
    }

    // optional uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }
}

impl ::protobuf::Message for RecordPose {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for RecordPose {
    fn new() -> RecordPose {
        RecordPose::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecordPose>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    RecordPose::get_index_for_reflect,
                    RecordPose::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecordPose>(
                    "RecordPose",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecordPose {
    fn clear(&mut self) {
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RecordPose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RecordPose {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GotoPose {
    // message fields
    index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GotoPose {}

impl GotoPose {
    pub fn new() -> GotoPose {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GotoPose {
        static mut instance: ::protobuf::lazy::Lazy<GotoPose> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GotoPose,
        };
        unsafe {
            instance.get(GotoPose::new)
        }
    }

    // required uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }
}

impl ::protobuf::Message for GotoPose {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for GotoPose {
    fn new() -> GotoPose {
        GotoPose::new()
    }

    fn descriptor_static(_: ::std::option::Option<GotoPose>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    GotoPose::get_index_for_reflect,
                    GotoPose::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GotoPose>(
                    "GotoPose",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GotoPose {
    fn clear(&mut self) {
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GotoPose {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GotoPose {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoseReached {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoseReached {}

impl PoseReached {
    pub fn new() -> PoseReached {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoseReached {
        static mut instance: ::protobuf::lazy::Lazy<PoseReached> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoseReached,
        };
        unsafe {
            instance.get(PoseReached::new)
        }
    }
}

impl ::protobuf::Message for PoseReached {
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

impl ::protobuf::MessageStatic for PoseReached {
    fn new() -> PoseReached {
        PoseReached::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoseReached>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PoseReached>(
                    "PoseReached",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoseReached {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoseReached {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoseReached {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayPoses {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayPoses {}

impl PlayPoses {
    pub fn new() -> PlayPoses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayPoses {
        static mut instance: ::protobuf::lazy::Lazy<PlayPoses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayPoses,
        };
        unsafe {
            instance.get(PlayPoses::new)
        }
    }
}

impl ::protobuf::Message for PlayPoses {
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

impl ::protobuf::MessageStatic for PlayPoses {
    fn new() -> PlayPoses {
        PlayPoses::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayPoses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PlayPoses>(
                    "PlayPoses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayPoses {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayPoses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayPoses {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClearPoses {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClearPoses {}

impl ClearPoses {
    pub fn new() -> ClearPoses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClearPoses {
        static mut instance: ::protobuf::lazy::Lazy<ClearPoses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClearPoses,
        };
        unsafe {
            instance.get(ClearPoses::new)
        }
    }
}

impl ::protobuf::Message for ClearPoses {
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

impl ::protobuf::MessageStatic for ClearPoses {
    fn new() -> ClearPoses {
        ClearPoses::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClearPoses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ClearPoses>(
                    "ClearPoses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClearPoses {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClearPoses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClearPoses {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LedColor {
    // message fields
    color: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LedColor {}

impl LedColor {
    pub fn new() -> LedColor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LedColor {
        static mut instance: ::protobuf::lazy::Lazy<LedColor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LedColor,
        };
        unsafe {
            instance.get(LedColor::new)
        }
    }

    // required uint32 color = 1;

    pub fn clear_color(&mut self) {
        self.color = ::std::option::Option::None;
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: u32) {
        self.color = ::std::option::Option::Some(v);
    }

    pub fn get_color(&self) -> u32 {
        self.color.unwrap_or(0)
    }

    fn get_color_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.color
    }
}

impl ::protobuf::Message for LedColor {
    fn is_initialized(&self) -> bool {
        if self.color.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.color = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.color {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.color {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for LedColor {
    fn new() -> LedColor {
        LedColor::new()
    }

    fn descriptor_static(_: ::std::option::Option<LedColor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "color",
                    LedColor::get_color_for_reflect,
                    LedColor::mut_color_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LedColor>(
                    "LedColor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LedColor {
    fn clear(&mut self) {
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LedColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LedColor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RobotToRobot {
    // message oneof groups
    arg: ::std::option::Option<RobotToRobot_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RobotToRobot {}

#[derive(Clone,PartialEq)]
pub enum RobotToRobot_oneof_arg {
    arbitrateMaster(ArbitrateMaster),
    setMotorPower(SetMotorPower),
    setJointPos(SetJointPos),
    setSlave(SetSlave),
    setMaster(SetMaster),
    newSlave(NewSlave),
    unpair(Unpair),
    recordPose(RecordPose),
    gotoPose(GotoPose),
    poseReached(PoseReached),
    playPoses(PlayPoses),
    clearPoses(ClearPoses),
    ledColor(LedColor),
}

impl RobotToRobot {
    pub fn new() -> RobotToRobot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RobotToRobot {
        static mut instance: ::protobuf::lazy::Lazy<RobotToRobot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RobotToRobot,
        };
        unsafe {
            instance.get(RobotToRobot::new)
        }
    }

    // optional .linkbot.bumpconnect.ArbitrateMaster arbitrateMaster = 1;

    pub fn clear_arbitrateMaster(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_arbitrateMaster(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_arbitrateMaster(&mut self, v: ArbitrateMaster) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_arbitrateMaster(&mut self) -> &mut ArbitrateMaster {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(ArbitrateMaster::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_arbitrateMaster(&mut self) -> ArbitrateMaster {
        if self.has_arbitrateMaster() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(v)) => v,
                _ => panic!(),
            }
        } else {
            ArbitrateMaster::new()
        }
    }

    pub fn get_arbitrateMaster(&self) -> &ArbitrateMaster {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(ref v)) => v,
            _ => ArbitrateMaster::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.SetMotorPower setMotorPower = 2;

    pub fn clear_setMotorPower(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setMotorPower(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setMotorPower(&mut self, v: SetMotorPower) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setMotorPower(&mut self) -> &mut SetMotorPower {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(SetMotorPower::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setMotorPower(&mut self) -> SetMotorPower {
        if self.has_setMotorPower() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(v)) => v,
                _ => panic!(),
            }
        } else {
            SetMotorPower::new()
        }
    }

    pub fn get_setMotorPower(&self) -> &SetMotorPower {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(ref v)) => v,
            _ => SetMotorPower::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.SetJointPos setJointPos = 3;

    pub fn clear_setJointPos(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setJointPos(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setJointPos(&mut self, v: SetJointPos) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setJointPos(&mut self) -> &mut SetJointPos {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(SetJointPos::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setJointPos(&mut self) -> SetJointPos {
        if self.has_setJointPos() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(v)) => v,
                _ => panic!(),
            }
        } else {
            SetJointPos::new()
        }
    }

    pub fn get_setJointPos(&self) -> &SetJointPos {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(ref v)) => v,
            _ => SetJointPos::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.SetSlave setSlave = 4;

    pub fn clear_setSlave(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setSlave(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setSlave(&mut self, v: SetSlave) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setSlave(&mut self) -> &mut SetSlave {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(SetSlave::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setSlave(&mut self) -> SetSlave {
        if self.has_setSlave() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(v)) => v,
                _ => panic!(),
            }
        } else {
            SetSlave::new()
        }
    }

    pub fn get_setSlave(&self) -> &SetSlave {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(ref v)) => v,
            _ => SetSlave::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.SetMaster setMaster = 5;

    pub fn clear_setMaster(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setMaster(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setMaster(&mut self, v: SetMaster) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setMaster(&mut self) -> &mut SetMaster {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(SetMaster::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setMaster(&mut self) -> SetMaster {
        if self.has_setMaster() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(v)) => v,
                _ => panic!(),
            }
        } else {
            SetMaster::new()
        }
    }

    pub fn get_setMaster(&self) -> &SetMaster {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(ref v)) => v,
            _ => SetMaster::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.NewSlave newSlave = 6;

    pub fn clear_newSlave(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_newSlave(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_newSlave(&mut self, v: NewSlave) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(v))
    }

    // Mutable pointer to the field.
    pub fn mut_newSlave(&mut self) -> &mut NewSlave {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(NewSlave::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_newSlave(&mut self) -> NewSlave {
        if self.has_newSlave() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(v)) => v,
                _ => panic!(),
            }
        } else {
            NewSlave::new()
        }
    }

    pub fn get_newSlave(&self) -> &NewSlave {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(ref v)) => v,
            _ => NewSlave::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.Unpair unpair = 7;

    pub fn clear_unpair(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_unpair(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unpair(&mut self, v: Unpair) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unpair(&mut self) -> &mut Unpair {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(Unpair::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unpair(&mut self) -> Unpair {
        if self.has_unpair() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(v)) => v,
                _ => panic!(),
            }
        } else {
            Unpair::new()
        }
    }

    pub fn get_unpair(&self) -> &Unpair {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(ref v)) => v,
            _ => Unpair::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.RecordPose recordPose = 8;

    pub fn clear_recordPose(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_recordPose(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_recordPose(&mut self, v: RecordPose) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(v))
    }

    // Mutable pointer to the field.
    pub fn mut_recordPose(&mut self) -> &mut RecordPose {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(RecordPose::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_recordPose(&mut self) -> RecordPose {
        if self.has_recordPose() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(v)) => v,
                _ => panic!(),
            }
        } else {
            RecordPose::new()
        }
    }

    pub fn get_recordPose(&self) -> &RecordPose {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(ref v)) => v,
            _ => RecordPose::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.GotoPose gotoPose = 9;

    pub fn clear_gotoPose(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_gotoPose(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_gotoPose(&mut self, v: GotoPose) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(v))
    }

    // Mutable pointer to the field.
    pub fn mut_gotoPose(&mut self) -> &mut GotoPose {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(GotoPose::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_gotoPose(&mut self) -> GotoPose {
        if self.has_gotoPose() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(v)) => v,
                _ => panic!(),
            }
        } else {
            GotoPose::new()
        }
    }

    pub fn get_gotoPose(&self) -> &GotoPose {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(ref v)) => v,
            _ => GotoPose::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.PoseReached poseReached = 10;

    pub fn clear_poseReached(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_poseReached(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_poseReached(&mut self, v: PoseReached) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(v))
    }

    // Mutable pointer to the field.
    pub fn mut_poseReached(&mut self) -> &mut PoseReached {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(PoseReached::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_poseReached(&mut self) -> PoseReached {
        if self.has_poseReached() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(v)) => v,
                _ => panic!(),
            }
        } else {
            PoseReached::new()
        }
    }

    pub fn get_poseReached(&self) -> &PoseReached {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(ref v)) => v,
            _ => PoseReached::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.PlayPoses playPoses = 11;

    pub fn clear_playPoses(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_playPoses(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_playPoses(&mut self, v: PlayPoses) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(v))
    }

    // Mutable pointer to the field.
    pub fn mut_playPoses(&mut self) -> &mut PlayPoses {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(PlayPoses::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_playPoses(&mut self) -> PlayPoses {
        if self.has_playPoses() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(v)) => v,
                _ => panic!(),
            }
        } else {
            PlayPoses::new()
        }
    }

    pub fn get_playPoses(&self) -> &PlayPoses {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(ref v)) => v,
            _ => PlayPoses::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.ClearPoses clearPoses = 12;

    pub fn clear_clearPoses(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_clearPoses(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_clearPoses(&mut self, v: ClearPoses) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(v))
    }

    // Mutable pointer to the field.
    pub fn mut_clearPoses(&mut self) -> &mut ClearPoses {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(ClearPoses::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_clearPoses(&mut self) -> ClearPoses {
        if self.has_clearPoses() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(v)) => v,
                _ => panic!(),
            }
        } else {
            ClearPoses::new()
        }
    }

    pub fn get_clearPoses(&self) -> &ClearPoses {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(ref v)) => v,
            _ => ClearPoses::default_instance(),
        }
    }

    // optional .linkbot.bumpconnect.LedColor ledColor = 13;

    pub fn clear_ledColor(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_ledColor(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ledColor(&mut self, v: LedColor) {
        self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ledColor(&mut self) -> &mut LedColor {
        if let ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(LedColor::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ledColor(&mut self) -> LedColor {
        if self.has_ledColor() {
            match self.arg.take() {
                ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(v)) => v,
                _ => panic!(),
            }
        } else {
            LedColor::new()
        }
    }

    pub fn get_ledColor(&self) -> &LedColor {
        match self.arg {
            ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(ref v)) => v,
            _ => LedColor::default_instance(),
        }
    }
}

impl ::protobuf::Message for RobotToRobot {
    fn is_initialized(&self) -> bool {
        if let Some(RobotToRobot_oneof_arg::arbitrateMaster(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::setMotorPower(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::setJointPos(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::setSlave(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::setMaster(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::newSlave(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::unpair(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::recordPose(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::gotoPose(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::poseReached(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::playPoses(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::clearPoses(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RobotToRobot_oneof_arg::ledColor(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::arbitrateMaster(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMotorPower(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setJointPos(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setSlave(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::setMaster(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::newSlave(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::unpair(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::recordPose(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::gotoPose(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::poseReached(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::playPoses(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::clearPoses(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RobotToRobot_oneof_arg::ledColor(is.read_message()?));
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
                &RobotToRobot_oneof_arg::arbitrateMaster(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::setMotorPower(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::setJointPos(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::setSlave(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::setMaster(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::newSlave(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::unpair(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::recordPose(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::gotoPose(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::poseReached(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::playPoses(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::clearPoses(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RobotToRobot_oneof_arg::ledColor(ref v) => {
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
                &RobotToRobot_oneof_arg::arbitrateMaster(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::setMotorPower(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::setJointPos(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::setSlave(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::setMaster(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::newSlave(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::unpair(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::recordPose(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::gotoPose(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::poseReached(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::playPoses(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::clearPoses(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RobotToRobot_oneof_arg::ledColor(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RobotToRobot {
    fn new() -> RobotToRobot {
        RobotToRobot::new()
    }

    fn descriptor_static(_: ::std::option::Option<RobotToRobot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ArbitrateMaster>(
                    "arbitrateMaster",
                    RobotToRobot::has_arbitrateMaster,
                    RobotToRobot::get_arbitrateMaster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetMotorPower>(
                    "setMotorPower",
                    RobotToRobot::has_setMotorPower,
                    RobotToRobot::get_setMotorPower,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetJointPos>(
                    "setJointPos",
                    RobotToRobot::has_setJointPos,
                    RobotToRobot::get_setJointPos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetSlave>(
                    "setSlave",
                    RobotToRobot::has_setSlave,
                    RobotToRobot::get_setSlave,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SetMaster>(
                    "setMaster",
                    RobotToRobot::has_setMaster,
                    RobotToRobot::get_setMaster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, NewSlave>(
                    "newSlave",
                    RobotToRobot::has_newSlave,
                    RobotToRobot::get_newSlave,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Unpair>(
                    "unpair",
                    RobotToRobot::has_unpair,
                    RobotToRobot::get_unpair,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RecordPose>(
                    "recordPose",
                    RobotToRobot::has_recordPose,
                    RobotToRobot::get_recordPose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, GotoPose>(
                    "gotoPose",
                    RobotToRobot::has_gotoPose,
                    RobotToRobot::get_gotoPose,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PoseReached>(
                    "poseReached",
                    RobotToRobot::has_poseReached,
                    RobotToRobot::get_poseReached,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, PlayPoses>(
                    "playPoses",
                    RobotToRobot::has_playPoses,
                    RobotToRobot::get_playPoses,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ClearPoses>(
                    "clearPoses",
                    RobotToRobot::has_clearPoses,
                    RobotToRobot::get_clearPoses,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LedColor>(
                    "ledColor",
                    RobotToRobot::has_ledColor,
                    RobotToRobot::get_ledColor,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RobotToRobot>(
                    "RobotToRobot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RobotToRobot {
    fn clear(&mut self) {
        self.clear_arbitrateMaster();
        self.clear_setMotorPower();
        self.clear_setJointPos();
        self.clear_setSlave();
        self.clear_setMaster();
        self.clear_newSlave();
        self.clear_unpair();
        self.clear_recordPose();
        self.clear_gotoPose();
        self.clear_poseReached();
        self.clear_playPoses();
        self.clear_clearPoses();
        self.clear_ledColor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RobotToRobot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RobotToRobot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11bumpconnect.proto\x12\x13linkbot.bumpconnect\x1a\x11commontypes.pr\
    oto\"k\n\x0fArbitrateMaster\x12*\n\x10millisSinceShock\x18\x01\x20\x02(\
    \rR\x10millisSinceShock\x12,\n\x11millisSinceButton\x18\x02\x20\x02(\rR\
    \x11millisSinceButton\"?\n\rSetMotorPower\x12\x0e\n\x02m1\x18\x01\x20\
    \x02(\x05R\x02m1\x12\x0e\n\x02m2\x18\x02\x20\x02(\x05R\x02m2\x12\x0e\n\
    \x02m3\x18\x03\x20\x02(\x05R\x02m3\"=\n\x0bSetJointPos\x12\x0e\n\x02m1\
    \x18\x01\x20\x02(\x02R\x02m1\x12\x0e\n\x02m2\x18\x02\x20\x02(\x02R\x02m2\
    \x12\x0e\n\x02m3\x18\x03\x20\x02(\x02R\x02m3\"f\n\tGroupInfo\x12\x18\n\
    \x07groupId\x18\x01\x20\x02(\rR\x07groupId\x12\x10\n\x03rgb\x18\x02\x20\
    \x02(\rR\x03rgb\x12-\n\x08masterId\x18\x03\x20\x01(\x0b2\x11.linkbot.Ser\
    ialIdR\x08masterId\"H\n\x08SetSlave\x12<\n\tgroupInfo\x18\x01\x20\x02(\
    \x0b2\x1e.linkbot.bumpconnect.GroupInfoR\tgroupInfo\"I\n\tSetMaster\x12<\
    \n\tgroupInfo\x18\x01\x20\x02(\x0b2\x1e.linkbot.bumpconnect.GroupInfoR\t\
    groupInfo\"\n\n\x08NewSlave\"\x08\n\x06Unpair\"\"\n\nRecordPose\x12\x14\
    \n\x05index\x18\x01\x20\x01(\rR\x05index\"\x20\n\x08GotoPose\x12\x14\n\
    \x05index\x18\x01\x20\x02(\rR\x05index\"\r\n\x0bPoseReached\"\x0b\n\tPla\
    yPoses\"\x0c\n\nClearPoses\"\x20\n\x08LedColor\x12\x14\n\x05color\x18\
    \x01\x20\x02(\rR\x05color\"\xf0\x06\n\x0cRobotToRobot\x12P\n\x0farbitrat\
    eMaster\x18\x01\x20\x01(\x0b2$.linkbot.bumpconnect.ArbitrateMasterH\0R\
    \x0farbitrateMaster\x12J\n\rsetMotorPower\x18\x02\x20\x01(\x0b2\".linkbo\
    t.bumpconnect.SetMotorPowerH\0R\rsetMotorPower\x12D\n\x0bsetJointPos\x18\
    \x03\x20\x01(\x0b2\x20.linkbot.bumpconnect.SetJointPosH\0R\x0bsetJointPo\
    s\x12;\n\x08setSlave\x18\x04\x20\x01(\x0b2\x1d.linkbot.bumpconnect.SetSl\
    aveH\0R\x08setSlave\x12>\n\tsetMaster\x18\x05\x20\x01(\x0b2\x1e.linkbot.\
    bumpconnect.SetMasterH\0R\tsetMaster\x12;\n\x08newSlave\x18\x06\x20\x01(\
    \x0b2\x1d.linkbot.bumpconnect.NewSlaveH\0R\x08newSlave\x125\n\x06unpair\
    \x18\x07\x20\x01(\x0b2\x1b.linkbot.bumpconnect.UnpairH\0R\x06unpair\x12A\
    \n\nrecordPose\x18\x08\x20\x01(\x0b2\x1f.linkbot.bumpconnect.RecordPoseH\
    \0R\nrecordPose\x12;\n\x08gotoPose\x18\t\x20\x01(\x0b2\x1d.linkbot.bumpc\
    onnect.GotoPoseH\0R\x08gotoPose\x12D\n\x0bposeReached\x18\n\x20\x01(\x0b\
    2\x20.linkbot.bumpconnect.PoseReachedH\0R\x0bposeReached\x12>\n\tplayPos\
    es\x18\x0b\x20\x01(\x0b2\x1e.linkbot.bumpconnect.PlayPosesH\0R\tplayPose\
    s\x12A\n\nclearPoses\x18\x0c\x20\x01(\x0b2\x1f.linkbot.bumpconnect.Clear\
    PosesH\0R\nclearPoses\x12;\n\x08ledColor\x18\r\x20\x01(\x0b2\x1d.linkbot\
    .bumpconnect.LedColorH\0R\x08ledColorB\x05\n\x03arg\
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
