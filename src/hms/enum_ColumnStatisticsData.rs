
impl ::std::default::Default for ColumnStatisticsData {
    fn default() -> Self {
        ColumnStatisticsData::BooleanStats(::std::default::Default::default())
    }
}
#[derive(PartialOrd, Debug, Clone, PartialEq)]
pub enum ColumnStatisticsData {
    BooleanStats(BooleanColumnStatsData),

    LongStats(LongColumnStatsData),

    DoubleStats(DoubleColumnStatsData),

    StringStats(StringColumnStatsData),

    BinaryStats(BinaryColumnStatsData),

    DecimalStats(DecimalColumnStatsData),

    DateStats(DateColumnStatsData),
}

impl ::pilota::thrift::Message for ColumnStatisticsData {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
            name: "ColumnStatisticsData",
        })?;
        match self {
            ColumnStatisticsData::BooleanStats(value) => {
                __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::LongStats(value) => {
                __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::DoubleStats(value) => {
                __protocol.write_struct_field(3, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::StringStats(value) => {
                __protocol.write_struct_field(4, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::BinaryStats(value) => {
                __protocol.write_struct_field(5, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::DecimalStats(value) => {
                __protocol.write_struct_field(6, value, ::pilota::thrift::TType::Struct)?;
            }
            ColumnStatisticsData::DateStats(value) => {
                __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
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
                Some(1) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(ColumnStatisticsData::BooleanStats(field_ident));
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
                        ret = Some(ColumnStatisticsData::LongStats(field_ident));
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
                        ret = Some(ColumnStatisticsData::DoubleStats(field_ident));
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
                        ret = Some(ColumnStatisticsData::StringStats(field_ident));
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(5) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(ColumnStatisticsData::BinaryStats(field_ident));
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(6) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(ColumnStatisticsData::DecimalStats(field_ident));
                    } else {
                        return ::std::result::Result::Err(
                            ::pilota::thrift::new_protocol_exception(
                                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                "received multiple fields for union from remote Message",
                            ),
                        );
                    }
                }
                Some(7) => {
                    if ret.is_none() {
                        let field_ident = ::pilota::thrift::Message::decode(__protocol)?;
                        __protocol.struct_len(&field_ident);
                        ret = Some(ColumnStatisticsData::DateStats(field_ident));
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
                    Some(1) => {
                        if ret.is_none() {
                            let field_ident = <BooleanColumnStatsData as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                            ret = Some(ColumnStatisticsData::BooleanStats(field_ident));
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
                            let field_ident =
                                <LongColumnStatsData as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ColumnStatisticsData::LongStats(field_ident));
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
                                <DoubleColumnStatsData as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ColumnStatisticsData::DoubleStats(field_ident));
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
                                <StringColumnStatsData as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ColumnStatisticsData::StringStats(field_ident));
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(5) => {
                        if ret.is_none() {
                            let field_ident =
                                <BinaryColumnStatsData as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ColumnStatisticsData::BinaryStats(field_ident));
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(6) => {
                        if ret.is_none() {
                            let field_ident = <DecimalColumnStatsData as ::pilota::thrift::Message>::decode_async(__protocol).await?;

                            ret = Some(ColumnStatisticsData::DecimalStats(field_ident));
                        } else {
                            return ::std::result::Result::Err(
                                ::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ),
                            );
                        }
                    }
                    Some(7) => {
                        if ret.is_none() {
                            let field_ident =
                                <DateColumnStatsData as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?;

                            ret = Some(ColumnStatisticsData::DateStats(field_ident));
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
            name: "ColumnStatisticsData",
        }) + match self {
            ColumnStatisticsData::BooleanStats(value) => {
                __protocol.struct_field_len(Some(1), value)
            }
            ColumnStatisticsData::LongStats(value) => __protocol.struct_field_len(Some(2), value),
            ColumnStatisticsData::DoubleStats(value) => __protocol.struct_field_len(Some(3), value),
            ColumnStatisticsData::StringStats(value) => __protocol.struct_field_len(Some(4), value),
            ColumnStatisticsData::BinaryStats(value) => __protocol.struct_field_len(Some(5), value),
            ColumnStatisticsData::DecimalStats(value) => {
                __protocol.struct_field_len(Some(6), value)
            }
            ColumnStatisticsData::DateStats(value) => __protocol.struct_field_len(Some(7), value),
        } + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
