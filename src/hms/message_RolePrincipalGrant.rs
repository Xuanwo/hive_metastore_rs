#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct RolePrincipalGrant {
    pub role_name: ::std::option::Option<::pilota::FastStr>,

    pub principal_name: ::std::option::Option<::pilota::FastStr>,

    pub principal_type: ::std::option::Option<PrincipalType>,

    pub grant_option: ::std::option::Option<bool>,

    pub grant_time: ::std::option::Option<i32>,

    pub grantor_name: ::std::option::Option<::pilota::FastStr>,

    pub grantor_principal_type: ::std::option::Option<PrincipalType>,
}
impl ::pilota::thrift::Message for RolePrincipalGrant {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "RolePrincipalGrant",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.role_name.as_ref() {
            __protocol.write_faststr_field(1, (value).clone())?;
        }
        if let Some(value) = self.principal_name.as_ref() {
            __protocol.write_faststr_field(2, (value).clone())?;
        }
        if let Some(value) = self.principal_type.as_ref() {
            __protocol.write_i32_field(3, (value).inner())?;
        }
        if let Some(value) = self.grant_option.as_ref() {
            __protocol.write_bool_field(4, *value)?;
        }
        if let Some(value) = self.grant_time.as_ref() {
            __protocol.write_i32_field(5, *value)?;
        }
        if let Some(value) = self.grantor_name.as_ref() {
            __protocol.write_faststr_field(6, (value).clone())?;
        }
        if let Some(value) = self.grantor_principal_type.as_ref() {
            __protocol.write_i32_field(7, (value).inner())?;
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
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_3 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(4) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_4 = Some(__protocol.read_bool()?);
                    }
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_5 = Some(__protocol.read_i32()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_6 = Some(__protocol.read_faststr()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_7 = Some(::pilota::thrift::Message::decode(__protocol)?);
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
                    "decode struct `RolePrincipalGrant` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            role_name: var_1,
            principal_name: var_2,
            principal_type: var_3,
            grant_option: var_4,
            grant_time: var_5,
            grantor_name: var_6,
            grantor_principal_type: var_7,
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
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_3 = Some(
                                <PrincipalType as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(4) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_4 = Some(__protocol.read_bool().await?);
                        }
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_5 = Some(__protocol.read_i32().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_6 = Some(__protocol.read_faststr().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_7 = Some(
                                <PrincipalType as ::pilota::thrift::Message>::decode_async(
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
                        "decode struct `RolePrincipalGrant` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                role_name: var_1,
                principal_name: var_2,
                principal_type: var_3,
                grant_option: var_4,
                grant_time: var_5,
                grantor_name: var_6,
                grantor_principal_type: var_7,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "RolePrincipalGrant",
        }) + self
            .role_name
            .as_ref()
            .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
            + self
                .principal_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
            + self.principal_type.as_ref().map_or(0, |value| {
                __protocol.i32_field_len(Some(3), (value).inner())
            })
            + self
                .grant_option
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(4), *value))
            + self
                .grant_time
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(5), *value))
            + self
                .grantor_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(6), value))
            + self.grantor_principal_type.as_ref().map_or(0, |value| {
                __protocol.i32_field_len(Some(7), (value).inner())
            })
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
