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
pub struct getDaemonVersionString {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDaemonVersionString {}

impl getDaemonVersionString {
    pub fn new() -> getDaemonVersionString {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDaemonVersionString {
        static mut instance: ::protobuf::lazy::Lazy<getDaemonVersionString> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDaemonVersionString,
        };
        unsafe {
            instance.get(getDaemonVersionString::new)
        }
    }
}

impl ::protobuf::Message for getDaemonVersionString {
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

impl ::protobuf::MessageStatic for getDaemonVersionString {
    fn new() -> getDaemonVersionString {
        getDaemonVersionString::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDaemonVersionString>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getDaemonVersionString>(
                    "getDaemonVersionString",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDaemonVersionString {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDaemonVersionString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDaemonVersionString {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getDaemonVersionString_In {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDaemonVersionString_In {}

impl getDaemonVersionString_In {
    pub fn new() -> getDaemonVersionString_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDaemonVersionString_In {
        static mut instance: ::protobuf::lazy::Lazy<getDaemonVersionString_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDaemonVersionString_In,
        };
        unsafe {
            instance.get(getDaemonVersionString_In::new)
        }
    }
}

impl ::protobuf::Message for getDaemonVersionString_In {
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

impl ::protobuf::MessageStatic for getDaemonVersionString_In {
    fn new() -> getDaemonVersionString_In {
        getDaemonVersionString_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDaemonVersionString_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getDaemonVersionString_In>(
                    "getDaemonVersionString_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDaemonVersionString_In {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDaemonVersionString_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDaemonVersionString_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getDaemonVersionString_Out {
    // message fields
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDaemonVersionString_Out {}

impl getDaemonVersionString_Out {
    pub fn new() -> getDaemonVersionString_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDaemonVersionString_Out {
        static mut instance: ::protobuf::lazy::Lazy<getDaemonVersionString_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDaemonVersionString_Out,
        };
        unsafe {
            instance.get(getDaemonVersionString_Out::new)
        }
    }

    // optional string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for getDaemonVersionString_Out {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for getDaemonVersionString_Out {
    fn new() -> getDaemonVersionString_Out {
        getDaemonVersionString_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDaemonVersionString_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    getDaemonVersionString_Out::get_value_for_reflect,
                    getDaemonVersionString_Out::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<getDaemonVersionString_Out>(
                    "getDaemonVersionString_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDaemonVersionString_Out {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDaemonVersionString_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDaemonVersionString_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getDongleCount {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDongleCount {}

impl getDongleCount {
    pub fn new() -> getDongleCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDongleCount {
        static mut instance: ::protobuf::lazy::Lazy<getDongleCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDongleCount,
        };
        unsafe {
            instance.get(getDongleCount::new)
        }
    }
}

impl ::protobuf::Message for getDongleCount {
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

impl ::protobuf::MessageStatic for getDongleCount {
    fn new() -> getDongleCount {
        getDongleCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDongleCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getDongleCount>(
                    "getDongleCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDongleCount {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDongleCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDongleCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getDongleCount_In {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDongleCount_In {}

impl getDongleCount_In {
    pub fn new() -> getDongleCount_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDongleCount_In {
        static mut instance: ::protobuf::lazy::Lazy<getDongleCount_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDongleCount_In,
        };
        unsafe {
            instance.get(getDongleCount_In::new)
        }
    }
}

impl ::protobuf::Message for getDongleCount_In {
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

impl ::protobuf::MessageStatic for getDongleCount_In {
    fn new() -> getDongleCount_In {
        getDongleCount_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDongleCount_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getDongleCount_In>(
                    "getDongleCount_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDongleCount_In {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDongleCount_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDongleCount_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getDongleCount_Out {
    // message fields
    dongleCount: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getDongleCount_Out {}

impl getDongleCount_Out {
    pub fn new() -> getDongleCount_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getDongleCount_Out {
        static mut instance: ::protobuf::lazy::Lazy<getDongleCount_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getDongleCount_Out,
        };
        unsafe {
            instance.get(getDongleCount_Out::new)
        }
    }

    // optional uint32 dongleCount = 1;

    pub fn clear_dongleCount(&mut self) {
        self.dongleCount = ::std::option::Option::None;
    }

    pub fn has_dongleCount(&self) -> bool {
        self.dongleCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dongleCount(&mut self, v: u32) {
        self.dongleCount = ::std::option::Option::Some(v);
    }

    pub fn get_dongleCount(&self) -> u32 {
        self.dongleCount.unwrap_or(0)
    }

    fn get_dongleCount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dongleCount
    }

    fn mut_dongleCount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dongleCount
    }
}

impl ::protobuf::Message for getDongleCount_Out {
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
                    self.dongleCount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dongleCount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dongleCount {
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

impl ::protobuf::MessageStatic for getDongleCount_Out {
    fn new() -> getDongleCount_Out {
        getDongleCount_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<getDongleCount_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dongleCount",
                    getDongleCount_Out::get_dongleCount_for_reflect,
                    getDongleCount_Out::mut_dongleCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<getDongleCount_Out>(
                    "getDongleCount_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getDongleCount_Out {
    fn clear(&mut self) {
        self.clear_dongleCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getDongleCount_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getDongleCount_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct addRobotRefs {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for addRobotRefs {}

impl addRobotRefs {
    pub fn new() -> addRobotRefs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static addRobotRefs {
        static mut instance: ::protobuf::lazy::Lazy<addRobotRefs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const addRobotRefs,
        };
        unsafe {
            instance.get(addRobotRefs::new)
        }
    }
}

impl ::protobuf::Message for addRobotRefs {
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

impl ::protobuf::MessageStatic for addRobotRefs {
    fn new() -> addRobotRefs {
        addRobotRefs::new()
    }

    fn descriptor_static(_: ::std::option::Option<addRobotRefs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<addRobotRefs>(
                    "addRobotRefs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for addRobotRefs {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for addRobotRefs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for addRobotRefs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct addRobotRefs_In {
    // message fields
    serialIds: ::protobuf::RepeatedField<super::commontypes::SerialId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for addRobotRefs_In {}

impl addRobotRefs_In {
    pub fn new() -> addRobotRefs_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static addRobotRefs_In {
        static mut instance: ::protobuf::lazy::Lazy<addRobotRefs_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const addRobotRefs_In,
        };
        unsafe {
            instance.get(addRobotRefs_In::new)
        }
    }

    // repeated .linkbot.SerialId serialIds = 1;

    pub fn clear_serialIds(&mut self) {
        self.serialIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_serialIds(&mut self, v: ::protobuf::RepeatedField<super::commontypes::SerialId>) {
        self.serialIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_serialIds(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.serialIds
    }

    // Take field
    pub fn take_serialIds(&mut self) -> ::protobuf::RepeatedField<super::commontypes::SerialId> {
        ::std::mem::replace(&mut self.serialIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_serialIds(&self) -> &[super::commontypes::SerialId] {
        &self.serialIds
    }

    fn get_serialIds_for_reflect(&self) -> &::protobuf::RepeatedField<super::commontypes::SerialId> {
        &self.serialIds
    }

    fn mut_serialIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.serialIds
    }
}

impl ::protobuf::Message for addRobotRefs_In {
    fn is_initialized(&self) -> bool {
        for v in &self.serialIds {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.serialIds)?;
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
        for value in &self.serialIds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.serialIds {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for addRobotRefs_In {
    fn new() -> addRobotRefs_In {
        addRobotRefs_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<addRobotRefs_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "serialIds",
                    addRobotRefs_In::get_serialIds_for_reflect,
                    addRobotRefs_In::mut_serialIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<addRobotRefs_In>(
                    "addRobotRefs_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for addRobotRefs_In {
    fn clear(&mut self) {
        self.clear_serialIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for addRobotRefs_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for addRobotRefs_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct addRobotRefs_Out {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for addRobotRefs_Out {}

impl addRobotRefs_Out {
    pub fn new() -> addRobotRefs_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static addRobotRefs_Out {
        static mut instance: ::protobuf::lazy::Lazy<addRobotRefs_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const addRobotRefs_Out,
        };
        unsafe {
            instance.get(addRobotRefs_Out::new)
        }
    }

    // optional .linkbot.daemon.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::OK)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for addRobotRefs_Out {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for addRobotRefs_Out {
    fn new() -> addRobotRefs_Out {
        addRobotRefs_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<addRobotRefs_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    addRobotRefs_Out::get_status_for_reflect,
                    addRobotRefs_Out::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<addRobotRefs_Out>(
                    "addRobotRefs_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for addRobotRefs_Out {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for addRobotRefs_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for addRobotRefs_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct acquireRobotRef {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for acquireRobotRef {}

impl acquireRobotRef {
    pub fn new() -> acquireRobotRef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static acquireRobotRef {
        static mut instance: ::protobuf::lazy::Lazy<acquireRobotRef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const acquireRobotRef,
        };
        unsafe {
            instance.get(acquireRobotRef::new)
        }
    }
}

impl ::protobuf::Message for acquireRobotRef {
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

impl ::protobuf::MessageStatic for acquireRobotRef {
    fn new() -> acquireRobotRef {
        acquireRobotRef::new()
    }

    fn descriptor_static(_: ::std::option::Option<acquireRobotRef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<acquireRobotRef>(
                    "acquireRobotRef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for acquireRobotRef {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for acquireRobotRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for acquireRobotRef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct acquireRobotRef_In {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for acquireRobotRef_In {}

impl acquireRobotRef_In {
    pub fn new() -> acquireRobotRef_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static acquireRobotRef_In {
        static mut instance: ::protobuf::lazy::Lazy<acquireRobotRef_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const acquireRobotRef_In,
        };
        unsafe {
            instance.get(acquireRobotRef_In::new)
        }
    }
}

impl ::protobuf::Message for acquireRobotRef_In {
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

impl ::protobuf::MessageStatic for acquireRobotRef_In {
    fn new() -> acquireRobotRef_In {
        acquireRobotRef_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<acquireRobotRef_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<acquireRobotRef_In>(
                    "acquireRobotRef_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for acquireRobotRef_In {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for acquireRobotRef_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for acquireRobotRef_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct acquireRobotRef_Out {
    // message fields
    serialId: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for acquireRobotRef_Out {}

impl acquireRobotRef_Out {
    pub fn new() -> acquireRobotRef_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static acquireRobotRef_Out {
        static mut instance: ::protobuf::lazy::Lazy<acquireRobotRef_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const acquireRobotRef_Out,
        };
        unsafe {
            instance.get(acquireRobotRef_Out::new)
        }
    }

    // optional .linkbot.SerialId serialId = 1;

    pub fn clear_serialId(&mut self) {
        self.serialId.clear();
    }

    pub fn has_serialId(&self) -> bool {
        self.serialId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialId(&mut self, v: super::commontypes::SerialId) {
        self.serialId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialId(&mut self) -> &mut super::commontypes::SerialId {
        if self.serialId.is_none() {
            self.serialId.set_default();
        }
        self.serialId.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialId(&mut self) -> super::commontypes::SerialId {
        self.serialId.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_serialId(&self) -> &super::commontypes::SerialId {
        self.serialId.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_serialId_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.serialId
    }

    fn mut_serialId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.serialId
    }
}

impl ::protobuf::Message for acquireRobotRef_Out {
    fn is_initialized(&self) -> bool {
        for v in &self.serialId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.serialId)?;
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
        if let Some(ref v) = self.serialId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.serialId.as_ref() {
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

impl ::protobuf::MessageStatic for acquireRobotRef_Out {
    fn new() -> acquireRobotRef_Out {
        acquireRobotRef_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<acquireRobotRef_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "serialId",
                    acquireRobotRef_Out::get_serialId_for_reflect,
                    acquireRobotRef_Out::mut_serialId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<acquireRobotRef_Out>(
                    "acquireRobotRef_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for acquireRobotRef_Out {
    fn clear(&mut self) {
        self.clear_serialId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for acquireRobotRef_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for acquireRobotRef_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct releaseRobotRefs {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for releaseRobotRefs {}

impl releaseRobotRefs {
    pub fn new() -> releaseRobotRefs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static releaseRobotRefs {
        static mut instance: ::protobuf::lazy::Lazy<releaseRobotRefs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const releaseRobotRefs,
        };
        unsafe {
            instance.get(releaseRobotRefs::new)
        }
    }
}

impl ::protobuf::Message for releaseRobotRefs {
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

impl ::protobuf::MessageStatic for releaseRobotRefs {
    fn new() -> releaseRobotRefs {
        releaseRobotRefs::new()
    }

    fn descriptor_static(_: ::std::option::Option<releaseRobotRefs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<releaseRobotRefs>(
                    "releaseRobotRefs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for releaseRobotRefs {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for releaseRobotRefs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for releaseRobotRefs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct releaseRobotRefs_In {
    // message fields
    serialIds: ::protobuf::RepeatedField<super::commontypes::SerialId>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for releaseRobotRefs_In {}

impl releaseRobotRefs_In {
    pub fn new() -> releaseRobotRefs_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static releaseRobotRefs_In {
        static mut instance: ::protobuf::lazy::Lazy<releaseRobotRefs_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const releaseRobotRefs_In,
        };
        unsafe {
            instance.get(releaseRobotRefs_In::new)
        }
    }

    // repeated .linkbot.SerialId serialIds = 1;

    pub fn clear_serialIds(&mut self) {
        self.serialIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_serialIds(&mut self, v: ::protobuf::RepeatedField<super::commontypes::SerialId>) {
        self.serialIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_serialIds(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.serialIds
    }

    // Take field
    pub fn take_serialIds(&mut self) -> ::protobuf::RepeatedField<super::commontypes::SerialId> {
        ::std::mem::replace(&mut self.serialIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_serialIds(&self) -> &[super::commontypes::SerialId] {
        &self.serialIds
    }

    fn get_serialIds_for_reflect(&self) -> &::protobuf::RepeatedField<super::commontypes::SerialId> {
        &self.serialIds
    }

    fn mut_serialIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::commontypes::SerialId> {
        &mut self.serialIds
    }
}

impl ::protobuf::Message for releaseRobotRefs_In {
    fn is_initialized(&self) -> bool {
        for v in &self.serialIds {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.serialIds)?;
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
        for value in &self.serialIds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.serialIds {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for releaseRobotRefs_In {
    fn new() -> releaseRobotRefs_In {
        releaseRobotRefs_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<releaseRobotRefs_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "serialIds",
                    releaseRobotRefs_In::get_serialIds_for_reflect,
                    releaseRobotRefs_In::mut_serialIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<releaseRobotRefs_In>(
                    "releaseRobotRefs_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for releaseRobotRefs_In {
    fn clear(&mut self) {
        self.clear_serialIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for releaseRobotRefs_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for releaseRobotRefs_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct releaseRobotRefs_Out {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for releaseRobotRefs_Out {}

impl releaseRobotRefs_Out {
    pub fn new() -> releaseRobotRefs_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static releaseRobotRefs_Out {
        static mut instance: ::protobuf::lazy::Lazy<releaseRobotRefs_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const releaseRobotRefs_Out,
        };
        unsafe {
            instance.get(releaseRobotRefs_Out::new)
        }
    }

    // optional .linkbot.daemon.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::OK)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for releaseRobotRefs_Out {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for releaseRobotRefs_Out {
    fn new() -> releaseRobotRefs_Out {
        releaseRobotRefs_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<releaseRobotRefs_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    releaseRobotRefs_Out::get_status_for_reflect,
                    releaseRobotRefs_Out::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<releaseRobotRefs_Out>(
                    "releaseRobotRefs_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for releaseRobotRefs_Out {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for releaseRobotRefs_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for releaseRobotRefs_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmit {}

impl transmit {
    pub fn new() -> transmit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmit {
        static mut instance: ::protobuf::lazy::Lazy<transmit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmit,
        };
        unsafe {
            instance.get(transmit::new)
        }
    }
}

impl ::protobuf::Message for transmit {
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

impl ::protobuf::MessageStatic for transmit {
    fn new() -> transmit {
        transmit::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<transmit>(
                    "transmit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmit_In {
    // message fields
    destination: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    payload: ::protobuf::SingularPtrField<super::robot::ClientToRobot>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmit_In {}

impl transmit_In {
    pub fn new() -> transmit_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmit_In {
        static mut instance: ::protobuf::lazy::Lazy<transmit_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmit_In,
        };
        unsafe {
            instance.get(transmit_In::new)
        }
    }

    // optional .linkbot.SerialId destination = 1;

    pub fn clear_destination(&mut self) {
        self.destination.clear();
    }

    pub fn has_destination(&self) -> bool {
        self.destination.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destination(&mut self, v: super::commontypes::SerialId) {
        self.destination = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination(&mut self) -> &mut super::commontypes::SerialId {
        if self.destination.is_none() {
            self.destination.set_default();
        }
        self.destination.as_mut().unwrap()
    }

    // Take field
    pub fn take_destination(&mut self) -> super::commontypes::SerialId {
        self.destination.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_destination(&self) -> &super::commontypes::SerialId {
        self.destination.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_destination_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.destination
    }

    fn mut_destination_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.destination
    }

    // optional .linkbot.robot.ClientToRobot payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: super::robot::ClientToRobot) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut super::robot::ClientToRobot {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> super::robot::ClientToRobot {
        self.payload.take().unwrap_or_else(|| super::robot::ClientToRobot::new())
    }

    pub fn get_payload(&self) -> &super::robot::ClientToRobot {
        self.payload.as_ref().unwrap_or_else(|| super::robot::ClientToRobot::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<super::robot::ClientToRobot> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::robot::ClientToRobot> {
        &mut self.payload
    }
}

impl ::protobuf::Message for transmit_In {
    fn is_initialized(&self) -> bool {
        for v in &self.destination {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destination)?;
                },
                2 => {
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
        if let Some(ref v) = self.destination.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.destination.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for transmit_In {
    fn new() -> transmit_In {
        transmit_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmit_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "destination",
                    transmit_In::get_destination_for_reflect,
                    transmit_In::mut_destination_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::robot::ClientToRobot>>(
                    "payload",
                    transmit_In::get_payload_for_reflect,
                    transmit_In::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<transmit_In>(
                    "transmit_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmit_In {
    fn clear(&mut self) {
        self.clear_destination();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmit_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmit_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmit_Out {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmit_Out {}

impl transmit_Out {
    pub fn new() -> transmit_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmit_Out {
        static mut instance: ::protobuf::lazy::Lazy<transmit_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmit_Out,
        };
        unsafe {
            instance.get(transmit_Out::new)
        }
    }

    // optional .linkbot.daemon.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::OK)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for transmit_Out {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for transmit_Out {
    fn new() -> transmit_Out {
        transmit_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmit_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    transmit_Out::get_status_for_reflect,
                    transmit_Out::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<transmit_Out>(
                    "transmit_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmit_Out {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmit_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmit_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmitBroadcast {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmitBroadcast {}

impl transmitBroadcast {
    pub fn new() -> transmitBroadcast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmitBroadcast {
        static mut instance: ::protobuf::lazy::Lazy<transmitBroadcast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmitBroadcast,
        };
        unsafe {
            instance.get(transmitBroadcast::new)
        }
    }
}

impl ::protobuf::Message for transmitBroadcast {
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

impl ::protobuf::MessageStatic for transmitBroadcast {
    fn new() -> transmitBroadcast {
        transmitBroadcast::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmitBroadcast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<transmitBroadcast>(
                    "transmitBroadcast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmitBroadcast {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmitBroadcast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmitBroadcast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmitBroadcast_In {
    // message fields
    broadcastMethod: ::std::option::Option<BroadcastMethod>,
    destinations: ::protobuf::RepeatedField<super::commontypes::SerialId>,
    payload: ::protobuf::SingularPtrField<super::robot::ClientToRobotBroadcast>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmitBroadcast_In {}

impl transmitBroadcast_In {
    pub fn new() -> transmitBroadcast_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmitBroadcast_In {
        static mut instance: ::protobuf::lazy::Lazy<transmitBroadcast_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmitBroadcast_In,
        };
        unsafe {
            instance.get(transmitBroadcast_In::new)
        }
    }

    // optional .linkbot.daemon.BroadcastMethod broadcastMethod = 1;

    pub fn clear_broadcastMethod(&mut self) {
        self.broadcastMethod = ::std::option::Option::None;
    }

    pub fn has_broadcastMethod(&self) -> bool {
        self.broadcastMethod.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcastMethod(&mut self, v: BroadcastMethod) {
        self.broadcastMethod = ::std::option::Option::Some(v);
    }

    pub fn get_broadcastMethod(&self) -> BroadcastMethod {
        self.broadcastMethod.unwrap_or(BroadcastMethod::BROADCAST)
    }

    fn get_broadcastMethod_for_reflect(&self) -> &::std::option::Option<BroadcastMethod> {
        &self.broadcastMethod
    }

    fn mut_broadcastMethod_for_reflect(&mut self) -> &mut ::std::option::Option<BroadcastMethod> {
        &mut self.broadcastMethod
    }

    // repeated .linkbot.SerialId destinations = 2;

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

    // optional .linkbot.robot.ClientToRobotBroadcast payload = 4;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: super::robot::ClientToRobotBroadcast) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut super::robot::ClientToRobotBroadcast {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> super::robot::ClientToRobotBroadcast {
        self.payload.take().unwrap_or_else(|| super::robot::ClientToRobotBroadcast::new())
    }

    pub fn get_payload(&self) -> &super::robot::ClientToRobotBroadcast {
        self.payload.as_ref().unwrap_or_else(|| super::robot::ClientToRobotBroadcast::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<super::robot::ClientToRobotBroadcast> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::robot::ClientToRobotBroadcast> {
        &mut self.payload
    }
}

impl ::protobuf::Message for transmitBroadcast_In {
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
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.broadcastMethod, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.destinations)?;
                },
                4 => {
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
        if let Some(v) = self.broadcastMethod {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        for value in &self.destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.payload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.broadcastMethod {
            os.write_enum(1, v.value())?;
        }
        for v in &self.destinations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for transmitBroadcast_In {
    fn new() -> transmitBroadcast_In {
        transmitBroadcast_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmitBroadcast_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<BroadcastMethod>>(
                    "broadcastMethod",
                    transmitBroadcast_In::get_broadcastMethod_for_reflect,
                    transmitBroadcast_In::mut_broadcastMethod_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "destinations",
                    transmitBroadcast_In::get_destinations_for_reflect,
                    transmitBroadcast_In::mut_destinations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::robot::ClientToRobotBroadcast>>(
                    "payload",
                    transmitBroadcast_In::get_payload_for_reflect,
                    transmitBroadcast_In::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<transmitBroadcast_In>(
                    "transmitBroadcast_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmitBroadcast_In {
    fn clear(&mut self) {
        self.clear_broadcastMethod();
        self.clear_destinations();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmitBroadcast_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmitBroadcast_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct transmitBroadcast_Out {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for transmitBroadcast_Out {}

impl transmitBroadcast_Out {
    pub fn new() -> transmitBroadcast_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static transmitBroadcast_Out {
        static mut instance: ::protobuf::lazy::Lazy<transmitBroadcast_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const transmitBroadcast_Out,
        };
        unsafe {
            instance.get(transmitBroadcast_Out::new)
        }
    }

    // optional .linkbot.daemon.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::OK)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for transmitBroadcast_Out {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for transmitBroadcast_Out {
    fn new() -> transmitBroadcast_Out {
        transmitBroadcast_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<transmitBroadcast_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    transmitBroadcast_Out::get_status_for_reflect,
                    transmitBroadcast_Out::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<transmitBroadcast_Out>(
                    "transmitBroadcast_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for transmitBroadcast_Out {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for transmitBroadcast_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for transmitBroadcast_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcRequest {
    // message fields
    requestId: ::std::option::Option<u32>,
    // message oneof groups
    arg: ::std::option::Option<RpcRequest_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcRequest {}

#[derive(Clone,PartialEq)]
pub enum RpcRequest_oneof_arg {
    getDaemonVersionString(getDaemonVersionString_In),
    getDongleCount(getDongleCount_In),
    addRobotRefs(addRobotRefs_In),
    releaseRobotRefs(releaseRobotRefs_In),
    transmit(transmit_In),
    transmitBroadcast(transmitBroadcast_In),
    acquireRobotRef(acquireRobotRef_In),
}

impl RpcRequest {
    pub fn new() -> RpcRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcRequest {
        static mut instance: ::protobuf::lazy::Lazy<RpcRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcRequest,
        };
        unsafe {
            instance.get(RpcRequest::new)
        }
    }

    // optional uint32 requestId = 1;

    pub fn clear_requestId(&mut self) {
        self.requestId = ::std::option::Option::None;
    }

    pub fn has_requestId(&self) -> bool {
        self.requestId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestId(&mut self, v: u32) {
        self.requestId = ::std::option::Option::Some(v);
    }

    pub fn get_requestId(&self) -> u32 {
        self.requestId.unwrap_or(0)
    }

    fn get_requestId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.requestId
    }

    fn mut_requestId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.requestId
    }

    // optional .linkbot.daemon.getDaemonVersionString.In getDaemonVersionString = 2;

    pub fn clear_getDaemonVersionString(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getDaemonVersionString(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getDaemonVersionString(&mut self, v: getDaemonVersionString_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getDaemonVersionString(&mut self) -> &mut getDaemonVersionString_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(getDaemonVersionString_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getDaemonVersionString(&mut self) -> getDaemonVersionString_In {
        if self.has_getDaemonVersionString() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(v)) => v,
                _ => panic!(),
            }
        } else {
            getDaemonVersionString_In::new()
        }
    }

    pub fn get_getDaemonVersionString(&self) -> &getDaemonVersionString_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(ref v)) => v,
            _ => getDaemonVersionString_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.getDongleCount.In getDongleCount = 3;

    pub fn clear_getDongleCount(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getDongleCount(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getDongleCount(&mut self, v: getDongleCount_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getDongleCount(&mut self) -> &mut getDongleCount_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(getDongleCount_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getDongleCount(&mut self) -> getDongleCount_In {
        if self.has_getDongleCount() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(v)) => v,
                _ => panic!(),
            }
        } else {
            getDongleCount_In::new()
        }
    }

    pub fn get_getDongleCount(&self) -> &getDongleCount_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(ref v)) => v,
            _ => getDongleCount_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.addRobotRefs.In addRobotRefs = 4;

    pub fn clear_addRobotRefs(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_addRobotRefs(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_addRobotRefs(&mut self, v: addRobotRefs_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_addRobotRefs(&mut self) -> &mut addRobotRefs_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(addRobotRefs_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_addRobotRefs(&mut self) -> addRobotRefs_In {
        if self.has_addRobotRefs() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(v)) => v,
                _ => panic!(),
            }
        } else {
            addRobotRefs_In::new()
        }
    }

    pub fn get_addRobotRefs(&self) -> &addRobotRefs_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(ref v)) => v,
            _ => addRobotRefs_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.releaseRobotRefs.In releaseRobotRefs = 5;

    pub fn clear_releaseRobotRefs(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_releaseRobotRefs(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_releaseRobotRefs(&mut self, v: releaseRobotRefs_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_releaseRobotRefs(&mut self) -> &mut releaseRobotRefs_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(releaseRobotRefs_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_releaseRobotRefs(&mut self) -> releaseRobotRefs_In {
        if self.has_releaseRobotRefs() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(v)) => v,
                _ => panic!(),
            }
        } else {
            releaseRobotRefs_In::new()
        }
    }

    pub fn get_releaseRobotRefs(&self) -> &releaseRobotRefs_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(ref v)) => v,
            _ => releaseRobotRefs_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.transmit.In transmit = 6;

    pub fn clear_transmit(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_transmit(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transmit(&mut self, v: transmit_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transmit(&mut self) -> &mut transmit_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(transmit_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transmit(&mut self) -> transmit_In {
        if self.has_transmit() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(v)) => v,
                _ => panic!(),
            }
        } else {
            transmit_In::new()
        }
    }

    pub fn get_transmit(&self) -> &transmit_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(ref v)) => v,
            _ => transmit_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.transmitBroadcast.In transmitBroadcast = 7;

    pub fn clear_transmitBroadcast(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_transmitBroadcast(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transmitBroadcast(&mut self, v: transmitBroadcast_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transmitBroadcast(&mut self) -> &mut transmitBroadcast_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(transmitBroadcast_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transmitBroadcast(&mut self) -> transmitBroadcast_In {
        if self.has_transmitBroadcast() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(v)) => v,
                _ => panic!(),
            }
        } else {
            transmitBroadcast_In::new()
        }
    }

    pub fn get_transmitBroadcast(&self) -> &transmitBroadcast_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(ref v)) => v,
            _ => transmitBroadcast_In::default_instance(),
        }
    }

    // optional .linkbot.daemon.acquireRobotRef.In acquireRobotRef = 8;

    pub fn clear_acquireRobotRef(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_acquireRobotRef(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_acquireRobotRef(&mut self, v: acquireRobotRef_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(v))
    }

    // Mutable pointer to the field.
    pub fn mut_acquireRobotRef(&mut self) -> &mut acquireRobotRef_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(acquireRobotRef_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_acquireRobotRef(&mut self) -> acquireRobotRef_In {
        if self.has_acquireRobotRef() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(v)) => v,
                _ => panic!(),
            }
        } else {
            acquireRobotRef_In::new()
        }
    }

    pub fn get_acquireRobotRef(&self) -> &acquireRobotRef_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(ref v)) => v,
            _ => acquireRobotRef_In::default_instance(),
        }
    }
}

impl ::protobuf::Message for RpcRequest {
    fn is_initialized(&self) -> bool {
        if let Some(RpcRequest_oneof_arg::getDaemonVersionString(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::getDongleCount(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::addRobotRefs(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::releaseRobotRefs(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::transmit(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::transmitBroadcast(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::acquireRobotRef(ref v)) = self.arg {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.requestId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDaemonVersionString(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getDongleCount(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::addRobotRefs(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::releaseRobotRefs(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmitBroadcast(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::acquireRobotRef(is.read_message()?));
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
        if let Some(v) = self.requestId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RpcRequest_oneof_arg::getDaemonVersionString(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::getDongleCount(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::addRobotRefs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::releaseRobotRefs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::transmit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::transmitBroadcast(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::acquireRobotRef(ref v) => {
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
        if let Some(v) = self.requestId {
            os.write_uint32(1, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RpcRequest_oneof_arg::getDaemonVersionString(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::getDongleCount(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::addRobotRefs(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::releaseRobotRefs(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::transmit(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::transmitBroadcast(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::acquireRobotRef(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RpcRequest {
    fn new() -> RpcRequest {
        RpcRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "requestId",
                    RpcRequest::get_requestId_for_reflect,
                    RpcRequest::mut_requestId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getDaemonVersionString_In>(
                    "getDaemonVersionString",
                    RpcRequest::has_getDaemonVersionString,
                    RpcRequest::get_getDaemonVersionString,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getDongleCount_In>(
                    "getDongleCount",
                    RpcRequest::has_getDongleCount,
                    RpcRequest::get_getDongleCount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, addRobotRefs_In>(
                    "addRobotRefs",
                    RpcRequest::has_addRobotRefs,
                    RpcRequest::get_addRobotRefs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, releaseRobotRefs_In>(
                    "releaseRobotRefs",
                    RpcRequest::has_releaseRobotRefs,
                    RpcRequest::get_releaseRobotRefs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmit_In>(
                    "transmit",
                    RpcRequest::has_transmit,
                    RpcRequest::get_transmit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmitBroadcast_In>(
                    "transmitBroadcast",
                    RpcRequest::has_transmitBroadcast,
                    RpcRequest::get_transmitBroadcast,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, acquireRobotRef_In>(
                    "acquireRobotRef",
                    RpcRequest::has_acquireRobotRef,
                    RpcRequest::get_acquireRobotRef,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcRequest>(
                    "RpcRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcRequest {
    fn clear(&mut self) {
        self.clear_requestId();
        self.clear_getDaemonVersionString();
        self.clear_getDongleCount();
        self.clear_addRobotRefs();
        self.clear_releaseRobotRefs();
        self.clear_transmit();
        self.clear_transmitBroadcast();
        self.clear_acquireRobotRef();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcReply {
    // message fields
    requestId: ::std::option::Option<u32>,
    // message oneof groups
    arg: ::std::option::Option<RpcReply_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcReply {}

#[derive(Clone,PartialEq)]
pub enum RpcReply_oneof_arg {
    getDaemonVersionString(getDaemonVersionString_Out),
    getDongleCount(getDongleCount_Out),
    addRobotRefs(addRobotRefs_Out),
    releaseRobotRefs(releaseRobotRefs_Out),
    transmit(transmit_Out),
    transmitBroadcast(transmitBroadcast_Out),
    acquireRobotRef(acquireRobotRef_Out),
}

impl RpcReply {
    pub fn new() -> RpcReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcReply {
        static mut instance: ::protobuf::lazy::Lazy<RpcReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcReply,
        };
        unsafe {
            instance.get(RpcReply::new)
        }
    }

    // optional uint32 requestId = 1;

    pub fn clear_requestId(&mut self) {
        self.requestId = ::std::option::Option::None;
    }

    pub fn has_requestId(&self) -> bool {
        self.requestId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestId(&mut self, v: u32) {
        self.requestId = ::std::option::Option::Some(v);
    }

    pub fn get_requestId(&self) -> u32 {
        self.requestId.unwrap_or(0)
    }

    fn get_requestId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.requestId
    }

    fn mut_requestId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.requestId
    }

    // optional .linkbot.daemon.getDaemonVersionString.Out getDaemonVersionString = 2;

    pub fn clear_getDaemonVersionString(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getDaemonVersionString(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getDaemonVersionString(&mut self, v: getDaemonVersionString_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getDaemonVersionString(&mut self) -> &mut getDaemonVersionString_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(getDaemonVersionString_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getDaemonVersionString(&mut self) -> getDaemonVersionString_Out {
        if self.has_getDaemonVersionString() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(v)) => v,
                _ => panic!(),
            }
        } else {
            getDaemonVersionString_Out::new()
        }
    }

    pub fn get_getDaemonVersionString(&self) -> &getDaemonVersionString_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(ref v)) => v,
            _ => getDaemonVersionString_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.getDongleCount.Out getDongleCount = 3;

    pub fn clear_getDongleCount(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getDongleCount(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getDongleCount(&mut self, v: getDongleCount_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getDongleCount(&mut self) -> &mut getDongleCount_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(getDongleCount_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getDongleCount(&mut self) -> getDongleCount_Out {
        if self.has_getDongleCount() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(v)) => v,
                _ => panic!(),
            }
        } else {
            getDongleCount_Out::new()
        }
    }

    pub fn get_getDongleCount(&self) -> &getDongleCount_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(ref v)) => v,
            _ => getDongleCount_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.addRobotRefs.Out addRobotRefs = 4;

    pub fn clear_addRobotRefs(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_addRobotRefs(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_addRobotRefs(&mut self, v: addRobotRefs_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_addRobotRefs(&mut self) -> &mut addRobotRefs_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(addRobotRefs_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_addRobotRefs(&mut self) -> addRobotRefs_Out {
        if self.has_addRobotRefs() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(v)) => v,
                _ => panic!(),
            }
        } else {
            addRobotRefs_Out::new()
        }
    }

    pub fn get_addRobotRefs(&self) -> &addRobotRefs_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(ref v)) => v,
            _ => addRobotRefs_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.releaseRobotRefs.Out releaseRobotRefs = 5;

    pub fn clear_releaseRobotRefs(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_releaseRobotRefs(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_releaseRobotRefs(&mut self, v: releaseRobotRefs_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_releaseRobotRefs(&mut self) -> &mut releaseRobotRefs_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(releaseRobotRefs_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_releaseRobotRefs(&mut self) -> releaseRobotRefs_Out {
        if self.has_releaseRobotRefs() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(v)) => v,
                _ => panic!(),
            }
        } else {
            releaseRobotRefs_Out::new()
        }
    }

    pub fn get_releaseRobotRefs(&self) -> &releaseRobotRefs_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(ref v)) => v,
            _ => releaseRobotRefs_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.transmit.Out transmit = 6;

    pub fn clear_transmit(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_transmit(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transmit(&mut self, v: transmit_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transmit(&mut self) -> &mut transmit_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::transmit(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmit(transmit_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transmit(&mut self) -> transmit_Out {
        if self.has_transmit() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::transmit(v)) => v,
                _ => panic!(),
            }
        } else {
            transmit_Out::new()
        }
    }

    pub fn get_transmit(&self) -> &transmit_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmit(ref v)) => v,
            _ => transmit_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.transmitBroadcast.Out transmitBroadcast = 7;

    pub fn clear_transmitBroadcast(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_transmitBroadcast(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_transmitBroadcast(&mut self, v: transmitBroadcast_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(v))
    }

    // Mutable pointer to the field.
    pub fn mut_transmitBroadcast(&mut self) -> &mut transmitBroadcast_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(transmitBroadcast_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_transmitBroadcast(&mut self) -> transmitBroadcast_Out {
        if self.has_transmitBroadcast() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(v)) => v,
                _ => panic!(),
            }
        } else {
            transmitBroadcast_Out::new()
        }
    }

    pub fn get_transmitBroadcast(&self) -> &transmitBroadcast_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(ref v)) => v,
            _ => transmitBroadcast_Out::default_instance(),
        }
    }

    // optional .linkbot.daemon.acquireRobotRef.Out acquireRobotRef = 8;

    pub fn clear_acquireRobotRef(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_acquireRobotRef(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_acquireRobotRef(&mut self, v: acquireRobotRef_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(v))
    }

    // Mutable pointer to the field.
    pub fn mut_acquireRobotRef(&mut self) -> &mut acquireRobotRef_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(acquireRobotRef_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_acquireRobotRef(&mut self) -> acquireRobotRef_Out {
        if self.has_acquireRobotRef() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(v)) => v,
                _ => panic!(),
            }
        } else {
            acquireRobotRef_Out::new()
        }
    }

    pub fn get_acquireRobotRef(&self) -> &acquireRobotRef_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(ref v)) => v,
            _ => acquireRobotRef_Out::default_instance(),
        }
    }
}

impl ::protobuf::Message for RpcReply {
    fn is_initialized(&self) -> bool {
        if let Some(RpcReply_oneof_arg::getDaemonVersionString(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::getDongleCount(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::addRobotRefs(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::releaseRobotRefs(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::transmit(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::transmitBroadcast(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::acquireRobotRef(ref v)) = self.arg {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.requestId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDaemonVersionString(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getDongleCount(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::addRobotRefs(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::releaseRobotRefs(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmit(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmitBroadcast(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::acquireRobotRef(is.read_message()?));
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
        if let Some(v) = self.requestId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RpcReply_oneof_arg::getDaemonVersionString(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::getDongleCount(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::addRobotRefs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::releaseRobotRefs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::transmit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::transmitBroadcast(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::acquireRobotRef(ref v) => {
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
        if let Some(v) = self.requestId {
            os.write_uint32(1, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.arg {
            match v {
                &RpcReply_oneof_arg::getDaemonVersionString(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::getDongleCount(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::addRobotRefs(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::releaseRobotRefs(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::transmit(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::transmitBroadcast(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::acquireRobotRef(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RpcReply {
    fn new() -> RpcReply {
        RpcReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "requestId",
                    RpcReply::get_requestId_for_reflect,
                    RpcReply::mut_requestId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getDaemonVersionString_Out>(
                    "getDaemonVersionString",
                    RpcReply::has_getDaemonVersionString,
                    RpcReply::get_getDaemonVersionString,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getDongleCount_Out>(
                    "getDongleCount",
                    RpcReply::has_getDongleCount,
                    RpcReply::get_getDongleCount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, addRobotRefs_Out>(
                    "addRobotRefs",
                    RpcReply::has_addRobotRefs,
                    RpcReply::get_addRobotRefs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, releaseRobotRefs_Out>(
                    "releaseRobotRefs",
                    RpcReply::has_releaseRobotRefs,
                    RpcReply::get_releaseRobotRefs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmit_Out>(
                    "transmit",
                    RpcReply::has_transmit,
                    RpcReply::get_transmit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmitBroadcast_Out>(
                    "transmitBroadcast",
                    RpcReply::has_transmitBroadcast,
                    RpcReply::get_transmitBroadcast,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, acquireRobotRef_Out>(
                    "acquireRobotRef",
                    RpcReply::has_acquireRobotRef,
                    RpcReply::get_acquireRobotRef,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcReply>(
                    "RpcReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcReply {
    fn clear(&mut self) {
        self.clear_requestId();
        self.clear_getDaemonVersionString();
        self.clear_getDongleCount();
        self.clear_addRobotRefs();
        self.clear_releaseRobotRefs();
        self.clear_transmit();
        self.clear_transmitBroadcast();
        self.clear_acquireRobotRef();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReceiveTransmission {
    // message fields
    serialId: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    payload: ::protobuf::SingularPtrField<super::robot::RobotToClient>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReceiveTransmission {}

impl ReceiveTransmission {
    pub fn new() -> ReceiveTransmission {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReceiveTransmission {
        static mut instance: ::protobuf::lazy::Lazy<ReceiveTransmission> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReceiveTransmission,
        };
        unsafe {
            instance.get(ReceiveTransmission::new)
        }
    }

    // optional .linkbot.SerialId serialId = 1;

    pub fn clear_serialId(&mut self) {
        self.serialId.clear();
    }

    pub fn has_serialId(&self) -> bool {
        self.serialId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialId(&mut self, v: super::commontypes::SerialId) {
        self.serialId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialId(&mut self) -> &mut super::commontypes::SerialId {
        if self.serialId.is_none() {
            self.serialId.set_default();
        }
        self.serialId.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialId(&mut self) -> super::commontypes::SerialId {
        self.serialId.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_serialId(&self) -> &super::commontypes::SerialId {
        self.serialId.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_serialId_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.serialId
    }

    fn mut_serialId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.serialId
    }

    // optional .linkbot.robot.RobotToClient payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: super::robot::RobotToClient) {
        self.payload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut super::robot::RobotToClient {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> super::robot::RobotToClient {
        self.payload.take().unwrap_or_else(|| super::robot::RobotToClient::new())
    }

    pub fn get_payload(&self) -> &super::robot::RobotToClient {
        self.payload.as_ref().unwrap_or_else(|| super::robot::RobotToClient::default_instance())
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularPtrField<super::robot::RobotToClient> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::robot::RobotToClient> {
        &mut self.payload
    }
}

impl ::protobuf::Message for ReceiveTransmission {
    fn is_initialized(&self) -> bool {
        for v in &self.serialId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.serialId)?;
                },
                2 => {
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
        if let Some(ref v) = self.serialId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.serialId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ReceiveTransmission {
    fn new() -> ReceiveTransmission {
        ReceiveTransmission::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReceiveTransmission>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "serialId",
                    ReceiveTransmission::get_serialId_for_reflect,
                    ReceiveTransmission::mut_serialId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::robot::RobotToClient>>(
                    "payload",
                    ReceiveTransmission::get_payload_for_reflect,
                    ReceiveTransmission::mut_payload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReceiveTransmission>(
                    "ReceiveTransmission",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReceiveTransmission {
    fn clear(&mut self) {
        self.clear_serialId();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReceiveTransmission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceiveTransmission {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DongleEvent {
    // message fields
    firmwareVersion: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DongleEvent {}

impl DongleEvent {
    pub fn new() -> DongleEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DongleEvent {
        static mut instance: ::protobuf::lazy::Lazy<DongleEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DongleEvent,
        };
        unsafe {
            instance.get(DongleEvent::new)
        }
    }

    // optional string firmwareVersion = 1;

    pub fn clear_firmwareVersion(&mut self) {
        self.firmwareVersion.clear();
    }

    pub fn has_firmwareVersion(&self) -> bool {
        self.firmwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmwareVersion(&mut self, v: ::std::string::String) {
        self.firmwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmwareVersion(&mut self) -> &mut ::std::string::String {
        if self.firmwareVersion.is_none() {
            self.firmwareVersion.set_default();
        }
        self.firmwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmwareVersion(&mut self) -> ::std::string::String {
        self.firmwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmwareVersion(&self) -> &str {
        match self.firmwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_firmwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.firmwareVersion
    }

    fn mut_firmwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.firmwareVersion
    }
}

impl ::protobuf::Message for DongleEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmwareVersion)?;
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
        if let Some(ref v) = self.firmwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.firmwareVersion.as_ref() {
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

impl ::protobuf::MessageStatic for DongleEvent {
    fn new() -> DongleEvent {
        DongleEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<DongleEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "firmwareVersion",
                    DongleEvent::get_firmwareVersion_for_reflect,
                    DongleEvent::mut_firmwareVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DongleEvent>(
                    "DongleEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DongleEvent {
    fn clear(&mut self) {
        self.clear_firmwareVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DongleEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DongleEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RobotEvent {
    // message fields
    serialId: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    firmwareVersion: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RobotEvent {}

impl RobotEvent {
    pub fn new() -> RobotEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RobotEvent {
        static mut instance: ::protobuf::lazy::Lazy<RobotEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RobotEvent,
        };
        unsafe {
            instance.get(RobotEvent::new)
        }
    }

    // optional .linkbot.SerialId serialId = 1;

    pub fn clear_serialId(&mut self) {
        self.serialId.clear();
    }

    pub fn has_serialId(&self) -> bool {
        self.serialId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialId(&mut self, v: super::commontypes::SerialId) {
        self.serialId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialId(&mut self) -> &mut super::commontypes::SerialId {
        if self.serialId.is_none() {
            self.serialId.set_default();
        }
        self.serialId.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialId(&mut self) -> super::commontypes::SerialId {
        self.serialId.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_serialId(&self) -> &super::commontypes::SerialId {
        self.serialId.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_serialId_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.serialId
    }

    fn mut_serialId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.serialId
    }

    // optional string firmwareVersion = 2;

    pub fn clear_firmwareVersion(&mut self) {
        self.firmwareVersion.clear();
    }

    pub fn has_firmwareVersion(&self) -> bool {
        self.firmwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firmwareVersion(&mut self, v: ::std::string::String) {
        self.firmwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firmwareVersion(&mut self) -> &mut ::std::string::String {
        if self.firmwareVersion.is_none() {
            self.firmwareVersion.set_default();
        }
        self.firmwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_firmwareVersion(&mut self) -> ::std::string::String {
        self.firmwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firmwareVersion(&self) -> &str {
        match self.firmwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_firmwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.firmwareVersion
    }

    fn mut_firmwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.firmwareVersion
    }
}

impl ::protobuf::Message for RobotEvent {
    fn is_initialized(&self) -> bool {
        for v in &self.serialId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.serialId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firmwareVersion)?;
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
        if let Some(ref v) = self.serialId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.firmwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.serialId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.firmwareVersion.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for RobotEvent {
    fn new() -> RobotEvent {
        RobotEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<RobotEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commontypes::SerialId>>(
                    "serialId",
                    RobotEvent::get_serialId_for_reflect,
                    RobotEvent::mut_serialId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "firmwareVersion",
                    RobotEvent::get_firmwareVersion_for_reflect,
                    RobotEvent::mut_firmwareVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RobotEvent>(
                    "RobotEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RobotEvent {
    fn clear(&mut self) {
        self.clear_serialId();
        self.clear_firmwareVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RobotEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RobotEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientToDaemon {
    // message oneof groups
    arg: ::std::option::Option<ClientToDaemon_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientToDaemon {}

#[derive(Clone,PartialEq)]
pub enum ClientToDaemon_oneof_arg {
    rpcRequest(RpcRequest),
}

impl ClientToDaemon {
    pub fn new() -> ClientToDaemon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientToDaemon {
        static mut instance: ::protobuf::lazy::Lazy<ClientToDaemon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientToDaemon,
        };
        unsafe {
            instance.get(ClientToDaemon::new)
        }
    }

    // optional .linkbot.daemon.RpcRequest rpcRequest = 1;

    pub fn clear_rpcRequest(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_rpcRequest(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rpcRequest(&mut self, v: RpcRequest) {
        self.arg = ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rpcRequest(&mut self) -> &mut RpcRequest {
        if let ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(RpcRequest::new()));
        }
        match self.arg {
            ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rpcRequest(&mut self) -> RpcRequest {
        if self.has_rpcRequest() {
            match self.arg.take() {
                ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            RpcRequest::new()
        }
    }

    pub fn get_rpcRequest(&self) -> &RpcRequest {
        match self.arg {
            ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(ref v)) => v,
            _ => RpcRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for ClientToDaemon {
    fn is_initialized(&self) -> bool {
        if let Some(ClientToDaemon_oneof_arg::rpcRequest(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(ClientToDaemon_oneof_arg::rpcRequest(is.read_message()?));
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
                &ClientToDaemon_oneof_arg::rpcRequest(ref v) => {
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
                &ClientToDaemon_oneof_arg::rpcRequest(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientToDaemon {
    fn new() -> ClientToDaemon {
        ClientToDaemon::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientToDaemon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RpcRequest>(
                    "rpcRequest",
                    ClientToDaemon::has_rpcRequest,
                    ClientToDaemon::get_rpcRequest,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientToDaemon>(
                    "ClientToDaemon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientToDaemon {
    fn clear(&mut self) {
        self.clear_rpcRequest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientToDaemon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientToDaemon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonToClient {
    // message oneof groups
    arg: ::std::option::Option<DaemonToClient_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonToClient {}

#[derive(Clone,PartialEq)]
pub enum DaemonToClient_oneof_arg {
    rpcReply(RpcReply),
    receive(ReceiveTransmission),
    dongleEvent(DongleEvent),
    robotEvent(RobotEvent),
}

impl DaemonToClient {
    pub fn new() -> DaemonToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonToClient {
        static mut instance: ::protobuf::lazy::Lazy<DaemonToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonToClient,
        };
        unsafe {
            instance.get(DaemonToClient::new)
        }
    }

    // optional .linkbot.daemon.RpcReply rpcReply = 1;

    pub fn clear_rpcReply(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_rpcReply(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rpcReply(&mut self, v: RpcReply) {
        self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rpcReply(&mut self) -> &mut RpcReply {
        if let ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(RpcReply::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rpcReply(&mut self) -> RpcReply {
        if self.has_rpcReply() {
            match self.arg.take() {
                ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(v)) => v,
                _ => panic!(),
            }
        } else {
            RpcReply::new()
        }
    }

    pub fn get_rpcReply(&self) -> &RpcReply {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(ref v)) => v,
            _ => RpcReply::default_instance(),
        }
    }

    // optional .linkbot.daemon.ReceiveTransmission receive = 2;

    pub fn clear_receive(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_receive(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_receive(&mut self, v: ReceiveTransmission) {
        self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(v))
    }

    // Mutable pointer to the field.
    pub fn mut_receive(&mut self) -> &mut ReceiveTransmission {
        if let ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(ReceiveTransmission::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_receive(&mut self) -> ReceiveTransmission {
        if self.has_receive() {
            match self.arg.take() {
                ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(v)) => v,
                _ => panic!(),
            }
        } else {
            ReceiveTransmission::new()
        }
    }

    pub fn get_receive(&self) -> &ReceiveTransmission {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(ref v)) => v,
            _ => ReceiveTransmission::default_instance(),
        }
    }

    // optional .linkbot.daemon.DongleEvent dongleEvent = 3;

    pub fn clear_dongleEvent(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_dongleEvent(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_dongleEvent(&mut self, v: DongleEvent) {
        self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(v))
    }

    // Mutable pointer to the field.
    pub fn mut_dongleEvent(&mut self) -> &mut DongleEvent {
        if let ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(DongleEvent::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_dongleEvent(&mut self) -> DongleEvent {
        if self.has_dongleEvent() {
            match self.arg.take() {
                ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(v)) => v,
                _ => panic!(),
            }
        } else {
            DongleEvent::new()
        }
    }

    pub fn get_dongleEvent(&self) -> &DongleEvent {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(ref v)) => v,
            _ => DongleEvent::default_instance(),
        }
    }

    // optional .linkbot.daemon.RobotEvent robotEvent = 4;

    pub fn clear_robotEvent(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_robotEvent(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_robotEvent(&mut self, v: RobotEvent) {
        self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(v))
    }

    // Mutable pointer to the field.
    pub fn mut_robotEvent(&mut self) -> &mut RobotEvent {
        if let ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(RobotEvent::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_robotEvent(&mut self) -> RobotEvent {
        if self.has_robotEvent() {
            match self.arg.take() {
                ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(v)) => v,
                _ => panic!(),
            }
        } else {
            RobotEvent::new()
        }
    }

    pub fn get_robotEvent(&self) -> &RobotEvent {
        match self.arg {
            ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(ref v)) => v,
            _ => RobotEvent::default_instance(),
        }
    }
}

impl ::protobuf::Message for DaemonToClient {
    fn is_initialized(&self) -> bool {
        if let Some(DaemonToClient_oneof_arg::rpcReply(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DaemonToClient_oneof_arg::receive(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DaemonToClient_oneof_arg::dongleEvent(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DaemonToClient_oneof_arg::robotEvent(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::rpcReply(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::receive(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::dongleEvent(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(DaemonToClient_oneof_arg::robotEvent(is.read_message()?));
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
                &DaemonToClient_oneof_arg::rpcReply(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DaemonToClient_oneof_arg::receive(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DaemonToClient_oneof_arg::dongleEvent(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DaemonToClient_oneof_arg::robotEvent(ref v) => {
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
                &DaemonToClient_oneof_arg::rpcReply(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DaemonToClient_oneof_arg::receive(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DaemonToClient_oneof_arg::dongleEvent(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DaemonToClient_oneof_arg::robotEvent(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DaemonToClient {
    fn new() -> DaemonToClient {
        DaemonToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RpcReply>(
                    "rpcReply",
                    DaemonToClient::has_rpcReply,
                    DaemonToClient::get_rpcReply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ReceiveTransmission>(
                    "receive",
                    DaemonToClient::has_receive,
                    DaemonToClient::get_receive,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DongleEvent>(
                    "dongleEvent",
                    DaemonToClient::has_dongleEvent,
                    DaemonToClient::get_dongleEvent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RobotEvent>(
                    "robotEvent",
                    DaemonToClient::has_robotEvent,
                    DaemonToClient::get_robotEvent,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonToClient>(
                    "DaemonToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonToClient {
    fn clear(&mut self) {
        self.clear_rpcReply();
        self.clear_receive();
        self.clear_dongleEvent();
        self.clear_robotEvent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    OK = 0,
    CANNOT_OPEN_DONGLE = 1,
    DONGLE_NOT_FOUND = 2,
    PORT_OUT_OF_RANGE = 3,
    UNREGISTERED_SERIALID = 5,
    INVALID_SERIALID = 6,
    DAEMON_UNAVAILABLE = 7,
    STRANGE_DONGLE = 8,
    INCOMPATIBLE_FIRMWARE = 9,
    BUFFER_OVERFLOW = 10,
    OTHER_ERROR = 11,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::OK),
            1 => ::std::option::Option::Some(Status::CANNOT_OPEN_DONGLE),
            2 => ::std::option::Option::Some(Status::DONGLE_NOT_FOUND),
            3 => ::std::option::Option::Some(Status::PORT_OUT_OF_RANGE),
            5 => ::std::option::Option::Some(Status::UNREGISTERED_SERIALID),
            6 => ::std::option::Option::Some(Status::INVALID_SERIALID),
            7 => ::std::option::Option::Some(Status::DAEMON_UNAVAILABLE),
            8 => ::std::option::Option::Some(Status::STRANGE_DONGLE),
            9 => ::std::option::Option::Some(Status::INCOMPATIBLE_FIRMWARE),
            10 => ::std::option::Option::Some(Status::BUFFER_OVERFLOW),
            11 => ::std::option::Option::Some(Status::OTHER_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::OK,
            Status::CANNOT_OPEN_DONGLE,
            Status::DONGLE_NOT_FOUND,
            Status::PORT_OUT_OF_RANGE,
            Status::UNREGISTERED_SERIALID,
            Status::INVALID_SERIALID,
            Status::DAEMON_UNAVAILABLE,
            Status::STRANGE_DONGLE,
            Status::INCOMPATIBLE_FIRMWARE,
            Status::BUFFER_OVERFLOW,
            Status::OTHER_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BroadcastMethod {
    BROADCAST = 0,
    MULTICAST_LOCAL = 1,
    MULTICAST_GLOBAL = 2,
}

impl ::protobuf::ProtobufEnum for BroadcastMethod {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BroadcastMethod> {
        match value {
            0 => ::std::option::Option::Some(BroadcastMethod::BROADCAST),
            1 => ::std::option::Option::Some(BroadcastMethod::MULTICAST_LOCAL),
            2 => ::std::option::Option::Some(BroadcastMethod::MULTICAST_GLOBAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BroadcastMethod] = &[
            BroadcastMethod::BROADCAST,
            BroadcastMethod::MULTICAST_LOCAL,
            BroadcastMethod::MULTICAST_GLOBAL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BroadcastMethod>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BroadcastMethod", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BroadcastMethod {
}

impl ::protobuf::reflect::ProtobufValue for BroadcastMethod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cdaemon.proto\x12\x0elinkbot.daemon\x1a\x0cnanopb.proto\x1a\x11comm\
    ontypes.proto\x1a\x0brobot.proto\"C\n\x16getDaemonVersionString\x1a\x04\
    \n\x02In\x1a#\n\x03Out\x12\x1c\n\x05value\x18\x01\x20\x01(\tR\x05valueB\
    \x06\x92?\x03\x08\x80\x01\"?\n\x0egetDongleCount\x1a\x04\n\x02In\x1a'\n\
    \x03Out\x12\x20\n\x0bdongleCount\x18\x01\x20\x01(\rR\x0bdongleCount\"\
    \x83\x01\n\x0caddRobotRefs\x1a<\n\x02In\x126\n\tserialIds\x18\x01\x20\
    \x03(\x0b2\x11.linkbot.SerialIdR\tserialIdsB\x05\x92?\x02\x10\x10\x1a5\n\
    \x03Out\x12.\n\x06status\x18\x01\x20\x01(\x0e2\x16.linkbot.daemon.Status\
    R\x06status\"M\n\x0facquireRobotRef\x1a\x04\n\x02In\x1a4\n\x03Out\x12-\n\
    \x08serialId\x18\x01\x20\x01(\x0b2\x11.linkbot.SerialIdR\x08serialId\"\
    \x87\x01\n\x10releaseRobotRefs\x1a<\n\x02In\x126\n\tserialIds\x18\x01\
    \x20\x03(\x0b2\x11.linkbot.SerialIdR\tserialIdsB\x05\x92?\x02\x10\x10\
    \x1a5\n\x03Out\x12.\n\x06status\x18\x01\x20\x01(\x0e2\x16.linkbot.daemon\
    .StatusR\x06status\"\xb4\x01\n\x08transmit\x1aq\n\x02In\x123\n\x0bdestin\
    ation\x18\x01\x20\x01(\x0b2\x11.linkbot.SerialIdR\x0bdestination\x126\n\
    \x07payload\x18\x02\x20\x01(\x0b2\x1c.linkbot.robot.ClientToRobotR\x07pa\
    yload\x1a5\n\x03Out\x12.\n\x06status\x18\x01\x20\x01(\x0e2\x16.linkbot.d\
    aemon.StatusR\x06status\"\x9b\x02\n\x11transmitBroadcast\x1a\xce\x01\n\
    \x02In\x12I\n\x0fbroadcastMethod\x18\x01\x20\x01(\x0e2\x1f.linkbot.daemo\
    n.BroadcastMethodR\x0fbroadcastMethod\x12<\n\x0cdestinations\x18\x02\x20\
    \x03(\x0b2\x11.linkbot.SerialIdR\x0cdestinationsB\x05\x92?\x02\x10\x08\
    \x12?\n\x07payload\x18\x04\x20\x01(\x0b2%.linkbot.robot.ClientToRobotBro\
    adcastR\x07payload\x1a5\n\x03Out\x12.\n\x06status\x18\x01\x20\x01(\x0e2\
    \x16.linkbot.daemon.StatusR\x06status\"\xde\x04\n\nRpcRequest\x12\x1c\n\
    \trequestId\x18\x01\x20\x01(\rR\trequestId\x12c\n\x16getDaemonVersionStr\
    ing\x18\x02\x20\x01(\x0b2).linkbot.daemon.getDaemonVersionString.InH\0R\
    \x16getDaemonVersionString\x12K\n\x0egetDongleCount\x18\x03\x20\x01(\x0b\
    2!.linkbot.daemon.getDongleCount.InH\0R\x0egetDongleCount\x12E\n\x0caddR\
    obotRefs\x18\x04\x20\x01(\x0b2\x1f.linkbot.daemon.addRobotRefs.InH\0R\
    \x0caddRobotRefs\x12Q\n\x10releaseRobotRefs\x18\x05\x20\x01(\x0b2#.linkb\
    ot.daemon.releaseRobotRefs.InH\0R\x10releaseRobotRefs\x129\n\x08transmit\
    \x18\x06\x20\x01(\x0b2\x1b.linkbot.daemon.transmit.InH\0R\x08transmit\
    \x12T\n\x11transmitBroadcast\x18\x07\x20\x01(\x0b2$.linkbot.daemon.trans\
    mitBroadcast.InH\0R\x11transmitBroadcast\x12N\n\x0facquireRobotRef\x18\
    \x08\x20\x01(\x0b2\".linkbot.daemon.acquireRobotRef.InH\0R\x0facquireRob\
    otRefB\x05\n\x03arg\"\xe3\x04\n\x08RpcReply\x12\x1c\n\trequestId\x18\x01\
    \x20\x01(\rR\trequestId\x12d\n\x16getDaemonVersionString\x18\x02\x20\x01\
    (\x0b2*.linkbot.daemon.getDaemonVersionString.OutH\0R\x16getDaemonVersio\
    nString\x12L\n\x0egetDongleCount\x18\x03\x20\x01(\x0b2\".linkbot.daemon.\
    getDongleCount.OutH\0R\x0egetDongleCount\x12F\n\x0caddRobotRefs\x18\x04\
    \x20\x01(\x0b2\x20.linkbot.daemon.addRobotRefs.OutH\0R\x0caddRobotRefs\
    \x12R\n\x10releaseRobotRefs\x18\x05\x20\x01(\x0b2$.linkbot.daemon.releas\
    eRobotRefs.OutH\0R\x10releaseRobotRefs\x12:\n\x08transmit\x18\x06\x20\
    \x01(\x0b2\x1c.linkbot.daemon.transmit.OutH\0R\x08transmit\x12U\n\x11tra\
    nsmitBroadcast\x18\x07\x20\x01(\x0b2%.linkbot.daemon.transmitBroadcast.O\
    utH\0R\x11transmitBroadcast\x12O\n\x0facquireRobotRef\x18\x08\x20\x01(\
    \x0b2#.linkbot.daemon.acquireRobotRef.OutH\0R\x0facquireRobotRefB\x05\n\
    \x03arg\"|\n\x13ReceiveTransmission\x12-\n\x08serialId\x18\x01\x20\x01(\
    \x0b2\x11.linkbot.SerialIdR\x08serialId\x126\n\x07payload\x18\x02\x20\
    \x01(\x0b2\x1c.linkbot.robot.RobotToClientR\x07payload\"?\n\x0bDongleEve\
    nt\x120\n\x0ffirmwareVersion\x18\x01\x20\x01(\tR\x0ffirmwareVersionB\x06\
    \x92?\x03\x08\x80\x01\"m\n\nRobotEvent\x12-\n\x08serialId\x18\x01\x20\
    \x01(\x0b2\x11.linkbot.SerialIdR\x08serialId\x120\n\x0ffirmwareVersion\
    \x18\x02\x20\x01(\tR\x0ffirmwareVersionB\x06\x92?\x03\x08\x80\x01\"U\n\
    \x0eClientToDaemon\x12<\n\nrpcRequest\x18\x01\x20\x01(\x0b2\x1a.linkbot.\
    daemon.RpcRequestH\0R\nrpcRequestB\x05\n\x03arg\"\x8f\x02\n\x0eDaemonToC\
    lient\x126\n\x08rpcReply\x18\x01\x20\x01(\x0b2\x18.linkbot.daemon.RpcRep\
    lyH\0R\x08rpcReply\x12?\n\x07receive\x18\x02\x20\x01(\x0b2#.linkbot.daem\
    on.ReceiveTransmissionH\0R\x07receive\x12?\n\x0bdongleEvent\x18\x03\x20\
    \x01(\x0b2\x1b.linkbot.daemon.DongleEventH\0R\x0bdongleEvent\x12<\n\nrob\
    otEvent\x18\x04\x20\x01(\x0b2\x1a.linkbot.daemon.RobotEventH\0R\nrobotEv\
    entB\x05\n\x03arg*\xf3\x01\n\x06Status\x12\x06\n\x02OK\x10\0\x12\x16\n\
    \x12CANNOT_OPEN_DONGLE\x10\x01\x12\x14\n\x10DONGLE_NOT_FOUND\x10\x02\x12\
    \x15\n\x11PORT_OUT_OF_RANGE\x10\x03\x12\x19\n\x15UNREGISTERED_SERIALID\
    \x10\x05\x12\x14\n\x10INVALID_SERIALID\x10\x06\x12\x16\n\x12DAEMON_UNAVA\
    ILABLE\x10\x07\x12\x12\n\x0eSTRANGE_DONGLE\x10\x08\x12\x19\n\x15INCOMPAT\
    IBLE_FIRMWARE\x10\t\x12\x13\n\x0fBUFFER_OVERFLOW\x10\n\x12\x0f\n\x0bOTHE\
    R_ERROR\x10\x0b*K\n\x0fBroadcastMethod\x12\r\n\tBROADCAST\x10\0\x12\x13\
    \n\x0fMULTICAST_LOCAL\x10\x01\x12\x14\n\x10MULTICAST_GLOBAL\x10\x02\
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
