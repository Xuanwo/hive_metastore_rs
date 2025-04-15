
impl ::std::default::Default for ShowLocksResponseElement {
    fn default() -> Self {
        ShowLocksResponseElement {
            lockid: ::std::default::Default::default(),
            dbname: ::std::default::Default::default(),
            tablename: ::std::default::Default::default(),
            partname: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            r#type: ::std::default::Default::default(),
            txnid: ::std::default::Default::default(),
            lastheartbeat: ::std::default::Default::default(),
            acquiredat: ::std::default::Default::default(),
            user: ::std::default::Default::default(),
            hostname: ::std::default::Default::default(),
            heartbeat_count: Some(0i32),
            agent_info: ::std::default::Default::default(),
            blocked_by_ext_id: ::std::default::Default::default(),
            blocked_by_int_id: ::std::default::Default::default(),
            lock_id_internal: ::std::default::Default::default(),
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub struct ShowLocksResponseElement {
    pub lockid: i64,

    pub dbname: ::pilota::FastStr,

    pub tablename: ::std::option::Option<::pilota::FastStr>,

    pub partname: ::std::option::Option<::pilota::FastStr>,

    pub state: LockState,

    pub r#type: LockType,

    pub txnid: ::std::option::Option<i64>,

    pub lastheartbeat: i64,

    pub acquiredat: ::std::option::Option<i64>,

    pub user: ::pilota::FastStr,

    pub hostname: ::pilota::FastStr,

    pub heartbeat_count: ::std::option::Option<i32>,

    pub agent_info: ::std::option::Option<::pilota::FastStr>,

    pub blocked_by_ext_id: ::std::option::Option<i64>,

    pub blocked_by_int_id: ::std::option::Option<i64>,

    pub lock_id_internal: ::std::option::Option<i64>,
}
impl ::pilota::thrift::Message for ShowLocksResponseElement {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "ShowLocksResponseElement",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_i64_field(1, *&self.lockid)?;
        __protocol.write_faststr_field(2, (&self.dbname).clone())?;
        if let Some(value) = self.tablename.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        if let Some(value) = self.partname.as_ref() {
            __protocol.write_faststr_field(4, (value).clone())?;
        }
        __protocol.write_i32_field(5, (&self.state).inner())?;
        __protocol.write_i32_field(6, (&self.r#type).inner())?;
        if let Some(value) = self.txnid.as_ref() {
            __protocol.write_i64_field(7, *value)?;
        }
        __protocol.write_i64_field(8, *&self.lastheartbeat)?;
        if let Some(value) = self.acquiredat.as_ref() {
            __protocol.write_i64_field(9, *value)?;
        }
        __protocol.write_faststr_field(10, (&self.user).clone())?;
        __protocol.write_faststr_field(11, (&self.hostname).clone())?;
        if let Some(value) = self.heartbeat_count.as_ref() {
            __protocol.write_i32_field(12, *value)?;
        }
        if let Some(value) = self.agent_info.as_ref() {
            __protocol.write_faststr_field(13, (value).clone())?;
        }
        if let Some(value) = self.blocked_by_ext_id.as_ref() {
            __protocol.write_i64_field(14, *value)?;
        }
        if let Some(value) = self.blocked_by_int_id.as_ref() {
            __protocol.write_i64_field(15, *value)?;
        }
        if let Some(value) = self.lock_id_internal.as_ref() {
            __protocol.write_i64_field(16, *value)?;
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
        let mut var_5 = None;
        let mut var_6 = None;
        let mut var_7 = None;
        let mut var_8 = None;
        let mut var_9 = None;
        let mut var_10 = None;
        let mut var_11 = None;
        let mut var_12 = Some(0i32);
        let mut var_13 = None;
        let mut var_14 = None;
        let mut var_15 = None;
        let mut var_16 = None;

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
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_2 = Some(__protocol.read_faststr()?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_3 = Some(__protocol.read_faststr()?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_4 = Some(__protocol.read_faststr()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_5 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_6 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_7 = Some(__protocol.read_i64()?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_8 = Some(__protocol.read_i64()?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_9 = Some(__protocol.read_i64()?);
                    }
                    Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_10 = Some(__protocol.read_faststr()?);
                    }
                    Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_11 = Some(__protocol.read_faststr()?);
                    }
                    Some(12) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_12 = Some(__protocol.read_i32()?);
                    }
                    Some(13) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_13 = Some(__protocol.read_faststr()?);
                    }
                    Some(14) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_14 = Some(__protocol.read_i64()?);
                    }
                    Some(15) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_15 = Some(__protocol.read_i64()?);
                    }
                    Some(16) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_16 = Some(__protocol.read_i64()?);
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
                    "decode struct `ShowLocksResponseElement` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field lockid is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field dbname is required".to_string(),
            ));
        };
        let Some(var_5) = var_5 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field state is required".to_string(),
            ));
        };
        let Some(var_6) = var_6 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field r#type is required".to_string(),
            ));
        };
        let Some(var_8) = var_8 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field lastheartbeat is required".to_string(),
            ));
        };
        let Some(var_10) = var_10 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field user is required".to_string(),
            ));
        };
        let Some(var_11) = var_11 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field hostname is required".to_string(),
            ));
        };

        let data = Self {
            lockid: var_1,
            dbname: var_2,
            tablename: var_3,
            partname: var_4,
            state: var_5,
            r#type: var_6,
            txnid: var_7,
            lastheartbeat: var_8,
            acquiredat: var_9,
            user: var_10,
            hostname: var_11,
            heartbeat_count: var_12,
            agent_info: var_13,
            blocked_by_ext_id: var_14,
            blocked_by_int_id: var_15,
            lock_id_internal: var_16,
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
            let mut var_5 = None;
            let mut var_6 = None;
            let mut var_7 = None;
            let mut var_8 = None;
            let mut var_9 = None;
            let mut var_10 = None;
            let mut var_11 = None;
            let mut var_12 = Some(0i32);
            let mut var_13 = None;
            let mut var_14 = None;
            let mut var_15 = None;
            let mut var_16 = None;

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
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_2 = Some(__protocol.read_faststr().await?);
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_3 = Some(__protocol.read_faststr().await?);
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_4 = Some(__protocol.read_faststr().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_5 = Some(
                                <LockState as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_6 = Some(
                                <LockType as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_7 = Some(__protocol.read_i64().await?);
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_8 = Some(__protocol.read_i64().await?);
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_9 = Some(__protocol.read_i64().await?);
                        }
                        Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_10 = Some(__protocol.read_faststr().await?);
                        }
                        Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_11 = Some(__protocol.read_faststr().await?);
                        }
                        Some(12) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_12 = Some(__protocol.read_i32().await?);
                        }
                        Some(13) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_13 = Some(__protocol.read_faststr().await?);
                        }
                        Some(14) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_14 = Some(__protocol.read_i64().await?);
                        }
                        Some(15) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_15 = Some(__protocol.read_i64().await?);
                        }
                        Some(16) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_16 = Some(__protocol.read_i64().await?);
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
                        "decode struct `ShowLocksResponseElement` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field lockid is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field dbname is required".to_string(),
                ));
            };
            let Some(var_5) = var_5 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field state is required".to_string(),
                ));
            };
            let Some(var_6) = var_6 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field r#type is required".to_string(),
                ));
            };
            let Some(var_8) = var_8 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field lastheartbeat is required".to_string(),
                ));
            };
            let Some(var_10) = var_10 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field user is required".to_string(),
                ));
            };
            let Some(var_11) = var_11 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field hostname is required".to_string(),
                ));
            };

            let data = Self {
                lockid: var_1,
                dbname: var_2,
                tablename: var_3,
                partname: var_4,
                state: var_5,
                r#type: var_6,
                txnid: var_7,
                lastheartbeat: var_8,
                acquiredat: var_9,
                user: var_10,
                hostname: var_11,
                heartbeat_count: var_12,
                agent_info: var_13,
                blocked_by_ext_id: var_14,
                blocked_by_int_id: var_15,
                lock_id_internal: var_16,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "ShowLocksResponseElement",
        }) + __protocol.i64_field_len(Some(1), *&self.lockid)
            + __protocol.faststr_field_len(Some(2), &self.dbname)
            + self
                .tablename
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + self
                .partname
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(4), value))
            + __protocol.i32_field_len(Some(5), (&self.state).inner())
            + __protocol.i32_field_len(Some(6), (&self.r#type).inner())
            + self
                .txnid
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(7), *value))
            + __protocol.i64_field_len(Some(8), *&self.lastheartbeat)
            + self
                .acquiredat
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(9), *value))
            + __protocol.faststr_field_len(Some(10), &self.user)
            + __protocol.faststr_field_len(Some(11), &self.hostname)
            + self
                .heartbeat_count
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(12), *value))
            + self
                .agent_info
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(13), value))
            + self
                .blocked_by_ext_id
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(14), *value))
            + self
                .blocked_by_int_id
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(15), *value))
            + self
                .lock_id_internal
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(16), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
