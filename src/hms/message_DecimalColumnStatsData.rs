#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct DecimalColumnStatsData {
    pub low_value: ::std::option::Option<Decimal>,

    pub high_value: ::std::option::Option<Decimal>,

    pub num_nulls: i64,

    pub num_d_vs: i64,

    pub bit_vectors: ::std::option::Option<::pilota::FastStr>,
}
impl ::pilota::thrift::Message for DecimalColumnStatsData {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "DecimalColumnStatsData",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.low_value.as_ref() {
            __protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.high_value.as_ref() {
            __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
        }
        __protocol.write_i64_field(3, *&self.num_nulls)?;
        __protocol.write_i64_field(4, *&self.num_d_vs)?;
        if let Some(value) = self.bit_vectors.as_ref() {
            __protocol.write_faststr_field(5, (value).clone())?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_3 = Some(__protocol.read_i64()?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                        var_4 = Some(__protocol.read_i64()?);
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
                err.prepend_msg(&format!(
                    "decode struct `DecimalColumnStatsData` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let Some(var_3) = var_3 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field num_nulls is required".to_string(),
            ));
        };
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field num_d_vs is required".to_string(),
            ));
        };

        let data = Self {
            low_value: var_1,
            high_value: var_2,
            num_nulls: var_3,
            num_d_vs: var_4,
            bit_vectors: var_5,
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_1 = Some(
                                <Decimal as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_2 = Some(
                                <Decimal as ::pilota::thrift::Message>::decode_async(__protocol)
                                    .await?,
                            );
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_3 = Some(__protocol.read_i64().await?);
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::I64 => {
                            var_4 = Some(__protocol.read_i64().await?);
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
                    err.prepend_msg(&format!(
                        "decode struct `DecimalColumnStatsData` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let Some(var_3) = var_3 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field num_nulls is required".to_string(),
                ));
            };
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field num_d_vs is required".to_string(),
                ));
            };

            let data = Self {
                low_value: var_1,
                high_value: var_2,
                num_nulls: var_3,
                num_d_vs: var_4,
                bit_vectors: var_5,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "DecimalColumnStatsData",
        }) + self
            .low_value
            .as_ref()
            .map_or(0, |value| __protocol.struct_field_len(Some(1), value))
            + self
                .high_value
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(2), value))
            + __protocol.i64_field_len(Some(3), *&self.num_nulls)
            + __protocol.i64_field_len(Some(4), *&self.num_d_vs)
            + self
                .bit_vectors
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(5), value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
