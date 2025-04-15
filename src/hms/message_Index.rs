#[derive(Debug, Default, Clone, PartialEq)]
pub struct Index {
    pub index_name: ::std::option::Option<::pilota::FastStr>,

    pub index_handler_class: ::std::option::Option<::pilota::FastStr>,

    pub db_name: ::std::option::Option<::pilota::FastStr>,

    pub orig_table_name: ::std::option::Option<::pilota::FastStr>,

    pub create_time: ::std::option::Option<i32>,

    pub last_access_time: ::std::option::Option<i32>,

    pub index_table_name: ::std::option::Option<::pilota::FastStr>,

    pub sd: ::std::option::Option<StorageDescriptor>,

    pub parameters: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

    pub deferred_rebuild: ::std::option::Option<bool>,
}
impl ::pilota::thrift::Message for Index {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Index" };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.index_name.as_ref() {
            __protocol.write_faststr_field(1, (value).clone())?;
        }
        if let Some(value) = self.index_handler_class.as_ref() {
            __protocol.write_faststr_field(2, (value).clone())?;
        }
        if let Some(value) = self.db_name.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        if let Some(value) = self.orig_table_name.as_ref() {
            __protocol.write_faststr_field(4, (value).clone())?;
        }
        if let Some(value) = self.create_time.as_ref() {
            __protocol.write_i32_field(5, *value)?;
        }
        if let Some(value) = self.last_access_time.as_ref() {
            __protocol.write_i32_field(6, *value)?;
        }
        if let Some(value) = self.index_table_name.as_ref() {
            __protocol.write_faststr_field(7, (value).clone())?;
        }
        if let Some(value) = self.sd.as_ref() {
            __protocol.write_struct_field(8, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.parameters.as_ref() {
            __protocol.write_map_field(
                9,
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
        if let Some(value) = self.deferred_rebuild.as_ref() {
            __protocol.write_bool_field(10, *value)?;
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
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_4 = Some(__protocol.read_faststr()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_5 = Some(__protocol.read_i32()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_6 = Some(__protocol.read_i32()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_7 = Some(__protocol.read_faststr()?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_8 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_9 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, __protocol.read_faststr()?);
                            }
                            __protocol.read_map_end()?;
                            val
                        });
                    }
                    Some(10) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_10 = Some(__protocol.read_bool()?);
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
                    "decode struct `Index` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            index_name: var_1,
            index_handler_class: var_2,
            db_name: var_3,
            orig_table_name: var_4,
            create_time: var_5,
            last_access_time: var_6,
            index_table_name: var_7,
            sd: var_8,
            parameters: var_9,
            deferred_rebuild: var_10,
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
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_4 = Some(__protocol.read_faststr().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_5 = Some(__protocol.read_i32().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_6 = Some(__protocol.read_i32().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_7 = Some(__protocol.read_faststr().await?);
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_8 = Some(
                                <StorageDescriptor as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                            var_9 = Some({
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
                        Some(10) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_10 = Some(__protocol.read_bool().await?);
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
                        "decode struct `Index` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                index_name: var_1,
                index_handler_class: var_2,
                db_name: var_3,
                orig_table_name: var_4,
                create_time: var_5,
                last_access_time: var_6,
                index_table_name: var_7,
                sd: var_8,
                parameters: var_9,
                deferred_rebuild: var_10,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Index" })
            + self
                .index_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
            + self
                .index_handler_class
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
            + self
                .db_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + self
                .orig_table_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(4), value))
            + self
                .create_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(5), *value))
            + self
                .last_access_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(6), *value))
            + self
                .index_table_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(7), value))
            + self
                .sd
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(8), value))
            + self.parameters.as_ref().map_or(0, |value| {
                __protocol.map_field_len(
                    Some(9),
                    ::pilota::thrift::TType::Binary,
                    ::pilota::thrift::TType::Binary,
                    value,
                    |__protocol, key| __protocol.faststr_len(key),
                    |__protocol, val| __protocol.faststr_len(val),
                )
            })
            + self
                .deferred_rebuild
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(10), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
