#[derive(Debug, Default, Clone, PartialEq)]
pub struct PrincipalPrivilegeSet {
    pub user_privileges: ::std::option::Option<
        ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<PrivilegeGrantInfo>>,
    >,

    pub group_privileges: ::std::option::Option<
        ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<PrivilegeGrantInfo>>,
    >,

    pub role_privileges: ::std::option::Option<
        ::pilota::AHashMap<::pilota::FastStr, ::std::vec::Vec<PrivilegeGrantInfo>>,
    >,
}
impl ::pilota::thrift::Message for PrincipalPrivilegeSet {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "PrincipalPrivilegeSet",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.user_privileges.as_ref() {
            __protocol.write_map_field(
                1,
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                &value,
                |__protocol, key| {
                    __protocol.write_faststr((key).clone())?;
                    ::std::result::Result::Ok(())
                },
                |__protocol, val| {
                    __protocol.write_list(
                        ::pilota::thrift::TType::Struct,
                        &val,
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    ::std::result::Result::Ok(())
                },
            )?;
        }
        if let Some(value) = self.group_privileges.as_ref() {
            __protocol.write_map_field(
                2,
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                &value,
                |__protocol, key| {
                    __protocol.write_faststr((key).clone())?;
                    ::std::result::Result::Ok(())
                },
                |__protocol, val| {
                    __protocol.write_list(
                        ::pilota::thrift::TType::Struct,
                        &val,
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    ::std::result::Result::Ok(())
                },
            )?;
        }
        if let Some(value) = self.role_privileges.as_ref() {
            __protocol.write_map_field(
                3,
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                &value,
                |__protocol, key| {
                    __protocol.write_faststr((key).clone())?;
                    ::std::result::Result::Ok(())
                },
                |__protocol, val| {
                    __protocol.write_list(
                        ::pilota::thrift::TType::Struct,
                        &val,
                        |__protocol, val| {
                            __protocol.write_struct(val)?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                    ::std::result::Result::Ok(())
                },
            )?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_1 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<PrivilegeGrantInfo> =
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
                            __protocol.read_map_end()?;
                            val
                        });
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_2 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<PrivilegeGrantInfo> =
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
                            __protocol.read_map_end()?;
                            val
                        });
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map => {
                        var_3 = Some({
                            let map_ident = __protocol.read_map_begin()?;
                            let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                            for _ in 0..map_ident.size {
                                val.insert(__protocol.read_faststr()?, unsafe {
                                    let list_ident = __protocol.read_list_begin()?;
                                    let mut val: ::std::vec::Vec<PrivilegeGrantInfo> =
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
                            __protocol.read_map_end()?;
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
                err.prepend_msg(&format!(
                    "decode struct `PrincipalPrivilegeSet` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            user_privileges: var_1,
            group_privileges: var_2,
            role_privileges: var_3,
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_1 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<PrivilegeGrantInfo as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(2) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_2 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<PrivilegeGrantInfo as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },Some(3) if field_ident.field_type == ::pilota::thrift::TType::Map  => {
                    var_3 = Some({
                        let map_ident = __protocol.read_map_begin().await?;
                        let mut val = ::pilota::AHashMap::with_capacity(map_ident.size);
                        for _ in 0..map_ident.size {
                            val.insert(__protocol.read_faststr().await?, {
                            let list_ident = __protocol.read_list_begin().await?;
                            let mut val = ::std::vec::Vec::with_capacity(list_ident.size);
                            for _ in 0..list_ident.size {
                                val.push(<PrivilegeGrantInfo as ::pilota::thrift::Message>::decode_async(__protocol).await?);
                            };
                            __protocol.read_list_end().await?;
                            val
                        });
                        }
                        __protocol.read_map_end().await?;
                        val
                    });

                },
                    _ => {
                        __protocol.skip(field_ident.field_type).await?;

                    },
                }

                __protocol.read_field_end().await?;


            };
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `PrincipalPrivilegeSet` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                user_privileges: var_1,
                group_privileges: var_2,
                role_privileges: var_3,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "PrincipalPrivilegeSet",
        }) + self.user_privileges.as_ref().map_or(0, |value| {
            __protocol.map_field_len(
                Some(1),
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                value,
                |__protocol, key| __protocol.faststr_len(key),
                |__protocol, val| {
                    __protocol.list_len(::pilota::thrift::TType::Struct, val, |__protocol, el| {
                        __protocol.struct_len(el)
                    })
                },
            )
        }) + self.group_privileges.as_ref().map_or(0, |value| {
            __protocol.map_field_len(
                Some(2),
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                value,
                |__protocol, key| __protocol.faststr_len(key),
                |__protocol, val| {
                    __protocol.list_len(::pilota::thrift::TType::Struct, val, |__protocol, el| {
                        __protocol.struct_len(el)
                    })
                },
            )
        }) + self.role_privileges.as_ref().map_or(0, |value| {
            __protocol.map_field_len(
                Some(3),
                ::pilota::thrift::TType::Binary,
                ::pilota::thrift::TType::List,
                value,
                |__protocol, key| __protocol.faststr_len(key),
                |__protocol, val| {
                    __protocol.list_len(::pilota::thrift::TType::Struct, val, |__protocol, el| {
                        __protocol.struct_len(el)
                    })
                },
            )
        }) + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
