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
pub struct getFirmwareVersionString {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getFirmwareVersionString {}

impl getFirmwareVersionString {
    pub fn new() -> getFirmwareVersionString {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getFirmwareVersionString {
        static mut instance: ::protobuf::lazy::Lazy<getFirmwareVersionString> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getFirmwareVersionString,
        };
        unsafe {
            instance.get(getFirmwareVersionString::new)
        }
    }
}

impl ::protobuf::Message for getFirmwareVersionString {
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

impl ::protobuf::MessageStatic for getFirmwareVersionString {
    fn new() -> getFirmwareVersionString {
        getFirmwareVersionString::new()
    }

    fn descriptor_static(_: ::std::option::Option<getFirmwareVersionString>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getFirmwareVersionString>(
                    "getFirmwareVersionString",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getFirmwareVersionString {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getFirmwareVersionString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getFirmwareVersionString {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getFirmwareVersionString_In {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getFirmwareVersionString_In {}

impl getFirmwareVersionString_In {
    pub fn new() -> getFirmwareVersionString_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getFirmwareVersionString_In {
        static mut instance: ::protobuf::lazy::Lazy<getFirmwareVersionString_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getFirmwareVersionString_In,
        };
        unsafe {
            instance.get(getFirmwareVersionString_In::new)
        }
    }
}

impl ::protobuf::Message for getFirmwareVersionString_In {
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

impl ::protobuf::MessageStatic for getFirmwareVersionString_In {
    fn new() -> getFirmwareVersionString_In {
        getFirmwareVersionString_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<getFirmwareVersionString_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<getFirmwareVersionString_In>(
                    "getFirmwareVersionString_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getFirmwareVersionString_In {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getFirmwareVersionString_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getFirmwareVersionString_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct getFirmwareVersionString_Out {
    // message fields
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for getFirmwareVersionString_Out {}

impl getFirmwareVersionString_Out {
    pub fn new() -> getFirmwareVersionString_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static getFirmwareVersionString_Out {
        static mut instance: ::protobuf::lazy::Lazy<getFirmwareVersionString_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const getFirmwareVersionString_Out,
        };
        unsafe {
            instance.get(getFirmwareVersionString_Out::new)
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

impl ::protobuf::Message for getFirmwareVersionString_Out {
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

impl ::protobuf::MessageStatic for getFirmwareVersionString_Out {
    fn new() -> getFirmwareVersionString_Out {
        getFirmwareVersionString_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<getFirmwareVersionString_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    getFirmwareVersionString_Out::get_value_for_reflect,
                    getFirmwareVersionString_Out::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<getFirmwareVersionString_Out>(
                    "getFirmwareVersionString_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for getFirmwareVersionString_Out {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for getFirmwareVersionString_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for getFirmwareVersionString_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct reboot {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for reboot {}

impl reboot {
    pub fn new() -> reboot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static reboot {
        static mut instance: ::protobuf::lazy::Lazy<reboot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const reboot,
        };
        unsafe {
            instance.get(reboot::new)
        }
    }
}

impl ::protobuf::Message for reboot {
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

impl ::protobuf::MessageStatic for reboot {
    fn new() -> reboot {
        reboot::new()
    }

    fn descriptor_static(_: ::std::option::Option<reboot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<reboot>(
                    "reboot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for reboot {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for reboot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for reboot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct reboot_In {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for reboot_In {}

impl reboot_In {
    pub fn new() -> reboot_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static reboot_In {
        static mut instance: ::protobuf::lazy::Lazy<reboot_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const reboot_In,
        };
        unsafe {
            instance.get(reboot_In::new)
        }
    }
}

impl ::protobuf::Message for reboot_In {
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

impl ::protobuf::MessageStatic for reboot_In {
    fn new() -> reboot_In {
        reboot_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<reboot_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<reboot_In>(
                    "reboot_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for reboot_In {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for reboot_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for reboot_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct reboot_Out {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for reboot_Out {}

impl reboot_Out {
    pub fn new() -> reboot_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static reboot_Out {
        static mut instance: ::protobuf::lazy::Lazy<reboot_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const reboot_Out,
        };
        unsafe {
            instance.get(reboot_Out::new)
        }
    }
}

impl ::protobuf::Message for reboot_Out {
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

impl ::protobuf::MessageStatic for reboot_Out {
    fn new() -> reboot_Out {
        reboot_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<reboot_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<reboot_Out>(
                    "reboot_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for reboot_Out {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for reboot_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for reboot_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setRadioMode {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setRadioMode {}

impl setRadioMode {
    pub fn new() -> setRadioMode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setRadioMode {
        static mut instance: ::protobuf::lazy::Lazy<setRadioMode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setRadioMode,
        };
        unsafe {
            instance.get(setRadioMode::new)
        }
    }
}

impl ::protobuf::Message for setRadioMode {
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

impl ::protobuf::MessageStatic for setRadioMode {
    fn new() -> setRadioMode {
        setRadioMode::new()
    }

    fn descriptor_static(_: ::std::option::Option<setRadioMode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<setRadioMode>(
                    "setRadioMode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setRadioMode {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setRadioMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setRadioMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setRadioMode_In {
    // message fields
    mode: ::std::option::Option<RadioMode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setRadioMode_In {}

impl setRadioMode_In {
    pub fn new() -> setRadioMode_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setRadioMode_In {
        static mut instance: ::protobuf::lazy::Lazy<setRadioMode_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setRadioMode_In,
        };
        unsafe {
            instance.get(setRadioMode_In::new)
        }
    }

    // optional .linkbot.dongle.RadioMode mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: RadioMode) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> RadioMode {
        self.mode.unwrap_or(RadioMode::FULL_DUPLEX)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<RadioMode> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<RadioMode> {
        &mut self.mode
    }
}

impl ::protobuf::Message for setRadioMode_In {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.mode, 1, &mut self.unknown_fields)?
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
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

impl ::protobuf::MessageStatic for setRadioMode_In {
    fn new() -> setRadioMode_In {
        setRadioMode_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<setRadioMode_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RadioMode>>(
                    "mode",
                    setRadioMode_In::get_mode_for_reflect,
                    setRadioMode_In::mut_mode_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<setRadioMode_In>(
                    "setRadioMode_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setRadioMode_In {
    fn clear(&mut self) {
        self.clear_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setRadioMode_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setRadioMode_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setRadioMode_Out {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setRadioMode_Out {}

impl setRadioMode_Out {
    pub fn new() -> setRadioMode_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setRadioMode_Out {
        static mut instance: ::protobuf::lazy::Lazy<setRadioMode_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setRadioMode_Out,
        };
        unsafe {
            instance.get(setRadioMode_Out::new)
        }
    }
}

impl ::protobuf::Message for setRadioMode_Out {
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

impl ::protobuf::MessageStatic for setRadioMode_Out {
    fn new() -> setRadioMode_Out {
        setRadioMode_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<setRadioMode_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<setRadioMode_Out>(
                    "setRadioMode_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setRadioMode_Out {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setRadioMode_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setRadioMode_Out {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setSessionId {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setSessionId {}

impl setSessionId {
    pub fn new() -> setSessionId {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setSessionId {
        static mut instance: ::protobuf::lazy::Lazy<setSessionId> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setSessionId,
        };
        unsafe {
            instance.get(setSessionId::new)
        }
    }
}

impl ::protobuf::Message for setSessionId {
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

impl ::protobuf::MessageStatic for setSessionId {
    fn new() -> setSessionId {
        setSessionId::new()
    }

    fn descriptor_static(_: ::std::option::Option<setSessionId>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<setSessionId>(
                    "setSessionId",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setSessionId {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setSessionId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setSessionId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setSessionId_In {
    // message fields
    sessionId: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setSessionId_In {}

impl setSessionId_In {
    pub fn new() -> setSessionId_In {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setSessionId_In {
        static mut instance: ::protobuf::lazy::Lazy<setSessionId_In> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setSessionId_In,
        };
        unsafe {
            instance.get(setSessionId_In::new)
        }
    }

    // optional uint32 sessionId = 1;

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
}

impl ::protobuf::Message for setSessionId_In {
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
                    self.sessionId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sessionId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sessionId {
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

impl ::protobuf::MessageStatic for setSessionId_In {
    fn new() -> setSessionId_In {
        setSessionId_In::new()
    }

    fn descriptor_static(_: ::std::option::Option<setSessionId_In>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sessionId",
                    setSessionId_In::get_sessionId_for_reflect,
                    setSessionId_In::mut_sessionId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<setSessionId_In>(
                    "setSessionId_In",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setSessionId_In {
    fn clear(&mut self) {
        self.clear_sessionId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setSessionId_In {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setSessionId_In {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct setSessionId_Out {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for setSessionId_Out {}

impl setSessionId_Out {
    pub fn new() -> setSessionId_Out {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static setSessionId_Out {
        static mut instance: ::protobuf::lazy::Lazy<setSessionId_Out> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const setSessionId_Out,
        };
        unsafe {
            instance.get(setSessionId_Out::new)
        }
    }
}

impl ::protobuf::Message for setSessionId_Out {
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

impl ::protobuf::MessageStatic for setSessionId_Out {
    fn new() -> setSessionId_Out {
        setSessionId_Out::new()
    }

    fn descriptor_static(_: ::std::option::Option<setSessionId_Out>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<setSessionId_Out>(
                    "setSessionId_Out",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for setSessionId_Out {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for setSessionId_Out {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for setSessionId_Out {
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
    routedRadioMessage: ::protobuf::SingularPtrField<super::radio::RoutedRadioMessage>,
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

    // optional .linkbot.radio.RoutedRadioMessage routedRadioMessage = 2;

    pub fn clear_routedRadioMessage(&mut self) {
        self.routedRadioMessage.clear();
    }

    pub fn has_routedRadioMessage(&self) -> bool {
        self.routedRadioMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routedRadioMessage(&mut self, v: super::radio::RoutedRadioMessage) {
        self.routedRadioMessage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routedRadioMessage(&mut self) -> &mut super::radio::RoutedRadioMessage {
        if self.routedRadioMessage.is_none() {
            self.routedRadioMessage.set_default();
        }
        self.routedRadioMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_routedRadioMessage(&mut self) -> super::radio::RoutedRadioMessage {
        self.routedRadioMessage.take().unwrap_or_else(|| super::radio::RoutedRadioMessage::new())
    }

    pub fn get_routedRadioMessage(&self) -> &super::radio::RoutedRadioMessage {
        self.routedRadioMessage.as_ref().unwrap_or_else(|| super::radio::RoutedRadioMessage::default_instance())
    }

    fn get_routedRadioMessage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::radio::RoutedRadioMessage> {
        &self.routedRadioMessage
    }

    fn mut_routedRadioMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::radio::RoutedRadioMessage> {
        &mut self.routedRadioMessage
    }
}

impl ::protobuf::Message for transmit_In {
    fn is_initialized(&self) -> bool {
        for v in &self.destination {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.routedRadioMessage {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.routedRadioMessage)?;
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
        if let Some(ref v) = self.routedRadioMessage.as_ref() {
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
        if let Some(ref v) = self.routedRadioMessage.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::radio::RoutedRadioMessage>>(
                    "routedRadioMessage",
                    transmit_In::get_routedRadioMessage_for_reflect,
                    transmit_In::mut_routedRadioMessage_for_reflect,
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
        self.clear_routedRadioMessage();
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
    queuedMessageCount: ::std::option::Option<i32>,
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

    // optional int32 queuedMessageCount = 1;

    pub fn clear_queuedMessageCount(&mut self) {
        self.queuedMessageCount = ::std::option::Option::None;
    }

    pub fn has_queuedMessageCount(&self) -> bool {
        self.queuedMessageCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queuedMessageCount(&mut self, v: i32) {
        self.queuedMessageCount = ::std::option::Option::Some(v);
    }

    pub fn get_queuedMessageCount(&self) -> i32 {
        self.queuedMessageCount.unwrap_or(0)
    }

    fn get_queuedMessageCount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.queuedMessageCount
    }

    fn mut_queuedMessageCount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.queuedMessageCount
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.queuedMessageCount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.queuedMessageCount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.queuedMessageCount {
            os.write_int32(1, v)?;
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "queuedMessageCount",
                    transmit_Out::get_queuedMessageCount_for_reflect,
                    transmit_Out::mut_queuedMessageCount_for_reflect,
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
        self.clear_queuedMessageCount();
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
    getFirmwareVersionString(getFirmwareVersionString_In),
    reboot(reboot_In),
    setRadioMode(setRadioMode_In),
    setSessionId(setSessionId_In),
    transmit(transmit_In),
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

    // optional .linkbot.dongle.getFirmwareVersionString.In getFirmwareVersionString = 2;

    pub fn clear_getFirmwareVersionString(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getFirmwareVersionString(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getFirmwareVersionString(&mut self, v: getFirmwareVersionString_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getFirmwareVersionString(&mut self) -> &mut getFirmwareVersionString_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(getFirmwareVersionString_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getFirmwareVersionString(&mut self) -> getFirmwareVersionString_In {
        if self.has_getFirmwareVersionString() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(v)) => v,
                _ => panic!(),
            }
        } else {
            getFirmwareVersionString_In::new()
        }
    }

    pub fn get_getFirmwareVersionString(&self) -> &getFirmwareVersionString_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(ref v)) => v,
            _ => getFirmwareVersionString_In::default_instance(),
        }
    }

    // optional .linkbot.dongle.reboot.In reboot = 3;

    pub fn clear_reboot(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_reboot(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reboot(&mut self, v: reboot_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(v))
    }

    // Mutable pointer to the field.
    pub fn mut_reboot(&mut self) -> &mut reboot_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(reboot_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reboot(&mut self) -> reboot_In {
        if self.has_reboot() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(v)) => v,
                _ => panic!(),
            }
        } else {
            reboot_In::new()
        }
    }

    pub fn get_reboot(&self) -> &reboot_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(ref v)) => v,
            _ => reboot_In::default_instance(),
        }
    }

    // optional .linkbot.dongle.setRadioMode.In setRadioMode = 4;

    pub fn clear_setRadioMode(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setRadioMode(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setRadioMode(&mut self, v: setRadioMode_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setRadioMode(&mut self) -> &mut setRadioMode_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(setRadioMode_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setRadioMode(&mut self) -> setRadioMode_In {
        if self.has_setRadioMode() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(v)) => v,
                _ => panic!(),
            }
        } else {
            setRadioMode_In::new()
        }
    }

    pub fn get_setRadioMode(&self) -> &setRadioMode_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(ref v)) => v,
            _ => setRadioMode_In::default_instance(),
        }
    }

    // optional .linkbot.dongle.setSessionId.In setSessionId = 5;

    pub fn clear_setSessionId(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setSessionId(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setSessionId(&mut self, v: setSessionId_In) {
        self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setSessionId(&mut self) -> &mut setSessionId_In {
        if let ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(setSessionId_In::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setSessionId(&mut self) -> setSessionId_In {
        if self.has_setSessionId() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(v)) => v,
                _ => panic!(),
            }
        } else {
            setSessionId_In::new()
        }
    }

    pub fn get_setSessionId(&self) -> &setSessionId_In {
        match self.arg {
            ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(ref v)) => v,
            _ => setSessionId_In::default_instance(),
        }
    }

    // optional .linkbot.dongle.transmit.In transmit = 6;

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
}

impl ::protobuf::Message for RpcRequest {
    fn is_initialized(&self) -> bool {
        if let Some(RpcRequest_oneof_arg::getFirmwareVersionString(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::reboot(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::setRadioMode(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::setSessionId(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcRequest_oneof_arg::transmit(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::getFirmwareVersionString(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::reboot(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setRadioMode(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::setSessionId(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcRequest_oneof_arg::transmit(is.read_message()?));
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
                &RpcRequest_oneof_arg::getFirmwareVersionString(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::reboot(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::setRadioMode(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::setSessionId(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcRequest_oneof_arg::transmit(ref v) => {
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
                &RpcRequest_oneof_arg::getFirmwareVersionString(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::reboot(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::setRadioMode(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::setSessionId(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcRequest_oneof_arg::transmit(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getFirmwareVersionString_In>(
                    "getFirmwareVersionString",
                    RpcRequest::has_getFirmwareVersionString,
                    RpcRequest::get_getFirmwareVersionString,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, reboot_In>(
                    "reboot",
                    RpcRequest::has_reboot,
                    RpcRequest::get_reboot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, setRadioMode_In>(
                    "setRadioMode",
                    RpcRequest::has_setRadioMode,
                    RpcRequest::get_setRadioMode,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, setSessionId_In>(
                    "setSessionId",
                    RpcRequest::has_setSessionId,
                    RpcRequest::get_setSessionId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmit_In>(
                    "transmit",
                    RpcRequest::has_transmit,
                    RpcRequest::get_transmit,
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
        self.clear_getFirmwareVersionString();
        self.clear_reboot();
        self.clear_setRadioMode();
        self.clear_setSessionId();
        self.clear_transmit();
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
    getFirmwareVersionString(getFirmwareVersionString_Out),
    reboot(reboot_Out),
    setRadioMode(setRadioMode_Out),
    setSessionId(setSessionId_Out),
    transmit(transmit_Out),
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

    // optional .linkbot.dongle.getFirmwareVersionString.Out getFirmwareVersionString = 2;

    pub fn clear_getFirmwareVersionString(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_getFirmwareVersionString(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_getFirmwareVersionString(&mut self, v: getFirmwareVersionString_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(v))
    }

    // Mutable pointer to the field.
    pub fn mut_getFirmwareVersionString(&mut self) -> &mut getFirmwareVersionString_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(getFirmwareVersionString_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_getFirmwareVersionString(&mut self) -> getFirmwareVersionString_Out {
        if self.has_getFirmwareVersionString() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(v)) => v,
                _ => panic!(),
            }
        } else {
            getFirmwareVersionString_Out::new()
        }
    }

    pub fn get_getFirmwareVersionString(&self) -> &getFirmwareVersionString_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(ref v)) => v,
            _ => getFirmwareVersionString_Out::default_instance(),
        }
    }

    // optional .linkbot.dongle.reboot.Out reboot = 3;

    pub fn clear_reboot(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_reboot(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::reboot(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_reboot(&mut self, v: reboot_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::reboot(v))
    }

    // Mutable pointer to the field.
    pub fn mut_reboot(&mut self) -> &mut reboot_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::reboot(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::reboot(reboot_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::reboot(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_reboot(&mut self) -> reboot_Out {
        if self.has_reboot() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::reboot(v)) => v,
                _ => panic!(),
            }
        } else {
            reboot_Out::new()
        }
    }

    pub fn get_reboot(&self) -> &reboot_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::reboot(ref v)) => v,
            _ => reboot_Out::default_instance(),
        }
    }

    // optional .linkbot.dongle.setRadioMode.Out setRadioMode = 4;

    pub fn clear_setRadioMode(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setRadioMode(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setRadioMode(&mut self, v: setRadioMode_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setRadioMode(&mut self) -> &mut setRadioMode_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(setRadioMode_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setRadioMode(&mut self) -> setRadioMode_Out {
        if self.has_setRadioMode() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(v)) => v,
                _ => panic!(),
            }
        } else {
            setRadioMode_Out::new()
        }
    }

    pub fn get_setRadioMode(&self) -> &setRadioMode_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(ref v)) => v,
            _ => setRadioMode_Out::default_instance(),
        }
    }

    // optional .linkbot.dongle.setSessionId.Out setSessionId = 5;

    pub fn clear_setSessionId(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_setSessionId(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_setSessionId(&mut self, v: setSessionId_Out) {
        self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(v))
    }

    // Mutable pointer to the field.
    pub fn mut_setSessionId(&mut self) -> &mut setSessionId_Out {
        if let ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(setSessionId_Out::new()));
        }
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_setSessionId(&mut self) -> setSessionId_Out {
        if self.has_setSessionId() {
            match self.arg.take() {
                ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(v)) => v,
                _ => panic!(),
            }
        } else {
            setSessionId_Out::new()
        }
    }

    pub fn get_setSessionId(&self) -> &setSessionId_Out {
        match self.arg {
            ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(ref v)) => v,
            _ => setSessionId_Out::default_instance(),
        }
    }

    // optional .linkbot.dongle.transmit.Out transmit = 6;

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
}

impl ::protobuf::Message for RpcReply {
    fn is_initialized(&self) -> bool {
        if let Some(RpcReply_oneof_arg::getFirmwareVersionString(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::reboot(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::setRadioMode(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::setSessionId(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RpcReply_oneof_arg::transmit(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::getFirmwareVersionString(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::reboot(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setRadioMode(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::setSessionId(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(RpcReply_oneof_arg::transmit(is.read_message()?));
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
                &RpcReply_oneof_arg::getFirmwareVersionString(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::reboot(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::setRadioMode(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::setSessionId(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RpcReply_oneof_arg::transmit(ref v) => {
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
                &RpcReply_oneof_arg::getFirmwareVersionString(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::reboot(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::setRadioMode(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::setSessionId(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RpcReply_oneof_arg::transmit(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, getFirmwareVersionString_Out>(
                    "getFirmwareVersionString",
                    RpcReply::has_getFirmwareVersionString,
                    RpcReply::get_getFirmwareVersionString,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, reboot_Out>(
                    "reboot",
                    RpcReply::has_reboot,
                    RpcReply::get_reboot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, setRadioMode_Out>(
                    "setRadioMode",
                    RpcReply::has_setRadioMode,
                    RpcReply::get_setRadioMode,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, setSessionId_Out>(
                    "setSessionId",
                    RpcReply::has_setSessionId,
                    RpcReply::get_setSessionId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, transmit_Out>(
                    "transmit",
                    RpcReply::has_transmit,
                    RpcReply::get_transmit,
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
        self.clear_getFirmwareVersionString();
        self.clear_reboot();
        self.clear_setRadioMode();
        self.clear_setSessionId();
        self.clear_transmit();
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
    source: ::protobuf::SingularPtrField<super::commontypes::SerialId>,
    routedRadioMessage: ::protobuf::SingularPtrField<super::radio::RoutedRadioMessage>,
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

    // optional .linkbot.SerialId source = 1;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: super::commontypes::SerialId) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut super::commontypes::SerialId {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> super::commontypes::SerialId {
        self.source.take().unwrap_or_else(|| super::commontypes::SerialId::new())
    }

    pub fn get_source(&self) -> &super::commontypes::SerialId {
        self.source.as_ref().unwrap_or_else(|| super::commontypes::SerialId::default_instance())
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::commontypes::SerialId> {
        &mut self.source
    }

    // optional .linkbot.radio.RoutedRadioMessage routedRadioMessage = 2;

    pub fn clear_routedRadioMessage(&mut self) {
        self.routedRadioMessage.clear();
    }

    pub fn has_routedRadioMessage(&self) -> bool {
        self.routedRadioMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routedRadioMessage(&mut self, v: super::radio::RoutedRadioMessage) {
        self.routedRadioMessage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routedRadioMessage(&mut self) -> &mut super::radio::RoutedRadioMessage {
        if self.routedRadioMessage.is_none() {
            self.routedRadioMessage.set_default();
        }
        self.routedRadioMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_routedRadioMessage(&mut self) -> super::radio::RoutedRadioMessage {
        self.routedRadioMessage.take().unwrap_or_else(|| super::radio::RoutedRadioMessage::new())
    }

    pub fn get_routedRadioMessage(&self) -> &super::radio::RoutedRadioMessage {
        self.routedRadioMessage.as_ref().unwrap_or_else(|| super::radio::RoutedRadioMessage::default_instance())
    }

    fn get_routedRadioMessage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::radio::RoutedRadioMessage> {
        &self.routedRadioMessage
    }

    fn mut_routedRadioMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::radio::RoutedRadioMessage> {
        &mut self.routedRadioMessage
    }
}

impl ::protobuf::Message for ReceiveTransmission {
    fn is_initialized(&self) -> bool {
        for v in &self.source {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.routedRadioMessage {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.routedRadioMessage)?;
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
        if let Some(ref v) = self.source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.routedRadioMessage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.source.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.routedRadioMessage.as_ref() {
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
                    "source",
                    ReceiveTransmission::get_source_for_reflect,
                    ReceiveTransmission::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::radio::RoutedRadioMessage>>(
                    "routedRadioMessage",
                    ReceiveTransmission::get_routedRadioMessage_for_reflect,
                    ReceiveTransmission::mut_routedRadioMessage_for_reflect,
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
        self.clear_source();
        self.clear_routedRadioMessage();
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
pub struct DebugMessage {
    // message fields
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugMessage {}

impl DebugMessage {
    pub fn new() -> DebugMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugMessage {
        static mut instance: ::protobuf::lazy::Lazy<DebugMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugMessage,
        };
        unsafe {
            instance.get(DebugMessage::new)
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

impl ::protobuf::Message for DebugMessage {
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

impl ::protobuf::MessageStatic for DebugMessage {
    fn new() -> DebugMessage {
        DebugMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    DebugMessage::get_value_for_reflect,
                    DebugMessage::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugMessage>(
                    "DebugMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugMessage {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonToDongle {
    // message oneof groups
    arg: ::std::option::Option<DaemonToDongle_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonToDongle {}

#[derive(Clone,PartialEq)]
pub enum DaemonToDongle_oneof_arg {
    rpcRequest(RpcRequest),
}

impl DaemonToDongle {
    pub fn new() -> DaemonToDongle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonToDongle {
        static mut instance: ::protobuf::lazy::Lazy<DaemonToDongle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonToDongle,
        };
        unsafe {
            instance.get(DaemonToDongle::new)
        }
    }

    // optional .linkbot.dongle.RpcRequest rpcRequest = 1;

    pub fn clear_rpcRequest(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_rpcRequest(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rpcRequest(&mut self, v: RpcRequest) {
        self.arg = ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rpcRequest(&mut self) -> &mut RpcRequest {
        if let ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(RpcRequest::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rpcRequest(&mut self) -> RpcRequest {
        if self.has_rpcRequest() {
            match self.arg.take() {
                ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(v)) => v,
                _ => panic!(),
            }
        } else {
            RpcRequest::new()
        }
    }

    pub fn get_rpcRequest(&self) -> &RpcRequest {
        match self.arg {
            ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(ref v)) => v,
            _ => RpcRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for DaemonToDongle {
    fn is_initialized(&self) -> bool {
        if let Some(DaemonToDongle_oneof_arg::rpcRequest(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(DaemonToDongle_oneof_arg::rpcRequest(is.read_message()?));
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
                &DaemonToDongle_oneof_arg::rpcRequest(ref v) => {
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
                &DaemonToDongle_oneof_arg::rpcRequest(ref v) => {
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

impl ::protobuf::MessageStatic for DaemonToDongle {
    fn new() -> DaemonToDongle {
        DaemonToDongle::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonToDongle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RpcRequest>(
                    "rpcRequest",
                    DaemonToDongle::has_rpcRequest,
                    DaemonToDongle::get_rpcRequest,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonToDongle>(
                    "DaemonToDongle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonToDongle {
    fn clear(&mut self) {
        self.clear_rpcRequest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonToDongle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonToDongle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DongleToDaemon {
    // message oneof groups
    arg: ::std::option::Option<DongleToDaemon_oneof_arg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DongleToDaemon {}

#[derive(Clone,PartialEq)]
pub enum DongleToDaemon_oneof_arg {
    rpcReply(RpcReply),
    receiveTransmission(ReceiveTransmission),
    debugMessage(DebugMessage),
}

impl DongleToDaemon {
    pub fn new() -> DongleToDaemon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DongleToDaemon {
        static mut instance: ::protobuf::lazy::Lazy<DongleToDaemon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DongleToDaemon,
        };
        unsafe {
            instance.get(DongleToDaemon::new)
        }
    }

    // optional .linkbot.dongle.RpcReply rpcReply = 1;

    pub fn clear_rpcReply(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_rpcReply(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_rpcReply(&mut self, v: RpcReply) {
        self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(v))
    }

    // Mutable pointer to the field.
    pub fn mut_rpcReply(&mut self) -> &mut RpcReply {
        if let ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(RpcReply::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_rpcReply(&mut self) -> RpcReply {
        if self.has_rpcReply() {
            match self.arg.take() {
                ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(v)) => v,
                _ => panic!(),
            }
        } else {
            RpcReply::new()
        }
    }

    pub fn get_rpcReply(&self) -> &RpcReply {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(ref v)) => v,
            _ => RpcReply::default_instance(),
        }
    }

    // optional .linkbot.dongle.ReceiveTransmission receiveTransmission = 2;

    pub fn clear_receiveTransmission(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_receiveTransmission(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_receiveTransmission(&mut self, v: ReceiveTransmission) {
        self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(v))
    }

    // Mutable pointer to the field.
    pub fn mut_receiveTransmission(&mut self) -> &mut ReceiveTransmission {
        if let ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(ReceiveTransmission::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_receiveTransmission(&mut self) -> ReceiveTransmission {
        if self.has_receiveTransmission() {
            match self.arg.take() {
                ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(v)) => v,
                _ => panic!(),
            }
        } else {
            ReceiveTransmission::new()
        }
    }

    pub fn get_receiveTransmission(&self) -> &ReceiveTransmission {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(ref v)) => v,
            _ => ReceiveTransmission::default_instance(),
        }
    }

    // optional .linkbot.dongle.DebugMessage debugMessage = 3;

    pub fn clear_debugMessage(&mut self) {
        self.arg = ::std::option::Option::None;
    }

    pub fn has_debugMessage(&self) -> bool {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_debugMessage(&mut self, v: DebugMessage) {
        self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(v))
    }

    // Mutable pointer to the field.
    pub fn mut_debugMessage(&mut self) -> &mut DebugMessage {
        if let ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(_)) = self.arg {
        } else {
            self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(DebugMessage::new()));
        }
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_debugMessage(&mut self) -> DebugMessage {
        if self.has_debugMessage() {
            match self.arg.take() {
                ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugMessage::new()
        }
    }

    pub fn get_debugMessage(&self) -> &DebugMessage {
        match self.arg {
            ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(ref v)) => v,
            _ => DebugMessage::default_instance(),
        }
    }
}

impl ::protobuf::Message for DongleToDaemon {
    fn is_initialized(&self) -> bool {
        if let Some(DongleToDaemon_oneof_arg::rpcReply(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DongleToDaemon_oneof_arg::receiveTransmission(ref v)) = self.arg {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DongleToDaemon_oneof_arg::debugMessage(ref v)) = self.arg {
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
                    self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::rpcReply(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::receiveTransmission(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.arg = ::std::option::Option::Some(DongleToDaemon_oneof_arg::debugMessage(is.read_message()?));
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
                &DongleToDaemon_oneof_arg::rpcReply(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DongleToDaemon_oneof_arg::receiveTransmission(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DongleToDaemon_oneof_arg::debugMessage(ref v) => {
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
                &DongleToDaemon_oneof_arg::rpcReply(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DongleToDaemon_oneof_arg::receiveTransmission(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DongleToDaemon_oneof_arg::debugMessage(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DongleToDaemon {
    fn new() -> DongleToDaemon {
        DongleToDaemon::new()
    }

    fn descriptor_static(_: ::std::option::Option<DongleToDaemon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RpcReply>(
                    "rpcReply",
                    DongleToDaemon::has_rpcReply,
                    DongleToDaemon::get_rpcReply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ReceiveTransmission>(
                    "receiveTransmission",
                    DongleToDaemon::has_receiveTransmission,
                    DongleToDaemon::get_receiveTransmission,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugMessage>(
                    "debugMessage",
                    DongleToDaemon::has_debugMessage,
                    DongleToDaemon::get_debugMessage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DongleToDaemon>(
                    "DongleToDaemon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DongleToDaemon {
    fn clear(&mut self) {
        self.clear_rpcReply();
        self.clear_receiveTransmission();
        self.clear_debugMessage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DongleToDaemon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DongleToDaemon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RadioMode {
    FULL_DUPLEX = 0,
    TRANSMIT = 1,
    RECEIVE = 2,
}

impl ::protobuf::ProtobufEnum for RadioMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RadioMode> {
        match value {
            0 => ::std::option::Option::Some(RadioMode::FULL_DUPLEX),
            1 => ::std::option::Option::Some(RadioMode::TRANSMIT),
            2 => ::std::option::Option::Some(RadioMode::RECEIVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RadioMode] = &[
            RadioMode::FULL_DUPLEX,
            RadioMode::TRANSMIT,
            RadioMode::RECEIVE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RadioMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RadioMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RadioMode {
}

impl ::protobuf::reflect::ProtobufValue for RadioMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cdongle.proto\x12\x0elinkbot.dongle\x1a\x0cnanopb.proto\x1a\x11comm\
    ontypes.proto\x1a\x0bradio.proto\"E\n\x18getFirmwareVersionString\x1a\
    \x04\n\x02In\x1a#\n\x03Out\x12\x1c\n\x05value\x18\x01\x20\x01(\tR\x05val\
    ueB\x06\x92?\x03\x08\x80\x01\"\x15\n\x06reboot\x1a\x04\n\x02In\x1a\x05\n\
    \x03Out\"J\n\x0csetRadioMode\x1a3\n\x02In\x12-\n\x04mode\x18\x01\x20\x01\
    (\x0e2\x19.linkbot.dongle.RadioModeR\x04mode\x1a\x05\n\x03Out\"9\n\x0cse\
    tSessionId\x1a\"\n\x02In\x12\x1c\n\tsessionId\x18\x01\x20\x01(\rR\tsessi\
    onId\x1a\x05\n\x03Out\"\xd0\x01\n\x08transmit\x1a\x8c\x01\n\x02In\x123\n\
    \x0bdestination\x18\x01\x20\x01(\x0b2\x11.linkbot.SerialIdR\x0bdestinati\
    on\x12Q\n\x12routedRadioMessage\x18\x02\x20\x01(\x0b2!.linkbot.radio.Rou\
    tedRadioMessageR\x12routedRadioMessage\x1a5\n\x03Out\x12.\n\x12queuedMes\
    sageCount\x18\x01\x20\x01(\x05R\x12queuedMessageCount\"\x9a\x03\n\nRpcRe\
    quest\x12\x1c\n\trequestId\x18\x01\x20\x01(\rR\trequestId\x12i\n\x18getF\
    irmwareVersionString\x18\x02\x20\x01(\x0b2+.linkbot.dongle.getFirmwareVe\
    rsionString.InH\0R\x18getFirmwareVersionString\x123\n\x06reboot\x18\x03\
    \x20\x01(\x0b2\x19.linkbot.dongle.reboot.InH\0R\x06reboot\x12E\n\x0csetR\
    adioMode\x18\x04\x20\x01(\x0b2\x1f.linkbot.dongle.setRadioMode.InH\0R\
    \x0csetRadioMode\x12E\n\x0csetSessionId\x18\x05\x20\x01(\x0b2\x1f.linkbo\
    t.dongle.setSessionId.InH\0R\x0csetSessionId\x129\n\x08transmit\x18\x06\
    \x20\x01(\x0b2\x1b.linkbot.dongle.transmit.InH\0R\x08transmitB\x05\n\x03\
    arg\"\x9d\x03\n\x08RpcReply\x12\x1c\n\trequestId\x18\x01\x20\x01(\rR\tre\
    questId\x12j\n\x18getFirmwareVersionString\x18\x02\x20\x01(\x0b2,.linkbo\
    t.dongle.getFirmwareVersionString.OutH\0R\x18getFirmwareVersionString\
    \x124\n\x06reboot\x18\x03\x20\x01(\x0b2\x1a.linkbot.dongle.reboot.OutH\0\
    R\x06reboot\x12F\n\x0csetRadioMode\x18\x04\x20\x01(\x0b2\x20.linkbot.don\
    gle.setRadioMode.OutH\0R\x0csetRadioMode\x12F\n\x0csetSessionId\x18\x05\
    \x20\x01(\x0b2\x20.linkbot.dongle.setSessionId.OutH\0R\x0csetSessionId\
    \x12:\n\x08transmit\x18\x06\x20\x01(\x0b2\x1c.linkbot.dongle.transmit.Ou\
    tH\0R\x08transmitB\x05\n\x03arg\"\x93\x01\n\x13ReceiveTransmission\x12)\
    \n\x06source\x18\x01\x20\x01(\x0b2\x11.linkbot.SerialIdR\x06source\x12Q\
    \n\x12routedRadioMessage\x18\x02\x20\x01(\x0b2!.linkbot.radio.RoutedRadi\
    oMessageR\x12routedRadioMessage\",\n\x0cDebugMessage\x12\x1c\n\x05value\
    \x18\x01\x20\x01(\tR\x05valueB\x06\x92?\x03\x08\x80\x01\"U\n\x0eDaemonTo\
    Dongle\x12<\n\nrpcRequest\x18\x01\x20\x01(\x0b2\x1a.linkbot.dongle.RpcRe\
    questH\0R\nrpcRequestB\x05\n\x03arg\"\xec\x01\n\x0eDongleToDaemon\x126\n\
    \x08rpcReply\x18\x01\x20\x01(\x0b2\x18.linkbot.dongle.RpcReplyH\0R\x08rp\
    cReply\x12W\n\x13receiveTransmission\x18\x02\x20\x01(\x0b2#.linkbot.dong\
    le.ReceiveTransmissionH\0R\x13receiveTransmission\x12B\n\x0cdebugMessage\
    \x18\x03\x20\x01(\x0b2\x1c.linkbot.dongle.DebugMessageH\0R\x0cdebugMessa\
    geB\x05\n\x03arg*7\n\tRadioMode\x12\x0f\n\x0bFULL_DUPLEX\x10\0\x12\x0c\n\
    \x08TRANSMIT\x10\x01\x12\x0b\n\x07RECEIVE\x10\x02\
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
