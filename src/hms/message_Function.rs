#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct Function {
    pub function_name: ::std::option::Option<::pilota::FastStr>,

    pub db_name: ::std::option::Option<::pilota::FastStr>,

    pub class_name: ::std::option::Option<::pilota::FastStr>,

    pub owner_name: ::std::option::Option<::pilota::FastStr>,

    pub owner_type: ::std::option::Option<PrincipalType>,

    pub create_time: ::std::option::Option<i32>,

    pub function_type: ::std::option::Option<FunctionType>,

    pub resource_uris: ::std::option::Option<::std::vec::Vec<ResourceUri>>,

    pub cat_name: ::std::option::Option<::pilota::FastStr>,
}
impl ::pilota::thrift::Message for Function {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Function" };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.function_name.as_ref() {
            __protocol.write_faststr_field(1, (value).clone())?;
        }
        if let Some(value) = self.db_name.as_ref() {
            __protocol.write_faststr_field(2, (value).clone())?;
        }
        if let Some(value) = self.class_name.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        if let Some(value) = self.owner_name.as_ref() {
            __protocol.write_faststr_field(4, (value).clone())?;
        }
        if let Some(value) = self.owner_type.as_ref() {
            __protocol.write_i32_field(5, (value).inner())?;
        }
        if let Some(value) = self.create_time.as_ref() {
            __protocol.write_i32_field(6, *value)?;
        }
        if let Some(value) = self.function_type.as_ref() {
            __protocol.write_i32_field(7, (value).inner())?;
        }
        if let Some(value) = self.resource_uris.as_ref() {
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
        if let Some(value) = self.cat_name.as_ref() {
            __protocol.write_faststr_field(9, (value).clone())?;
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
                        var_5 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_6 = Some(__protocol.read_i32()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_7 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::List => {
                        var_8 = Some(unsafe {
                            let list_ident = __protocol.read_list_begin()?;
                            let mut val: ::std::vec::Vec<ResourceUri> =
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
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_9 = Some(__protocol.read_faststr()?);
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
                    "decode struct `Function` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            function_name: var_1,
            db_name: var_2,
            class_name: var_3,
            owner_name: var_4,
            owner_type: var_5,
            create_time: var_6,
            function_type: var_7,
            resource_uris: var_8,
            cat_name: var_9,
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
                            var_5 = Some(
                                <PrincipalType as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_6 = Some(__protocol.read_i32().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_7 = Some(
                                <FunctionType as ::pilota::thrift::Message>::decode_async(
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
                                        <ResourceUri as ::pilota::thrift::Message>::decode_async(
                                            __protocol,
                                        )
                                        .await?,
                                    );
                                }
                                __protocol.read_list_end().await?;
                                val
                            });
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_9 = Some(__protocol.read_faststr().await?);
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
                        "decode struct `Function` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                function_name: var_1,
                db_name: var_2,
                class_name: var_3,
                owner_name: var_4,
                owner_type: var_5,
                create_time: var_6,
                function_type: var_7,
                resource_uris: var_8,
                cat_name: var_9,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Function" })
            + self
                .function_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
            + self
                .db_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
            + self
                .class_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + self
                .owner_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(4), value))
            + self.owner_type.as_ref().map_or(0, |value| {
                __protocol.i32_field_len(Some(5), (value).inner())
            })
            + self
                .create_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(6), *value))
            + self.function_type.as_ref().map_or(0, |value| {
                __protocol.i32_field_len(Some(7), (value).inner())
            })
            + self.resource_uris.as_ref().map_or(0, |value| {
                __protocol.list_field_len(
                    Some(8),
                    ::pilota::thrift::TType::Struct,
                    value,
                    |__protocol, el| __protocol.struct_len(el),
                )
            })
            + self
                .cat_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(9), value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
