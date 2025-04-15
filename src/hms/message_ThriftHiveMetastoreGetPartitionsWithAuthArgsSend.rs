
impl ::std::default::Default for ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
    fn default() -> Self {
        ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
            db_name: ::std::default::Default::default(),
            tbl_name: ::std::default::Default::default(),
            max_parts: -1i16,
            user_name: ::std::default::Default::default(),
            group_names: ::std::default::Default::default(),
        }
    }
}
#[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
pub struct ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
    pub db_name: ::pilota::FastStr,

    pub tbl_name: ::pilota::FastStr,

    pub max_parts: i16,

    pub user_name: ::pilota::FastStr,

    pub group_names: ::std::vec::Vec<::pilota::FastStr>,
}
impl ::pilota::thrift::Message for ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreGetPartitionsWithAuthArgsSend",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        __protocol.write_faststr_field(1, (&self.db_name).clone())?;
        __protocol.write_faststr_field(2, (&self.tbl_name).clone())?;
        __protocol.write_i16_field(3, *&self.max_parts)?;
        __protocol.write_faststr_field(4, (&self.user_name).clone())?;
        __protocol.write_list_field(
            5,
            ::pilota::thrift::TType::Binary,
            &&self.group_names,
            |__protocol, val| {
                __protocol.write_faststr((val).clone())?;
                ::std::result::Result::Ok(())
            },
        )?;
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
        let mut var_3 = -1i16;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_1 = Some(__protocol.read_faststr()?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_2 = Some(__protocol.read_faststr()?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::I16 => {
                        var_3 = __protocol.read_i16()?;
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_4 = Some(__protocol.read_faststr()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_5 = Some(unsafe {
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
                err.prepend_msg(&format!("decode struct `ThriftHiveMetastoreGetPartitionsWithAuthArgsSend` field(#{}) failed, caused by: ", field_id));
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
        let Some(var_4) = var_4 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field user_name is required".to_string(),
            ));
        };
        let Some(var_5) = var_5 else {
            return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                "field group_names is required".to_string(),
            ));
        };

        let data = Self {
            db_name: var_1,
            tbl_name: var_2,
            max_parts: var_3,
            user_name: var_4,
            group_names: var_5,
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
            let mut var_3 = -1i16;
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_1 = Some(__protocol.read_faststr().await?);
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_2 = Some(__protocol.read_faststr().await?);
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::I16 => {
                            var_3 = __protocol.read_i16().await?;
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_4 = Some(__protocol.read_faststr().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            var_5 = Some({
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(__protocol.read_faststr().await?);
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
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
                    err.prepend_msg(&format!("decode struct `ThriftHiveMetastoreGetPartitionsWithAuthArgsSend` field(#{}) failed, caused by: ", field_id));
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
            let Some(var_4) = var_4 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field user_name is required".to_string(),
                ));
            };
            let Some(var_5) = var_5 else {
                return ::std::result::Result::Err(::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    "field group_names is required".to_string(),
                ));
            };

            let data = Self {
                db_name: var_1,
                tbl_name: var_2,
                max_parts: var_3,
                user_name: var_4,
                group_names: var_5,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "ThriftHiveMetastoreGetPartitionsWithAuthArgsSend",
        }) + __protocol.faststr_field_len(Some(1), &self.db_name)
            + __protocol.faststr_field_len(Some(2), &self.tbl_name)
            + __protocol.i16_field_len(Some(3), *&self.max_parts)
            + __protocol.faststr_field_len(Some(4), &self.user_name)
            + __protocol.list_field_len(
                Some(5),
                ::pilota::thrift::TType::Binary,
                &self.group_names,
                |__protocol, el| __protocol.faststr_len(el),
            )
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
