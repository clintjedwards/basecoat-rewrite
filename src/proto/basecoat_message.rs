// This file is generated by rust-protobuf 2.27.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `basecoat_message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct Account {
    // message fields
    pub id: ::std::string::String,
    pub hash: ::std::vec::Vec<u8>,
    pub state: Account_State,
    pub created: i64,
    pub modified: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Account {
    fn default() -> &'a Account {
        <Account as ::protobuf::Message>::default_instance()
    }
}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // bytes hash = 2;


    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    // .proto.Account.State state = 3;


    pub fn get_state(&self) -> Account_State {
        self.state
    }
    pub fn clear_state(&mut self) {
        self.state = Account_State::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: Account_State) {
        self.state = v;
    }

    // int64 created = 4;


    pub fn get_created(&self) -> i64 {
        self.created
    }
    pub fn clear_created(&mut self) {
        self.created = 0;
    }

    // Param is passed by value, moved
    pub fn set_created(&mut self, v: i64) {
        self.created = v;
    }

    // int64 modified = 5;


    pub fn get_modified(&self) -> i64 {
        self.modified
    }
    pub fn clear_modified(&mut self) {
        self.modified = 0;
    }

    // Param is passed by value, moved
    pub fn set_modified(&mut self, v: i64) {
        self.modified = v;
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 3, &mut self.unknown_fields)?
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.created = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.modified = tmp;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        }
        if self.state != Account_State::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(3, self.state);
        }
        if self.created != 0 {
            my_size += ::protobuf::rt::value_size(4, self.created, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.modified != 0 {
            my_size += ::protobuf::rt::value_size(5, self.modified, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
        }
        if self.state != Account_State::UNKNOWN {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.state))?;
        }
        if self.created != 0 {
            os.write_int64(4, self.created)?;
        }
        if self.modified != 0 {
            os.write_int64(5, self.modified)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Account| { &m.id },
                |m: &mut Account| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "hash",
                |m: &Account| { &m.hash },
                |m: &mut Account| { &mut m.hash },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Account_State>>(
                "state",
                |m: &Account| { &m.state },
                |m: &mut Account| { &mut m.state },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "created",
                |m: &Account| { &m.created },
                |m: &mut Account| { &mut m.created },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "modified",
                |m: &Account| { &m.modified },
                |m: &mut Account| { &mut m.modified },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Account>(
                "Account",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Account {
        static instance: ::protobuf::rt::LazyV2<Account> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Account::new)
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.id.clear();
        self.hash.clear();
        self.state = Account_State::UNKNOWN;
        self.created = 0;
        self.modified = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Account_State {
    UNKNOWN = 0,
    ACTIVE = 1,
    DISABLED = 2,
}

impl ::protobuf::ProtobufEnum for Account_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Account_State> {
        match value {
            0 => ::std::option::Option::Some(Account_State::UNKNOWN),
            1 => ::std::option::Option::Some(Account_State::ACTIVE),
            2 => ::std::option::Option::Some(Account_State::DISABLED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Account_State] = &[
            Account_State::UNKNOWN,
            Account_State::ACTIVE,
            Account_State::DISABLED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Account_State>("Account.State", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Account_State {
}

impl ::std::default::Default for Account_State {
    fn default() -> Self {
        Account_State::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Account_State {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16basecoat_message.proto\x12\x05proto\"\xbf\x01\n\x07Account\x12\x0e\
    \n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x12\n\x04hash\x18\x02\x20\x01(\
    \x0cR\x04hash\x12*\n\x05state\x18\x03\x20\x01(\x0e2\x14.proto.Account.St\
    ateR\x05state\x12\x18\n\x07created\x18\x04\x20\x01(\x03R\x07created\x12\
    \x1a\n\x08modified\x18\x05\x20\x01(\x03R\x08modified\".\n\x05State\x12\
    \x0b\n\x07UNKNOWN\x10\0\x12\n\n\x06ACTIVE\x10\x01\x12\x0c\n\x08DISABLED\
    \x10\x02B&Z$github.com/clintjedwards/gofer/protob\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
