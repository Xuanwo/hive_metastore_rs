
impl ::std::default::Default for ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend {
    fn default() -> Self {
        ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(
            ::std::default::Default::default(),
        )
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub enum ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend {
    Ok(bool),

    O1(NoSuchObjectException),

    O2(InvalidObjectException),

    O3(MetaException),

    O4(InvalidInputException),
}

impl ::pilota::thrift::Message for ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend",
        })?;
        match self {
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(value) => {
                __protocol.write_bool_field(0, *value)?;
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O1(value) => {
                __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O2(value) => {
                __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O3(value) => {
                __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O4(value) => {
                __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
            }
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
        let mut ret = None;
        __protocol.read_struct_begin()?;
        loop {
            let field_ident = __protocol.read_field_begin()?;
            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                __protocol.field_stop_len();
                break;
            } else {
                __protocol.field_begin_len(field_ident.field_type, field_ident.id);
            }
            match field_ident.id {
                Some(0) => {
                    if ret.is_none() {
                        let field_ident = __protocol.read_bool()?;
                        __protocol.bool_len(*&field_ident);
                        ret = Some(
                            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(
                                field_ident,
                            ),
                        );
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(1) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(
                            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O1(
                                field_ident,
                            ),
                        );
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(2) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(
                            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O2(
                                field_ident,
                            ),
                        );
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(3) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(
                            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O3(
                                field_ident,
                            ),
                        );
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(4) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(
                            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O4(
                                field_ident,
                            ),
                        );
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                _ => {
                    __protocol.skip(field_ident.field_type)?;
                }
            }
        }
        __protocol.read_field_end()?;
        __protocol.read_struct_end()?;
        if let Some(ret) = ret {
            ::std::result::Result::Ok(ret)
        } else {
            ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "received empty union from remote Message",
            ))
        }
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
            let mut ret = None;
            __protocol.read_struct_begin().await?;
            loop {
                let field_ident = __protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {
                    break;
                } else {
                }
                match field_ident.id {
                    Some(0) => {
                        if ret.is_none() {
                            let field_ident = __protocol.read_bool().await?;

                            ret = Some(
                                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(
                                    field_ident,
                                ),
                            );
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(1) => {
                        if ret.is_none() {
                            let field_ident =
                                <NoSuchObjectException as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(
                                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O1(
                                    field_ident,
                                ),
                            );
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(2) => {
                        if ret.is_none() {
                            let field_ident = <InvalidObjectException as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                            ret = Some(
                                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O2(
                                    field_ident,
                                ),
                            );
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(3) => {
                        if ret.is_none() {
                            let field_ident =
                                <MetaException as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(
                                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O3(
                                    field_ident,
                                ),
                            );
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(4) => {
                        if ret.is_none() {
                            let field_ident =
                                <InvalidInputException as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(
                                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O4(
                                    field_ident,
                                ),
                            );
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;
                    }
                }
            }
            __protocol.read_field_end().await?;
            __protocol.read_struct_end().await?;
            if let Some(ret) = ret {
                ::std::result::Result::Ok(ret)
            } else {
                ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "received empty union from remote Message",
                ))
            }
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend",
        }) + match self {
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(value) => {
                __protocol.bool_field_len(Some(0), *value)
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O1(value) => {
                __protocol.struct_field_len(Some(1), value)
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O2(value) => {
                __protocol.struct_field_len(Some(2), value)
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O3(value) => {
                __protocol.struct_field_len(Some(3), value)
            }
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O4(value) => {
                __protocol.struct_field_len(Some(4), value)
            }
        } + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
