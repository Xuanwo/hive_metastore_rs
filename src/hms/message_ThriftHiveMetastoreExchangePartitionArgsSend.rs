#[derive(Debug, Default, Clone, PartialEq)]
pub struct ThriftHiveMetastoreExchangePartitionArgsSend {
    pub partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,

    pub source_db: ::pilota::FastStr,

    pub source_table_name: ::pilota::FastStr,

    pub dest_db: ::pilota::FastStr,

    pub dest_table_name: ::pilota::FastStr,
}
impl ::pilota::thrift::Message for ThriftHiveMetastoreExchangePartitionArgsSend {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreExchangePartitionArgsSend",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_map_field(
            1,
            ::pilota::thrift::TType::Binary,
            ::pilota::thrift::TType::Binary,
            &&self.partition_specs,
            |__protocol, key| {
                __protocol.write_faststr((key).clone())?;
                ::std::result::Result::Ok(())
            },
            |__protocol, val| {
                __protocol.write_faststr((val).clone())?;
                ::std::result::Result::Ok(())
            },
        )?;
        __protocol.write_faststr_field(2, (&self.source_db).clone())?;
        __protocol.write_faststr_field(3, (&self.source_table_name).clone())?;
        __protocol.write_faststr_field(4, (&self.dest_db).clone())?;
        __protocol.write_faststr_field(5, (&self.dest_table_name).clone())?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_1 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, __protocol.read_faststr()?);
                            }
                            __protocol.read_map_end()?;
                            val
                        });
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
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_5 = Some(__protocol.read_faststr()?);
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
                err.prepend_msg(&format!("decode struct `ThriftHiveMetastoreExchangePartitionArgsSend` field(#{}) failed, caused by: ", field_id));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field partition_specs is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field source_db is required".to_string(),
            ));
        };
        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field source_table_name is required".to_string(),
            ));
        };
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field dest_db is required".to_string(),
            ));
        };
        let Some(var_5) = var_5 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field dest_table_name is required".to_string(),
            ));
        };

        let data = Self {
            partition_specs: var_1,
            source_db: var_2,
            source_table_name: var_3,
            dest_db: var_4,
            dest_table_name: var_5,
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            var_1 = Some({
                                let map_ident = __protocol.read_map_begin().await?;
                                let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                                for _ in 0..map_ident.size {
                                    val.insert(
                                        __protocol.read_faststr().await?,
                                        __protocol.read_faststr().await?,
                                    );
                                }
                                __protocol.read_map_end().await?;
                                val
                            });
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
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_5 = Some(__protocol.read_faststr().await?);
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
                    err.prepend_msg(&format!("decode struct `ThriftHiveMetastoreExchangePartitionArgsSend` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field partition_specs is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field source_db is required".to_string(),
                ));
            };
            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field source_table_name is required".to_string(),
                ));
            };
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field dest_db is required".to_string(),
                ));
            };
            let Some(var_5) = var_5 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field dest_table_name is required".to_string(),
                ));
            };

            let data = Self {
                partition_specs: var_1,
                source_db: var_2,
                source_table_name: var_3,
                dest_db: var_4,
                dest_table_name: var_5,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreExchangePartitionArgsSend",
        }) + __protocol.map_field_len(
            Some(1),
            ::pilota::thrift::TType::Binary,
            ::pilota::thrift::TType::Binary,
            &self.partition_specs,
            |__protocol, key| __protocol.faststr_len(key),
            |__protocol, val| __protocol.faststr_len(val),
        ) + __protocol.faststr_field_len(Some(2), &self.source_db)
            + __protocol.faststr_field_len(Some(3), &self.source_table_name)
            + __protocol.faststr_field_len(Some(4), &self.dest_db)
            + __protocol.faststr_field_len(Some(5), &self.dest_table_name)
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
