// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `msg.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foo.bar.Foo)
pub struct Foo {
    // message fields
    // @@protoc_insertion_point(field:foo.bar.Foo.first_field)
    pub first_field: i32,
    // @@protoc_insertion_point(field:foo.bar.Foo.second_field)
    pub second_field: ::std::string::String,
    // @@protoc_insertion_point(field:foo.bar.Foo.fourth_field)
    pub fourth_field: bool,
    // @@protoc_insertion_point(field:foo.bar.Foo.fifth_field)
    pub fifth_field: ::protobuf::EnumOrUnknown<EnFoo>,
    // special fields
    // @@protoc_insertion_point(special_field:foo.bar.Foo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Foo {
    fn default() -> &'a Foo {
        <Foo as ::protobuf::Message>::default_instance()
    }
}

impl Foo {
    pub fn new() -> Foo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "first_field",
            |m: &Foo| { &m.first_field },
            |m: &mut Foo| { &mut m.first_field },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "second_field",
            |m: &Foo| { &m.second_field },
            |m: &mut Foo| { &mut m.second_field },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fourth_field",
            |m: &Foo| { &m.fourth_field },
            |m: &mut Foo| { &mut m.fourth_field },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fifth_field",
            |m: &Foo| { &m.fifth_field },
            |m: &mut Foo| { &mut m.fifth_field },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Foo>(
            "Foo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Foo {
    const NAME: &'static str = "Foo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.first_field = is.read_int32()?;
                },
                18 => {
                    self.second_field = is.read_string()?;
                },
                32 => {
                    self.fourth_field = is.read_bool()?;
                },
                40 => {
                    self.fifth_field = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.first_field != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.first_field);
        }
        if !self.second_field.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.second_field);
        }
        if self.fourth_field != false {
            my_size += 1 + 1;
        }
        if self.fifth_field != ::protobuf::EnumOrUnknown::new(EnFoo::DEFAULT) {
            my_size += ::protobuf::rt::int32_size(5, self.fifth_field.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.first_field != 0 {
            os.write_int32(1, self.first_field)?;
        }
        if !self.second_field.is_empty() {
            os.write_string(2, &self.second_field)?;
        }
        if self.fourth_field != false {
            os.write_bool(4, self.fourth_field)?;
        }
        if self.fifth_field != ::protobuf::EnumOrUnknown::new(EnFoo::DEFAULT) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.fifth_field))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Foo {
        Foo::new()
    }

    fn clear(&mut self) {
        self.first_field = 0;
        self.second_field.clear();
        self.fourth_field = false;
        self.fifth_field = ::protobuf::EnumOrUnknown::new(EnFoo::DEFAULT);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Foo {
        static instance: Foo = Foo {
            first_field: 0,
            second_field: ::std::string::String::new(),
            fourth_field: false,
            fifth_field: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Foo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Foo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Foo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Foo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:foo.bar.EnFoo)
pub enum EnFoo {
    // @@protoc_insertion_point(enum_value:foo.bar.EnFoo.DEFAULT)
    DEFAULT = 0,
    // @@protoc_insertion_point(enum_value:foo.bar.EnFoo.FIRST)
    FIRST = 1,
    // @@protoc_insertion_point(enum_value:foo.bar.EnFoo.SECOND)
    SECOND = 2,
}

impl ::protobuf::Enum for EnFoo {
    const NAME: &'static str = "EnFoo";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnFoo> {
        match value {
            0 => ::std::option::Option::Some(EnFoo::DEFAULT),
            1 => ::std::option::Option::Some(EnFoo::FIRST),
            2 => ::std::option::Option::Some(EnFoo::SECOND),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EnFoo] = &[
        EnFoo::DEFAULT,
        EnFoo::FIRST,
        EnFoo::SECOND,
    ];
}

impl ::protobuf::EnumFull for EnFoo {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EnFoo").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EnFoo {
    fn default() -> Self {
        EnFoo::DEFAULT
    }
}

impl EnFoo {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EnFoo>("EnFoo")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tmsg.proto\x12\x07foo.bar\"\x9d\x01\n\x03Foo\x12\x1f\n\x0bfirst_field\
    \x18\x01\x20\x01(\x05R\nfirstField\x12!\n\x0csecond_field\x18\x02\x20\
    \x01(\tR\x0bsecondField\x12!\n\x0cfourth_field\x18\x04\x20\x01(\x08R\x0b\
    fourthField\x12/\n\x0bfifth_field\x18\x05\x20\x01(\x0e2\x0e.foo.bar.EnFo\
    oR\nfifthField*+\n\x05EnFoo\x12\x0b\n\x07DEFAULT\x10\0\x12\t\n\x05FIRST\
    \x10\x01\x12\n\n\x06SECOND\x10\x0222\n\nFooService\x12$\n\x04FooT\x12\
    \x0c.foo.bar.Foo\x1a\x0c.foo.bar.Foo\"\0B\x15\n\x13com.tencent.foo.barJ\
    \x87\x04\n\x06\x12\x04\0\0\x15\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\
    \n\x01\x02\x12\x03\x02\0\x10\n\x08\n\x01\x08\x12\x03\x04\0,\n\t\n\x02\
    \x08\x01\x12\x03\x04\0,\n\n\n\x02\x05\0\x12\x04\x06\0\n\x01\n\n\n\x03\
    \x05\0\x01\x12\x03\x06\x05\n\n\x0b\n\x04\x05\0\x02\0\x12\x03\x07\x02\x0e\
    \n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x07\x02\t\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x07\x0c\r\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x08\x02\x0c\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x08\x02\x07\n\x0c\n\x05\x05\0\x02\
    \x01\x02\x12\x03\x08\n\x0b\n\x0b\n\x04\x05\0\x02\x02\x12\x03\t\x02\r\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03\t\x02\x08\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03\t\x0b\x0c\n\n\n\x02\x04\0\x12\x04\x0c\0\x11\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x0c\x08\x0b\n\x0b\n\x04\x04\0\x02\0\x12\x03\r\x02\x18\
    \n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\r\x08\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x16\x17\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\x02\x1a\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\t\x15\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0e\x18\x19\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03\x0f\x02\x18\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0f\x02\
    \x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0f\x07\x13\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\x0f\x16\x17\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x10\
    \x02\x18\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x10\x02\x07\n\x0c\n\x05\
    \x04\0\x02\x03\x01\x12\x03\x10\x08\x13\n\x0c\n\x05\x04\0\x02\x03\x03\x12\
    \x03\x10\x16\x17\n\n\n\x02\x06\0\x12\x04\x13\0\x15\x01\n\n\n\x03\x06\0\
    \x01\x12\x03\x13\x08\x12\n\x0b\n\x04\x06\0\x02\0\x12\x03\x14\x02\x20\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x14\x06\n\n\x0c\n\x05\x06\0\x02\0\x02\
    \x12\x03\x14\x0b\x0e\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x14\x19\x1cb\
    \x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Foo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(EnFoo::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
