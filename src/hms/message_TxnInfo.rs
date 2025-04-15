
impl ::std::default::Default for TxnInfo {
    fn default() -> Self {
        TxnInfo {
            id: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            user: ::std::default::Default::default(),
            hostname: ::std::default::Default::default(),
            agent_info: Some(::pilota::FastStr::from_static_str("Unknown")),
            heartbeat_count: Some(0i32),
            meta_info: ::std::default::Default::default(),
            started_time: ::std::default::Default::default(),
            last_heartbeat_time: ::std::default::Default::default(),
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub struct TxnInfo {
    pub id: i64,

    pub state: TxnState,

    pub user: ::pilota::FastStr,

    pub hostname: ::pilota::FastStr,

    pub agent_info: ::std::option::Option<::pilota::FastStr>,

    pub heartbeat_count: ::std::option::Option<i32>,

    pub meta_info: ::std::option::Option<::pilota::FastStr>,

    pub started_time: ::std::option::Option<i64>,

    pub last_heartbeat_time: ::std::option::Option<i64>,
}
impl ::pilota::thrift::Message for TxnInfo {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TxnInfo" };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_i64_field(1, *&self.id)?;
        __protocol.write_i32_field(2, (&self.state).inner())?;
        __protocol.write_faststr_field(3, (&self.user).clone())?;
        __protocol.write_faststr_field(4, (&self.hostname).clone())?;
        if let Some(value) = self.agent_info.as_ref() {
            __protocol.write_faststr_field(5, (value).clone())?;
        }
        if let Some(value) = self.heartbeat_count.as_ref() {
            __protocol.write_i32_field(6, *value)?;
        }
        if let Some(value) = self.meta_info.as_ref() {
            __protocol.write_faststr_field(7, (value).clone())?;
        }
        if let Some(value) = self.started_time.as_ref() {
            __protocol.write_i64_field(8, *value)?;
        }
        if let Some(value) = self.last_heartbeat_time.as_ref() {
            __protocol.write_i64_field(9, *value)?;
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
        let mut var_5 = Some(::pilota::FastStr::from_static_str("Unknown"));
        let mut var_6 = Some(0i32);
        let mut var_7 = None;
        let mut var_8 = None;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_1 = Some(__protocol.read_i64()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_3 = Some(__protocol.read_faststr()?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_4 = Some(__protocol.read_faststr()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_5 = Some(__protocol.read_faststr()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_6 = Some(__protocol.read_i32()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_7 = Some(__protocol.read_faststr()?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_8 = Some(__protocol.read_i64()?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_9 = Some(__protocol.read_i64()?);
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
                    "decode struct `TxnInfo` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field id is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field state is required".to_string(),
            ));
        };
        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field user is required".to_string(),
            ));
        };
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field hostname is required".to_string(),
            ));
        };

        let data = Self {
            id: var_1,
            state: var_2,
            user: var_3,
            hostname: var_4,
            agent_info: var_5,
            heartbeat_count: var_6,
            meta_info: var_7,
            started_time: var_8,
            last_heartbeat_time: var_9,
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
            let mut var_5 = Some(::pilota::FastStr::from_static_str("Unknown"));
            let mut var_6 = Some(0i32);
            let mut var_7 = None;
            let mut var_8 = None;
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_1 = Some(__protocol.read_i64().await?);
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_2 = Some(
                                <TxnState as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_3 = Some(__protocol.read_faststr().await?);
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_4 = Some(__protocol.read_faststr().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_5 = Some(__protocol.read_faststr().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_6 = Some(__protocol.read_i32().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_7 = Some(__protocol.read_faststr().await?);
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_8 = Some(__protocol.read_i64().await?);
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_9 = Some(__protocol.read_i64().await?);
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
                        "decode struct `TxnInfo` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field id is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field state is required".to_string(),
                ));
            };
            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field user is required".to_string(),
                ));
            };
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field hostname is required".to_string(),
                ));
            };

            let data = Self {
                id: var_1,
                state: var_2,
                user: var_3,
                hostname: var_4,
                agent_info: var_5,
                heartbeat_count: var_6,
                meta_info: var_7,
                started_time: var_8,
                last_heartbeat_time: var_9,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TxnInfo" })
            + __protocol.i64_field_len(Some(1), *&self.id)
            + __protocol.i32_field_len(Some(2), (&self.state).inner())
            + __protocol.faststr_field_len(Some(3), &self.user)
            + __protocol.faststr_field_len(Some(4), &self.hostname)
            + self
                .agent_info
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(5), value))
            + self
                .heartbeat_count
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(6), *value))
            + self
                .meta_info
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(7), value))
            + self
                .started_time
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(8), *value))
            + self
                .last_heartbeat_time
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(9), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
