
impl ::std::default::Default for Table {
    fn default() -> Self {
        Table {
            table_name: ::std::default::Default::default(),
            db_name: ::std::default::Default::default(),
            owner: ::std::default::Default::default(),
            create_time: ::std::default::Default::default(),
            last_access_time: ::std::default::Default::default(),
            retention: ::std::default::Default::default(),
            sd: ::std::default::Default::default(),
            partition_keys: ::std::default::Default::default(),
            parameters: ::std::default::Default::default(),
            view_original_text: ::std::default::Default::default(),
            view_expanded_text: ::std::default::Default::default(),
            table_type: ::std::default::Default::default(),
            privileges: ::std::default::Default::default(),
            temporary: Some(false),
            rewrite_enabled: ::std::default::Default::default(),
            cat_name: ::std::default::Default::default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub table_name: ::std::option::Option<::pilota::FastStr>,

    pub db_name: ::std::option::Option<::pilota::FastStr>,

    pub owner: ::std::option::Option<::pilota::FastStr>,

    pub create_time: ::std::option::Option<i32>,

    pub last_access_time: ::std::option::Option<i32>,

    pub retention: ::std::option::Option<i32>,

    pub sd: ::std::option::Option<StorageDescriptor>,

    pub partition_keys: ::std::option::Option<::std::vec::Vec<FieldSchema>>,

    pub parameters: ::std::option::Option<::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>>,

    pub view_original_text: ::std::option::Option<::pilota::FastStr>,

    pub view_expanded_text: ::std::option::Option<::pilota::FastStr>,

    pub table_type: ::std::option::Option<::pilota::FastStr>,

    pub privileges: ::std::option::Option<PrincipalPrivilegeSet>,

    pub temporary: ::std::option::Option<bool>,

    pub rewrite_enabled: ::std::option::Option<bool>,

    pub cat_name: ::std::option::Option<::pilota::FastStr>,
}
impl ::pilota::thrift::Message for Table {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Table" };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.table_name.as_ref() {
            __protocol.write_faststr_field(1, (value).clone())?;
        }
        if let Some(value) = self.db_name.as_ref() {
            __protocol.write_faststr_field(2, (value).clone())?;
        }
        if let Some(value) = self.owner.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        if let Some(value) = self.create_time.as_ref() {
            __protocol.write_i32_field(4, *value)?;
        }
        if let Some(value) = self.last_access_time.as_ref() {
            __protocol.write_i32_field(5, *value)?;
        }
        if let Some(value) = self.retention.as_ref() {
            __protocol.write_i32_field(6, *value)?;
        }
        if let Some(value) = self.sd.as_ref() {
            __protocol.write_struct_field(7, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.partition_keys.as_ref() {
            __protocol.write_list_field(
                8,
                ::pilota::thrift::TType::Struct,
                &value,
                |__protocol, val| {
                    __protocol.write_struct(val)?;
                    ::std::result::Result::Ok(())
                },
            )?;
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
        if let Some(value) = self.view_original_text.as_ref() {
            __protocol.write_faststr_field(10, (value).clone())?;
        }
        if let Some(value) = self.view_expanded_text.as_ref() {
            __protocol.write_faststr_field(11, (value).clone())?;
        }
        if let Some(value) = self.table_type.as_ref() {
            __protocol.write_faststr_field(12, (value).clone())?;
        }
        if let Some(value) = self.privileges.as_ref() {
            __protocol.write_struct_field(13, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.temporary.as_ref() {
            __protocol.write_bool_field(14, *value)?;
        }
        if let Some(value) = self.rewrite_enabled.as_ref() {
            __protocol.write_bool_field(15, *value)?;
        }
        if let Some(value) = self.cat_name.as_ref() {
            __protocol.write_faststr_field(16, (value).clone())?;
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
        let mut var_12 = None;
        let mut var_13 = None;
        let mut var_14 = Some(false);
        let mut var_15 = None;
        let mut var_16 = None;

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
                        var_4 = Some(__protocol.read_i32()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_5 = Some(__protocol.read_i32()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_6 = Some(__protocol.read_i32()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_7 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_8 = Some(unsafe {
                            let list_ident = __protocol.read_list_begin()?;
                            let mut val: ::std::vec::Vec<FieldSchema> =
                                ::std::vec::Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr()
                                    .offset(i as isize)
                                    .write(::pilota::thrift::Message::decode(__protocol)?);
                            }
                            val.set_len(list_ident.size);
                            __protocol.read_list_end()?;
                            val
                        });
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
                    Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_10 = Some(__protocol.read_faststr()?);
                    }
                    Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_11 = Some(__protocol.read_faststr()?);
                    }
                    Some(12) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_12 = Some(__protocol.read_faststr()?);
                    }
                    Some(13) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_13 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(14) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_14 = Some(__protocol.read_bool()?);
                    }
                    Some(15) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_15 = Some(__protocol.read_bool()?);
                    }
                    Some(16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_16 = Some(__protocol.read_faststr()?);
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
                    "decode struct `Table` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            table_name: var_1,
            db_name: var_2,
            owner: var_3,
            create_time: var_4,
            last_access_time: var_5,
            retention: var_6,
            sd: var_7,
            partition_keys: var_8,
            parameters: var_9,
            view_original_text: var_10,
            view_expanded_text: var_11,
            table_type: var_12,
            privileges: var_13,
            temporary: var_14,
            rewrite_enabled: var_15,
            cat_name: var_16,
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
            let mut var_12 = None;
            let mut var_13 = None;
            let mut var_14 = Some(false);
            let mut var_15 = None;
            let mut var_16 = None;

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
                            var_4 = Some(__protocol.read_i32().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_5 = Some(__protocol.read_i32().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_6 = Some(__protocol.read_i32().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_7 = Some(
                                <StorageDescriptor as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::List => {
                            var_8 = Some({
                                let list_ident = __protocol.read_list_begin().await?;
                                let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(
                                        <FieldSchema as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
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
                        Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_10 = Some(__protocol.read_faststr().await?);
                        }
                        Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_11 = Some(__protocol.read_faststr().await?);
                        }
                        Some(12) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_12 = Some(__protocol.read_faststr().await?);
                        }
                        Some(13) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_13 = Some(
                                <PrincipalPrivilegeSet as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(14) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_14 = Some(__protocol.read_bool().await?);
                        }
                        Some(15) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_15 = Some(__protocol.read_bool().await?);
                        }
                        Some(16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_16 = Some(__protocol.read_faststr().await?);
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
                        "decode struct `Table` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                table_name: var_1,
                db_name: var_2,
                owner: var_3,
                create_time: var_4,
                last_access_time: var_5,
                retention: var_6,
                sd: var_7,
                partition_keys: var_8,
                parameters: var_9,
                view_original_text: var_10,
                view_expanded_text: var_11,
                table_type: var_12,
                privileges: var_13,
                temporary: var_14,
                rewrite_enabled: var_15,
                cat_name: var_16,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Table" })
            + self
                .table_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
            + self
                .db_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
            + self
                .owner
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + self
                .create_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(4), *value))
            + self
                .last_access_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(5), *value))
            + self
                .retention
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(6), *value))
            + self
                .sd
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(7), value))
            + self.partition_keys.as_ref().map_or(0, |value| {
                __protocol.list_field_len(
                    Some(8),
                    ::pilota::thrift::TType::Struct,
                    value,
                    |__protocol, el| __protocol.struct_len(el),
                )
            })
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
                .view_original_text
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(10), value))
            + self
                .view_expanded_text
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(11), value))
            + self
                .table_type
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(12), value))
            + self
                .privileges
                .as_ref()
                .map_or(0, |value| __protocol.struct_field_len(Some(13), value))
            + self
                .temporary
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(14), *value))
            + self
                .rewrite_enabled
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(15), *value))
            + self
                .cat_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(16), value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
