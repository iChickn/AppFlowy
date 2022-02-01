// This file is generated by rust-protobuf 2.22.1. Do not edit
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
//! Generated file from `user_setting.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UserPreferences {
    // message fields
    pub user_id: ::std::string::String,
    pub appearance_setting: ::protobuf::SingularPtrField<AppearanceSettings>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserPreferences {
    fn default() -> &'a UserPreferences {
        <UserPreferences as ::protobuf::Message>::default_instance()
    }
}

impl UserPreferences {
    pub fn new() -> UserPreferences {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }

    // .AppearanceSettings appearance_setting = 2;


    pub fn get_appearance_setting(&self) -> &AppearanceSettings {
        self.appearance_setting.as_ref().unwrap_or_else(|| <AppearanceSettings as ::protobuf::Message>::default_instance())
    }
    pub fn clear_appearance_setting(&mut self) {
        self.appearance_setting.clear();
    }

    pub fn has_appearance_setting(&self) -> bool {
        self.appearance_setting.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appearance_setting(&mut self, v: AppearanceSettings) {
        self.appearance_setting = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_appearance_setting(&mut self) -> &mut AppearanceSettings {
        if self.appearance_setting.is_none() {
            self.appearance_setting.set_default();
        }
        self.appearance_setting.as_mut().unwrap()
    }

    // Take field
    pub fn take_appearance_setting(&mut self) -> AppearanceSettings {
        self.appearance_setting.take().unwrap_or_else(|| AppearanceSettings::new())
    }
}

impl ::protobuf::Message for UserPreferences {
    fn is_initialized(&self) -> bool {
        for v in &self.appearance_setting {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.appearance_setting)?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if let Some(ref v) = self.appearance_setting.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if let Some(ref v) = self.appearance_setting.as_ref() {
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

    fn new() -> UserPreferences {
        UserPreferences::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &UserPreferences| { &m.user_id },
                |m: &mut UserPreferences| { &mut m.user_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AppearanceSettings>>(
                "appearance_setting",
                |m: &UserPreferences| { &m.appearance_setting },
                |m: &mut UserPreferences| { &mut m.appearance_setting },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserPreferences>(
                "UserPreferences",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserPreferences {
        static instance: ::protobuf::rt::LazyV2<UserPreferences> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserPreferences::new)
    }
}

impl ::protobuf::Clear for UserPreferences {
    fn clear(&mut self) {
        self.user_id.clear();
        self.appearance_setting.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserPreferences {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserPreferences {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppearanceSettings {
    // message fields
    pub theme: ::std::string::String,
    pub language: ::std::string::String,
    pub reset_as_default: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AppearanceSettings {
    fn default() -> &'a AppearanceSettings {
        <AppearanceSettings as ::protobuf::Message>::default_instance()
    }
}

impl AppearanceSettings {
    pub fn new() -> AppearanceSettings {
        ::std::default::Default::default()
    }

    // string theme = 1;


    pub fn get_theme(&self) -> &str {
        &self.theme
    }
    pub fn clear_theme(&mut self) {
        self.theme.clear();
    }

    // Param is passed by value, moved
    pub fn set_theme(&mut self, v: ::std::string::String) {
        self.theme = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_theme(&mut self) -> &mut ::std::string::String {
        &mut self.theme
    }

    // Take field
    pub fn take_theme(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.theme, ::std::string::String::new())
    }

    // string language = 2;


    pub fn get_language(&self) -> &str {
        &self.language
    }
    pub fn clear_language(&mut self) {
        self.language.clear();
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: ::std::string::String) {
        self.language = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language(&mut self) -> &mut ::std::string::String {
        &mut self.language
    }

    // Take field
    pub fn take_language(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.language, ::std::string::String::new())
    }

    // bool reset_as_default = 3;


    pub fn get_reset_as_default(&self) -> bool {
        self.reset_as_default
    }
    pub fn clear_reset_as_default(&mut self) {
        self.reset_as_default = false;
    }

    // Param is passed by value, moved
    pub fn set_reset_as_default(&mut self, v: bool) {
        self.reset_as_default = v;
    }
}

impl ::protobuf::Message for AppearanceSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.theme)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.language)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.reset_as_default = tmp;
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
        if !self.theme.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.theme);
        }
        if !self.language.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.language);
        }
        if self.reset_as_default != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.theme.is_empty() {
            os.write_string(1, &self.theme)?;
        }
        if !self.language.is_empty() {
            os.write_string(2, &self.language)?;
        }
        if self.reset_as_default != false {
            os.write_bool(3, self.reset_as_default)?;
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

    fn new() -> AppearanceSettings {
        AppearanceSettings::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "theme",
                |m: &AppearanceSettings| { &m.theme },
                |m: &mut AppearanceSettings| { &mut m.theme },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "language",
                |m: &AppearanceSettings| { &m.language },
                |m: &mut AppearanceSettings| { &mut m.language },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "reset_as_default",
                |m: &AppearanceSettings| { &m.reset_as_default },
                |m: &mut AppearanceSettings| { &mut m.reset_as_default },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AppearanceSettings>(
                "AppearanceSettings",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AppearanceSettings {
        static instance: ::protobuf::rt::LazyV2<AppearanceSettings> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AppearanceSettings::new)
    }
}

impl ::protobuf::Clear for AppearanceSettings {
    fn clear(&mut self) {
        self.theme.clear();
        self.language.clear();
        self.reset_as_default = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppearanceSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppearanceSettings {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12user_setting.proto\"n\n\x0fUserPreferences\x12\x17\n\x07user_id\
    \x18\x01\x20\x01(\tR\x06userId\x12B\n\x12appearance_setting\x18\x02\x20\
    \x01(\x0b2\x13.AppearanceSettingsR\x11appearanceSetting\"p\n\x12Appearan\
    ceSettings\x12\x14\n\x05theme\x18\x01\x20\x01(\tR\x05theme\x12\x1a\n\x08\
    language\x18\x02\x20\x01(\tR\x08language\x12(\n\x10reset_as_default\x18\
    \x03\x20\x01(\x08R\x0eresetAsDefaultJ\xd5\x02\n\x06\x12\x04\0\0\n\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x02\x08\x17\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\
    \x04\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x03\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\
    \x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04.\n\x0c\n\x05\x04\0\
    \x02\x01\x06\x12\x03\x04\x04\x16\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x04\x17)\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04,-\n\n\n\x02\x04\x01\
    \x12\x04\x06\0\n\x01\n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\x1a\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x07\x04\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03\x07\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x07\x0b\x10\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x07\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\
    \x12\x03\x08\x04\x18\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x08\x04\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x08\x0b\x13\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03\x08\x16\x17\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\t\
    \x04\x1e\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\t\x04\x08\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03\t\t\x19\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03\t\x1c\x1db\x06proto3\
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
