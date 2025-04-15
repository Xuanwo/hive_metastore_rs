#[derive(Debug, Default, Clone, PartialEq)]
pub struct PartitionWithoutSd {
    pub values: ::std::option::Option<::std::vec::Vec<::pilota::FastStr>>,

    pub create_time: ::std::option::Option<i32>,

    pub last_access_time: ::std::option::Option<i32>,

    pub relative_path: ::std::option::Option<::pilota::FastStr>,

    pub parameters: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

    pub privileges: ::std::option::Option<PrincipalPrivilegeSet>,
}
impl ::pilota::thrift::Message for PartitionWithoutSd {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "PartitionWithoutSd",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.values.as_ref() {
            __protocol.write_list_field(
                1,
                ::pilota::thrift::TType::Binary,
                &value,
                |__protocol, val| {
                    __protocol.write_faststr((val).clone())?;
                    ::std::result::Result::Ok(())
                },
            )?;
        }
        if let Some(value) = self.create_time.as_ref() {
            __protocol.write_i32_field(2, *value)?;
        }
        if let Some(value) = self.last_access_time.as_ref() {
            __protocol.write_i32_field(3, *value)?;
        }
        if let Some(value) = self.relative_path.as_ref() {
            __protocol.write_faststr_field(4, (value).clone())?;
        }
        if let Some(value) = self.parameters.as_ref() {
            __protocol.write_map_field(
                5,
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::Binary,
                &value,
                |__protocol, key| {
                    __protocol.write_faststr((key).clone())?;
                    ::std::result::Result::Ok(())
                },
                |__protocol, val| {
                    __protocol.write_faststr((val).clone())?;
                    ::std::result::Result::Ok(())
                },
            )?;
        }
        if let Some(value) = self.privileges.as_ref() {
            __protocol.write_struct_field(6, value, ::pilota::thrift::TType::Struct)?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_1 = Some(unsafe {
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
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_2 = Some(__protocol.read_i32()?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_3 = Some(__protocol.read_i32()?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_4 = Some(__protocol.read_faststr()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_5 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, __protocol.read_faststr()?);
                            }
                            __protocol.read_map_end()?;
                            val
                        });
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_6 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                    "decode struct `PartitionWithoutSd` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            values: var_1,
            create_time: var_2,
            last_access_time: var_3,
            relative_path: var_4,
            parameters: var_5,
            privileges: var_6,
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            var_1 = Some({
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(__protocol.read_faststr().await?);
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_2 = Some(__protocol.read_i32().await?);
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_3 = Some(__protocol.read_i32().await?);
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_4 = Some(__protocol.read_faststr().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            var_5 = Some({
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
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_6 = Some(
                                <PrincipalPrivilegeSet as ::pilota::thrift::Message>::decode_async(
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
                        "decode struct `PartitionWithoutSd` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                values: var_1,
                create_time: var_2,
                last_access_time: var_3,
                relative_path: var_4,
                parameters: var_5,
                privileges: var_6,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "PartitionWithoutSd",
        }) + self.values.as_ref().map_or(0, |value| {
            __protocol.list_field_len(
                Some(1),
                ::pilota::thrift::TType::Binary,
                value,
                |__protocol, el| __protocol.faststr_len(el),
            )
        }) + self
            .create_time
            .as_ref()
            .map_or(0, |value| __protocol.i32_field_len(Some(2), *value))
            + self
                .last_access_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(3), *value))
            + self
                .relative_path
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(4), value))
            + self.parameters.as_ref().map_or(0, |value| {
                __protocol.map_field_len(
                    Some(5),
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::Binary,
                    value,
                    |__protocol, key| __protocol.faststr_len(key),
                    |__protocol, val| __protocol.faststr_len(val),
                )
            })
            + self
                .privileges
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(6), value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
