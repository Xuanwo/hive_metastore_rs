
impl ::std::default::Default for ThriftHiveMetastoreGetTableNamesByFilterResultRecv {
    fn default() -> Self {
        ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(::std::default::Default::default())
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub enum ThriftHiveMetastoreGetTableNamesByFilterResultRecv {
    Ok(::std::vec::Vec<::pilota::FastStr>),

    O1(MetaException),

    O2(InvalidOperationException),

    O3(UnknownDbException),
}

impl ::pilota::thrift::Message for ThriftHiveMetastoreGetTableNamesByFilterResultRecv {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreGetTableNamesByFilterResultRecv",
        })?;
        match self {
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(value) => {
                __protocol.write_list_field(
                    0,
                    ::pilota::thrift::TType::Binary,
                    &value,
                    |__protocol, val| {
                        __protocol.write_faststr((val).clone())?;
                        ::std::result::Result::Ok(())
                    },
                )?;
            }
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(value) => {
                __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
            }
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(value) => {
                __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
            }
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(value) => {
                __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
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
                        let field_ident = unsafe {
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
                        };
                        __protocol.list_len(
                            ::pilota::thrift::TType::Binary,
                            &field_ident,
                            |__protocol, el| __protocol.faststr_len(el),
                        );
                        ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(
                            field_ident,
                        ));
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
                        ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(
                            field_ident,
                        ));
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
                        ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(
                            field_ident,
                        ));
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
                        ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(
                            field_ident,
                        ));
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
                            let field_ident = {
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(__protocol.read_faststr().await?);
                                }
                                __protocol.read_list_end().await?;
                                val
                            };

                            ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(
                                field_ident,
                            ));
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
                                <MetaException as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(
                                field_ident,
                            ));
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
                            let field_ident = <InvalidOperationException as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                            ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(
                                field_ident,
                            ));
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
                                <UnknownDbException as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(
                                field_ident,
                            ));
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
            name: "ThriftHiveMetastoreGetTableNamesByFilterResultRecv",
        }) + match self {
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(value) => __protocol
                .list_field_len(
                    Some(0),
                    ::pilota::thrift::TType::Binary,
                    value,
                    |__protocol, el| __protocol.faststr_len(el),
                ),
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(value) => {
                __protocol.struct_field_len(Some(1), value)
            }
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(value) => {
                __protocol.struct_field_len(Some(2), value)
            }
            ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(value) => {
                __protocol.struct_field_len(Some(3), value)
            }
        } + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
