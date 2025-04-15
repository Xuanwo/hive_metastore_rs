
impl ::std::default::Default for DropPartitionsRequest {
    fn default() -> Self {
        DropPartitionsRequest {
            db_name: ::std::default::Default::default(),
            tbl_name: ::std::default::Default::default(),
            parts: ::std::default::Default::default(),
            delete_data: ::std::default::Default::default(),
            if_exists: Some(true),
            ignore_protection: ::std::default::Default::default(),
            environment_context: ::std::default::Default::default(),
            need_result: Some(true),
            cat_name: ::std::default::Default::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DropPartitionsRequest {
    pub db_name: ::pilota::FastStr,

    pub tbl_name: ::pilota::FastStr,

    pub parts: RequestPartsSpec,

    pub delete_data: ::std::option::Option<bool>,

    pub if_exists: ::std::option::Option<bool>,

    pub ignore_protection: ::std::option::Option<bool>,

    pub environment_context: ::std::option::Option<EnvironmentContext>,

    pub need_result: ::std::option::Option<bool>,

    pub cat_name: ::std::option::Option<::pilota::FastStr>,
}
impl ::pilota::thrift::Message for DropPartitionsRequest {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "DropPartitionsRequest",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_faststr_field(1, (&self.db_name).clone())?;
        __protocol.write_faststr_field(2, (&self.tbl_name).clone())?;
        __protocol.write_struct_field(3, &self.parts, ::pilota::thrift::TType::Struct)?;
        if let Some(value) = self.delete_data.as_ref() {
            __protocol.write_bool_field(4, *value)?;
        }
        if let Some(value) = self.if_exists.as_ref() {
            __protocol.write_bool_field(5, *value)?;
        }
        if let Some(value) = self.ignore_protection.as_ref() {
            __protocol.write_bool_field(6, *value)?;
        }
        if let Some(value) = self.environment_context.as_ref() {
            __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.need_result.as_ref() {
            __protocol.write_bool_field(8, *value)?;
        }
        if let Some(value) = self.cat_name.as_ref() {
            __protocol.write_faststr_field(9, (value).clone())?;
        }
        __protocol.write_field_stop()?;
        __protocol.write_struct_end()?;
        ::std::result::Result::Ok(())
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        __protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::{thrift::TLengthProtocolExt, Buf};

        let mut var_1 = None;
        let mut var_2 = None;
        let mut var_3 = None;
        let mut var_4 = None;
        let mut var_5 = Some(true);
        let mut var_6 = None;
        let mut var_7 = None;
        let mut var_8 = Some(true);
        let mut var_9 = None;

        let mut __pilota_decoding_field_id = None;

        __protocol.read_struct_begin()?;
        if let ::std::result::Result::Err(mut err) = (|| {
            loop {
                let field_ident = __protocol.read_field_begin()?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    __protocol.field_stop_len();
                    break;
                } else {
                    __protocol.field_begin_len(field_ident.field_type, field_ident.id);
                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_1 = Some(__protocol.read_faststr()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_2 = Some(__protocol.read_faststr()?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_4 = Some(__protocol.read_bool()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_5 = Some(__protocol.read_bool()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_6 = Some(__protocol.read_bool()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_7 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_8 = Some(__protocol.read_bool()?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_9 = Some(__protocol.read_faststr()?);
                    }
                    _ => {
                        __protocol.skip(field_ident.field_type)?;
                    }
                }

                __protocol.read_field_end()?;
                __protocol.field_end_len();
            }
            ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
        })() {
            if let Some(field_id) = __pilota_decoding_field_id {
                err.prepend_msg(&format!(
                    "decode struct `DropPartitionsRequest` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field db_name is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field tbl_name is required".to_string(),
            ));
        };
        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field parts is required".to_string(),
            ));
        };

        let data = Self {
            db_name: var_1,
            tbl_name: var_2,
            parts: var_3,
            delete_data: var_4,
            if_exists: var_5,
            ignore_protection: var_6,
            environment_context: var_7,
            need_result: var_8,
            cat_name: var_9,
        };
        ::std::result::Result::Ok(data)
    }

    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
        __protocol: &'a mut T,
    ) -> ::std::pin::Pin<
        ::std::boxed::Box<
            dyn ::std::future::Future<
                    Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                > + Send
                + 'a,
        >,
    > {
        ::std::boxed::Box::pin(async move {
            let mut var_1 = None;
            let mut var_2 = None;
            let mut var_3 = None;
            let mut var_4 = None;
            let mut var_5 = Some(true);
            let mut var_6 = None;
            let mut var_7 = None;
            let mut var_8 = Some(true);
            let mut var_9 = None;

            let mut __pilota_decoding_field_id = None;

            __protocol.read_struct_begin().await?;
            if let ::std::result::Result::Err(mut err) = async {
                loop {
                    let field_ident = __protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    } else {
                    }
                    __pilota_decoding_field_id = field_ident.id;
                    match field_ident.id {
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_1 = Some(__protocol.read_faststr().await?);
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_2 = Some(__protocol.read_faststr().await?);
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_3 = Some(
                                <RequestPartsSpec as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_4 = Some(__protocol.read_bool().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_5 = Some(__protocol.read_bool().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_6 = Some(__protocol.read_bool().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_7 = Some(
                                <EnvironmentContext as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_8 = Some(__protocol.read_bool().await?);
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_9 = Some(__protocol.read_faststr().await?);
                        }
                        _ => {
                            __protocol.skip(field_ident.field_type).await?;
                        }
                    }

                    __protocol.read_field_end().await?;
                }
                ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
            }
            .await
            {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!(
                        "decode struct `DropPartitionsRequest` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field db_name is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field tbl_name is required".to_string(),
                ));
            };
            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field parts is required".to_string(),
                ));
            };

            let data = Self {
                db_name: var_1,
                tbl_name: var_2,
                parts: var_3,
                delete_data: var_4,
                if_exists: var_5,
                ignore_protection: var_6,
                environment_context: var_7,
                need_result: var_8,
                cat_name: var_9,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "DropPartitionsRequest",
        }) + __protocol.faststr_field_len(Some(1), &self.db_name)
            + __protocol.faststr_field_len(Some(2), &self.tbl_name)
            + __protocol.struct_field_len(Some(3), &self.parts)
            + self
                .delete_data
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(4), *value))
            + self
                .if_exists
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(5), *value))
            + self
                .ignore_protection
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(6), *value))
            + self
                .environment_context
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(7), value))
            + self
                .need_result
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(8), *value))
            + self
                .cat_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(9), value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
