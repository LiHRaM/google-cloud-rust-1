// This file is generated by rust-protobuf 2.23.0. Do not edit
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
//! Generated file from `google/api/servicecontrol/v1/metric_value.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct MetricValue {
    // message fields
    pub labels: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub start_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub end_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // message oneof groups
    pub value: ::std::option::Option<MetricValue_oneof_value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MetricValue {
    fn default() -> &'a MetricValue {
        <MetricValue as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum MetricValue_oneof_value {
    bool_value(bool),
    int64_value(i64),
    double_value(f64),
    string_value(::std::string::String),
    distribution_value(super::distribution::Distribution),
}

impl MetricValue {
    pub fn new() -> MetricValue {
        ::std::default::Default::default()
    }

    // repeated .google.api.servicecontrol.v1.MetricValue.LabelsEntry labels = 1;


    pub fn get_labels(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.labels
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.labels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_labels(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.labels
    }

    // Take field
    pub fn take_labels(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.labels, ::std::collections::HashMap::new())
    }

    // .google.protobuf.Timestamp start_time = 2;


    pub fn get_start_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.start_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_start_time(&mut self) {
        self.start_time.clear();
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.start_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.start_time.is_none() {
            self.start_time.set_default();
        }
        self.start_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.start_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // .google.protobuf.Timestamp end_time = 3;


    pub fn get_end_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.end_time.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_end_time(&mut self) {
        self.end_time.clear();
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.end_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.end_time.is_none() {
            self.end_time.set_default();
        }
        self.end_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.end_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // bool bool_value = 4;


    pub fn get_bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::bool_value(v)) => v,
            _ => false,
        }
    }
    pub fn clear_bool_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(MetricValue_oneof_value::bool_value(v))
    }

    // int64 int64_value = 5;


    pub fn get_int64_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::int64_value(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_int64_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int64_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::int64_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int64_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(MetricValue_oneof_value::int64_value(v))
    }

    // double double_value = 6;


    pub fn get_double_value(&self) -> f64 {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::double_value(v)) => v,
            _ => 0.,
        }
    }
    pub fn clear_double_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::double_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.value = ::std::option::Option::Some(MetricValue_oneof_value::double_value(v))
    }

    // string string_value = 7;


    pub fn get_string_value(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::string_value(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_string_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(MetricValue_oneof_value::string_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(MetricValue_oneof_value::string_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(MetricValue_oneof_value::string_value(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.value.take() {
                ::std::option::Option::Some(MetricValue_oneof_value::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .google.api.servicecontrol.v1.Distribution distribution_value = 8;


    pub fn get_distribution_value(&self) -> &super::distribution::Distribution {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(ref v)) => v,
            _ => <super::distribution::Distribution as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_distribution_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_distribution_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_distribution_value(&mut self, v: super::distribution::Distribution) {
        self.value = ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_distribution_value(&mut self) -> &mut super::distribution::Distribution {
        if let ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(super::distribution::Distribution::new()));
        }
        match self.value {
            ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_distribution_value(&mut self) -> super::distribution::Distribution {
        if self.has_distribution_value() {
            match self.value.take() {
                ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(v)) => v,
                _ => panic!(),
            }
        } else {
            super::distribution::Distribution::new()
        }
    }
}

impl ::protobuf::Message for MetricValue {
    fn is_initialized(&self) -> bool {
        for v in &self.start_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end_time {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(MetricValue_oneof_value::distribution_value(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.labels)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_time)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_time)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(MetricValue_oneof_value::bool_value(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(MetricValue_oneof_value::int64_value(is.read_int64()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(MetricValue_oneof_value::double_value(is.read_double()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(MetricValue_oneof_value::string_value(is.read_string()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(MetricValue_oneof_value::distribution_value(is.read_message()?));
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.labels);
        if let Some(ref v) = self.start_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &MetricValue_oneof_value::bool_value(v) => {
                    my_size += 2;
                },
                &MetricValue_oneof_value::int64_value(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &MetricValue_oneof_value::double_value(v) => {
                    my_size += 9;
                },
                &MetricValue_oneof_value::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(7, &v);
                },
                &MetricValue_oneof_value::distribution_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.labels, os)?;
        if let Some(ref v) = self.start_time.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &MetricValue_oneof_value::bool_value(v) => {
                    os.write_bool(4, v)?;
                },
                &MetricValue_oneof_value::int64_value(v) => {
                    os.write_int64(5, v)?;
                },
                &MetricValue_oneof_value::double_value(v) => {
                    os.write_double(6, v)?;
                },
                &MetricValue_oneof_value::string_value(ref v) => {
                    os.write_string(7, v)?;
                },
                &MetricValue_oneof_value::distribution_value(ref v) => {
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

    fn new() -> MetricValue {
        MetricValue::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "labels",
                |m: &MetricValue| { &m.labels },
                |m: &mut MetricValue| { &mut m.labels },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "start_time",
                |m: &MetricValue| { &m.start_time },
                |m: &mut MetricValue| { &mut m.start_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                "end_time",
                |m: &MetricValue| { &m.end_time },
                |m: &mut MetricValue| { &mut m.end_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "bool_value",
                MetricValue::has_bool_value,
                MetricValue::get_bool_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                "int64_value",
                MetricValue::has_int64_value,
                MetricValue::get_int64_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                "double_value",
                MetricValue::has_double_value,
                MetricValue::get_double_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "string_value",
                MetricValue::has_string_value,
                MetricValue::get_string_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::distribution::Distribution>(
                "distribution_value",
                MetricValue::has_distribution_value,
                MetricValue::get_distribution_value,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MetricValue>(
                "MetricValue",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MetricValue {
        static instance: ::protobuf::rt::LazyV2<MetricValue> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MetricValue::new)
    }
}

impl ::protobuf::Clear for MetricValue {
    fn clear(&mut self) {
        self.labels.clear();
        self.start_time.clear();
        self.end_time.clear();
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetricValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricValue {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetricValueSet {
    // message fields
    pub metric_name: ::std::string::String,
    pub metric_values: ::protobuf::RepeatedField<MetricValue>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MetricValueSet {
    fn default() -> &'a MetricValueSet {
        <MetricValueSet as ::protobuf::Message>::default_instance()
    }
}

impl MetricValueSet {
    pub fn new() -> MetricValueSet {
        ::std::default::Default::default()
    }

    // string metric_name = 1;


    pub fn get_metric_name(&self) -> &str {
        &self.metric_name
    }
    pub fn clear_metric_name(&mut self) {
        self.metric_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric_name(&mut self, v: ::std::string::String) {
        self.metric_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metric_name(&mut self) -> &mut ::std::string::String {
        &mut self.metric_name
    }

    // Take field
    pub fn take_metric_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metric_name, ::std::string::String::new())
    }

    // repeated .google.api.servicecontrol.v1.MetricValue metric_values = 2;


    pub fn get_metric_values(&self) -> &[MetricValue] {
        &self.metric_values
    }
    pub fn clear_metric_values(&mut self) {
        self.metric_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric_values(&mut self, v: ::protobuf::RepeatedField<MetricValue>) {
        self.metric_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metric_values(&mut self) -> &mut ::protobuf::RepeatedField<MetricValue> {
        &mut self.metric_values
    }

    // Take field
    pub fn take_metric_values(&mut self) -> ::protobuf::RepeatedField<MetricValue> {
        ::std::mem::replace(&mut self.metric_values, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for MetricValueSet {
    fn is_initialized(&self) -> bool {
        for v in &self.metric_values {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metric_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metric_values)?;
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
        if !self.metric_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.metric_name);
        }
        for value in &self.metric_values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.metric_name.is_empty() {
            os.write_string(1, &self.metric_name)?;
        }
        for v in &self.metric_values {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> MetricValueSet {
        MetricValueSet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "metric_name",
                |m: &MetricValueSet| { &m.metric_name },
                |m: &mut MetricValueSet| { &mut m.metric_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MetricValue>>(
                "metric_values",
                |m: &MetricValueSet| { &m.metric_values },
                |m: &mut MetricValueSet| { &mut m.metric_values },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MetricValueSet>(
                "MetricValueSet",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MetricValueSet {
        static instance: ::protobuf::rt::LazyV2<MetricValueSet> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MetricValueSet::new)
    }
}

impl ::protobuf::Clear for MetricValueSet {
    fn clear(&mut self) {
        self.metric_name.clear();
        self.metric_values.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetricValueSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricValueSet {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/google/api/servicecontrol/v1/metric_value.proto\x12\x1cgoogle.api.ser\
    vicecontrol.v1\x1a\x1cgoogle/api/annotations.proto\x1a/google/api/servic\
    econtrol/v1/distribution.proto\x1a\x1fgoogle/protobuf/timestamp.proto\
    \x1a\x17google/type/money.proto\"\xfd\x03\n\x0bMetricValue\x12M\n\x06lab\
    els\x18\x01\x20\x03(\x0b25.google.api.servicecontrol.v1.MetricValue.Labe\
    lsEntryR\x06labels\x129\n\nstart_time\x18\x02\x20\x01(\x0b2\x1a.google.p\
    rotobuf.TimestampR\tstartTime\x125\n\x08end_time\x18\x03\x20\x01(\x0b2\
    \x1a.google.protobuf.TimestampR\x07endTime\x12\x1f\n\nbool_value\x18\x04\
    \x20\x01(\x08H\0R\tboolValue\x12!\n\x0bint64_value\x18\x05\x20\x01(\x03H\
    \0R\nint64Value\x12#\n\x0cdouble_value\x18\x06\x20\x01(\x01H\0R\x0bdoubl\
    eValue\x12#\n\x0cstring_value\x18\x07\x20\x01(\tH\0R\x0bstringValue\x12[\
    \n\x12distribution_value\x18\x08\x20\x01(\x0b2*.google.api.servicecontro\
    l.v1.DistributionH\0R\x11distributionValue\x1a9\n\x0bLabelsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value:\x028\x01B\x07\n\x05value\"\x81\x01\n\x0eMetricValueSet\x12\
    \x1f\n\x0bmetric_name\x18\x01\x20\x01(\tR\nmetricName\x12N\n\rmetric_val\
    ues\x18\x02\x20\x03(\x0b2).google.api.servicecontrol.v1.MetricValueR\x0c\
    metricValuesB\x88\x01\n\x20com.google.api.servicecontrol.v1B\x13MetricVa\
    lueSetProtoP\x01ZJgoogle.golang.org/genproto/googleapis/api/servicecontr\
    ol/v1;servicecontrol\xf8\x01\x01J\x9c\x14\n\x06\x12\x04\x0e\0M\x01\n\xbd\
    \x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\
    \x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Versio\
    n\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0%\n\t\n\x02\x03\0\
    \x12\x03\x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\09\n\t\n\x02\x03\x02\x12\
    \x03\x14\0)\n\t\n\x02\x03\x03\x12\x03\x15\0!\n\x08\n\x01\x08\x12\x03\x17\
    \0\x1f\n\t\n\x02\x08\x1f\x12\x03\x17\0\x1f\n\x08\n\x01\x08\x12\x03\x18\0\
    a\n\t\n\x02\x08\x0b\x12\x03\x18\0a\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\
    \x02\x08\n\x12\x03\x19\0\"\n\x08\n\x01\x08\x12\x03\x1a\04\n\t\n\x02\x08\
    \x08\x12\x03\x1a\04\n\x08\n\x01\x08\x12\x03\x1b\09\n\t\n\x02\x08\x01\x12\
    \x03\x1b\09\n/\n\x02\x04\0\x12\x04\x1e\0B\x01\x1a#\x20Represents\x20a\
    \x20single\x20metric\x20value.\n\n\n\n\x03\x04\0\x01\x12\x03\x1e\x08\x13\
    \n\xca\x01\n\x04\x04\0\x02\0\x12\x03#\x02!\x1a\xbc\x01\x20The\x20labels\
    \x20describing\x20the\x20metric\x20value.\n\x20See\x20comments\x20on\n\
    \x20[google.api.servicecontrol.v1.Operation.labels][google.api.serviceco\
    ntrol.v1.Operation.labels]\n\x20for\x20the\x20overriding\x20relationship\
    .\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04#\x02\x1e\x15\n\x0c\n\x05\x04\0\x02\
    \0\x06\x12\x03#\x02\x15\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03#\x16\x1c\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03#\x1f\x20\n\x98\x02\n\x04\x04\0\x02\
    \x01\x12\x03)\x02+\x1a\x8a\x02\x20The\x20start\x20of\x20the\x20time\x20p\
    eriod\x20over\x20which\x20this\x20metric\x20value's\x20measurement\n\x20\
    applies.\x20The\x20time\x20period\x20has\x20different\x20semantics\x20fo\
    r\x20different\x20metric\n\x20types\x20(cumulative,\x20delta,\x20and\x20\
    gauge).\x20See\x20the\x20metric\x20definition\n\x20documentation\x20in\
    \x20the\x20service\x20configuration\x20for\x20details.\n\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04)\x02#!\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03)\x02\
    \x1b\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03)\x1c&\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03))*\n^\n\x04\x04\0\x02\x02\x12\x03-\x02)\x1aQ\x20The\x20\
    end\x20of\x20the\x20time\x20period\x20over\x20which\x20this\x20metric\
    \x20value's\x20measurement\n\x20applies.\n\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04-\x02)+\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03-\x02\x1b\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03-\x1c$\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03-'(\n\xae\x01\n\x04\x04\0\x08\0\x12\x042\x02A\x03\x1a\x9f\x01\x20The\
    \x20value.\x20The\x20type\x20of\x20value\x20used\x20in\x20the\x20request\
    \x20must\n\x20agree\x20with\x20the\x20metric\x20definition\x20in\x20the\
    \x20service\x20configuration,\x20otherwise\n\x20the\x20MetricValue\x20is\
    \x20rejected.\n\n\x0c\n\x05\x04\0\x08\0\x01\x12\x032\x08\r\n\x1f\n\x04\
    \x04\0\x02\x03\x12\x034\x04\x18\x1a\x12\x20A\x20boolean\x20value.\n\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x034\x04\x08\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x034\t\x13\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x034\x16\x17\n-\n\
    \x04\x04\0\x02\x04\x12\x037\x04\x1a\x1a\x20\x20A\x20signed\x2064-bit\x20\
    integer\x20value.\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x037\x04\t\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x037\n\x15\n\x0c\n\x05\x04\0\x02\x04\x03\x12\
    \x037\x18\x19\n7\n\x04\x04\0\x02\x05\x12\x03:\x04\x1c\x1a*\x20A\x20doubl\
    e\x20precision\x20floating\x20point\x20value.\n\n\x0c\n\x05\x04\0\x02\
    \x05\x05\x12\x03:\x04\n\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03:\x0b\x17\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03:\x1a\x1b\n#\n\x04\x04\0\x02\x06\x12\
    \x03=\x04\x1c\x1a\x16\x20A\x20text\x20string\x20value.\n\n\x0c\n\x05\x04\
    \0\x02\x06\x05\x12\x03=\x04\n\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03=\x0b\
    \x17\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03=\x1a\x1b\n$\n\x04\x04\0\x02\
    \x07\x12\x03@\x04(\x1a\x17\x20A\x20distribution\x20value.\n\n\x0c\n\x05\
    \x04\0\x02\x07\x06\x12\x03@\x04\x10\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03@\x11#\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03@&'\n\xaf\x01\n\x02\x04\
    \x01\x12\x04G\0M\x01\x1a\xa2\x01\x20Represents\x20a\x20set\x20of\x20metr\
    ic\x20values\x20in\x20the\x20same\x20metric.\n\x20Each\x20metric\x20valu\
    e\x20in\x20the\x20set\x20should\x20have\x20a\x20unique\x20combination\
    \x20of\x20start\x20time,\n\x20end\x20time,\x20and\x20label\x20values.\n\
    \n\n\n\x03\x04\x01\x01\x12\x03G\x08\x16\nD\n\x04\x04\x01\x02\0\x12\x03I\
    \x02\x19\x1a7\x20The\x20metric\x20name\x20defined\x20in\x20the\x20servic\
    e\x20configuration.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04I\x02G\x18\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03I\x02\x08\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03I\t\x14\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03I\x17\x18\n)\n\
    \x04\x04\x01\x02\x01\x12\x03L\x02)\x1a\x1c\x20The\x20values\x20in\x20thi\
    s\x20metric.\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03L\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x01\x06\x12\x03L\x0b\x16\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03L\x17$\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03L'(b\x06proto3\
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