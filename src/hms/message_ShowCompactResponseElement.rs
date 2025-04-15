
impl ::std::default::Default for ShowCompactResponseElement {
    fn default() -> Self {
        ShowCompactResponseElement {
            dbname: ::std::default::Default::default(),
            tablename: ::std::default::Default::default(),
            partitionname: ::std::default::Default::default(),
            r#type: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            workerid: ::std::default::Default::default(),
            start: ::std::default::Default::default(),
            run_as: ::std::default::Default::default(),
            hightest_txn_id: ::std::default::Default::default(),
            meta_info: ::std::default::Default::default(),
            end_time: ::std::default::Default::default(),
            hadoop_job_id: Some(::pilota::FastStr::from_static_str("None")),
            id: ::std::default::Default::default(),
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub struct ShowCompactResponseElement {
    pub dbname: ::pilota::FastStr,

    pub tablename: ::pilota::FastStr,

    pub partitionname: ::std::option::Option<::pilota::FastStr>,

    pub r#type: CompactionType,

    pub state: ::pilota::FastStr,

    pub workerid: ::std::option::Option<::pilota::FastStr>,

    pub start: ::std::option::Option<i64>,

    pub run_as: ::std::option::Option<::pilota::FastStr>,

    pub hightest_txn_id: ::std::option::Option<i64>,

    pub meta_info: ::std::option::Option<::pilota::FastStr>,

    pub end_time: ::std::option::Option<i64>,

    pub hadoop_job_id: ::std::option::Option<::pilota::FastStr>,

    pub id: ::std::option::Option<i64>,
}
impl ::pilota::thrift::Message for ShowCompactResponseElement {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "ShowCompactResponseElement",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_faststr_field(1, (&self.dbname).clone())?;
        __protocol.write_faststr_field(2, (&self.tablename).clone())?;
        if let Some(value) = self.partitionname.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        __protocol.write_i32_field(4, (&self.r#type).inner())?;
        __protocol.write_faststr_field(5, (&self.state).clone())?;
        if let Some(value) = self.workerid.as_ref() {
            __protocol.write_faststr_field(6, (value).clone())?;
        }
        if let Some(value) = self.start.as_ref() {
            __protocol.write_i64_field(7, *value)?;
        }
        if let Some(value) = self.run_as.as_ref() {
            __protocol.write_faststr_field(8, (value).clone())?;
        }
        if let Some(value) = self.hightest_txn_id.as_ref() {
            __protocol.write_i64_field(9, *value)?;
        }
        if let Some(value) = self.meta_info.as_ref() {
            __protocol.write_faststr_field(10, (value).clone())?;
        }
        if let Some(value) = self.end_time.as_ref() {
            __protocol.write_i64_field(11, *value)?;
        }
        if let Some(value) = self.hadoop_job_id.as_ref() {
            __protocol.write_faststr_field(12, (value).clone())?;
        }
        if let Some(value) = self.id.as_ref() {
            __protocol.write_i64_field(13, *value)?;
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
        let mut var_12 = Some(::pilota::FastStr::from_static_str("None"));
        let mut var_13 = None;

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
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_3 = Some(__protocol.read_faststr()?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_4 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_5 = Some(__protocol.read_faststr()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_6 = Some(__protocol.read_faststr()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_7 = Some(__protocol.read_i64()?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_8 = Some(__protocol.read_faststr()?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_9 = Some(__protocol.read_i64()?);
                    }
                    Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_10 = Some(__protocol.read_faststr()?);
                    }
                    Some(11) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_11 = Some(__protocol.read_i64()?);
                    }
                    Some(12) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_12 = Some(__protocol.read_faststr()?);
                    }
                    Some(13) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_13 = Some(__protocol.read_i64()?);
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
                    "decode struct `ShowCompactResponseElement` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field dbname is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field tablename is required".to_string(),
            ));
        };
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field r#type is required".to_string(),
            ));
        };
        let Some(var_5) = var_5 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field state is required".to_string(),
            ));
        };

        let data = Self {
            dbname: var_1,
            tablename: var_2,
            partitionname: var_3,
            r#type: var_4,
            state: var_5,
            workerid: var_6,
            start: var_7,
            run_as: var_8,
            hightest_txn_id: var_9,
            meta_info: var_10,
            end_time: var_11,
            hadoop_job_id: var_12,
            id: var_13,
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
            let mut var_12 = Some(::pilota::FastStr::from_static_str("None"));
            let mut var_13 = None;

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
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_3 = Some(__protocol.read_faststr().await?);
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_4 = Some(
                                <CompactionType as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_5 = Some(__protocol.read_faststr().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_6 = Some(__protocol.read_faststr().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_7 = Some(__protocol.read_i64().await?);
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_8 = Some(__protocol.read_faststr().await?);
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_9 = Some(__protocol.read_i64().await?);
                        }
                        Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_10 = Some(__protocol.read_faststr().await?);
                        }
                        Some(11) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_11 = Some(__protocol.read_i64().await?);
                        }
                        Some(12) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_12 = Some(__protocol.read_faststr().await?);
                        }
                        Some(13) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_13 = Some(__protocol.read_i64().await?);
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
                        "decode struct `ShowCompactResponseElement` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field dbname is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field tablename is required".to_string(),
                ));
            };
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field r#type is required".to_string(),
                ));
            };
            let Some(var_5) = var_5 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field state is required".to_string(),
                ));
            };

            let data = Self {
                dbname: var_1,
                tablename: var_2,
                partitionname: var_3,
                r#type: var_4,
                state: var_5,
                workerid: var_6,
                start: var_7,
                run_as: var_8,
                hightest_txn_id: var_9,
                meta_info: var_10,
                end_time: var_11,
                hadoop_job_id: var_12,
                id: var_13,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "ShowCompactResponseElement",
        }) + __protocol.faststr_field_len(Some(1), &self.dbname)
            + __protocol.faststr_field_len(Some(2), &self.tablename)
            + self
                .partitionname
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + __protocol.i32_field_len(Some(4), (&self.r#type).inner())
            + __protocol.faststr_field_len(Some(5), &self.state)
            + self
                .workerid
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(6), value))
            + self
                .start
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(7), *value))
            + self
                .run_as
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(8), value))
            + self
                .hightest_txn_id
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(9), *value))
            + self
                .meta_info
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(10), value))
            + self
                .end_time
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(11), *value))
            + self
                .hadoop_job_id
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(12), value))
            + self
                .id
                .as_ref()
                .map_or(0, |value| __protocol.i64_field_len(Some(13), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
