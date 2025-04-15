
impl ::std::default::Default for AddDynamicPartitions {
    fn default() -> Self {
        AddDynamicPartitions {
            txnid: ::std::default::Default::default(),
            dbname: ::std::default::Default::default(),
            tablename: ::std::default::Default::default(),
            partitionnames: ::std::default::Default::default(),
            operation_type: Some(DataOperationType::UNSET),
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub struct AddDynamicPartitions {
    pub txnid: i64,

    pub dbname: ::pilota::FastStr,

    pub tablename: ::pilota::FastStr,

    pub partitionnames: ::std::vec::Vec<::pilota::FastStr>,

    pub operation_type: ::std::option::Option<DataOperationType>,
}
impl ::pilota::thrift::Message for AddDynamicPartitions {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "AddDynamicPartitions",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_i64_field(1, *&self.txnid)?;
        __protocol.write_faststr_field(2, (&self.dbname).clone())?;
        __protocol.write_faststr_field(3, (&self.tablename).clone())?;
        __protocol.write_list_field(
            4,
            ::pilota::thrift::TType::Binary,
            &&self.partitionnames,
            |__protocol, val| {
                __protocol.write_faststr((val).clone())?;
                ::std::result::Result::Ok(())
            },
        )?;
        if let Some(value) = self.operation_type.as_ref() {
            __protocol.write_i32_field(5, (value).inner())?;
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
        let mut var_5 = Some(DataOperationType::UNSET);

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
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_4 = Some(unsafe {
                            let list_ident = __protocol.read_list_begin()?;
                            let mut val: ::std::vec::Vec<::pilota::FastStr> =
                                ::std::vec::Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr()
                                    .offset(i as isize)
                                    .write(__protocol.read_faststr()?);
                            }
                            val.set_len(list_ident.size);
                            __protocol.read_list_end()?;
                            val
                        });
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_5 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                    "decode struct `AddDynamicPartitions` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_1) = var_1 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field txnid is required".to_string(),
            ));
        };
        let Some(var_2) = var_2 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field dbname is required".to_string(),
            ));
        };
        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field tablename is required".to_string(),
            ));
        };
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field partitionnames is required".to_string(),
            ));
        };

        let data = Self {
            txnid: var_1,
            dbname: var_2,
            tablename: var_3,
            partitionnames: var_4,
            operation_type: var_5,
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
            let mut var_5 = Some(DataOperationType::UNSET);

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
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            var_4 = Some({
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(__protocol.read_faststr().await?);
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_5 = Some(
                                <DataOperationType as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
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
                        "decode struct `AddDynamicPartitions` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_1) = var_1 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field txnid is required".to_string(),
                ));
            };
            let Some(var_2) = var_2 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field dbname is required".to_string(),
                ));
            };
            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field tablename is required".to_string(),
                ));
            };
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field partitionnames is required".to_string(),
                ));
            };

            let data = Self {
                txnid: var_1,
                dbname: var_2,
                tablename: var_3,
                partitionnames: var_4,
                operation_type: var_5,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "AddDynamicPartitions",
        }) + __protocol.i64_field_len(Some(1), *&self.txnid)
            + __protocol.faststr_field_len(Some(2), &self.dbname)
            + __protocol.faststr_field_len(Some(3), &self.tablename)
            + __protocol.list_field_len(
                Some(4),
                ::pilota::thrift::TType::Binary,
                &self.partitionnames,
                |__protocol, el| __protocol.faststr_len(el),
            )
            + self.operation_type.as_ref().map_or(0, |value| {
                __protocol.i32_field_len(Some(5), (value).inner())
            })
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
