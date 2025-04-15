#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct GrantRevokePrivilegeRequest {
    pub request_type: ::std::option::Option<GrantRevokeType>,

    pub privileges: ::std::option::Option<PrivilegeBag>,

    pub revoke_grant_option: ::std::option::Option<bool>,
}
impl ::pilota::thrift::Message for GrantRevokePrivilegeRequest {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "GrantRevokePrivilegeRequest",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.request_type.as_ref() {
            __protocol.write_i32_field(1, (value).inner())?;
        }
        if let Some(value) = self.privileges.as_ref() {
            __protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
        }
        if let Some(value) = self.revoke_grant_option.as_ref() {
            __protocol.write_bool_field(3, *value)?;
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
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_1 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(2) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                        var_2 = Some(::pilota::thrift::Message::decode(__protocol)?);
                    }
                    Some(3) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_3 = Some(__protocol.read_bool()?);
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
                    "decode struct `GrantRevokePrivilegeRequest` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            request_type: var_1,
            privileges: var_2,
            revoke_grant_option: var_3,
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
                        Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_1 = Some(
                                <GrantRevokeType as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(2) if field_ident.field_type == ::pilota::thrift::TType::Struct => {
                            var_2 = Some(
                                <PrivilegeBag as ::pilota::thrift::Message>::decode_async(
                                    __protocol,
                                )
                                .await?,
                            );
                        }
                        Some(3) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_3 = Some(__protocol.read_bool().await?);
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
                    err.prepend_msg(&format!("decode struct `GrantRevokePrivilegeRequest` field(#{}) failed, caused by: ", field_id));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                request_type: var_1,
                privileges: var_2,
                revoke_grant_option: var_3,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "GrantRevokePrivilegeRequest",
        }) + self.request_type.as_ref().map_or(0, |value| {
            __protocol.i32_field_len(Some(1), (value).inner())
        }) + self
            .privileges
            .as_ref()
            .map_or(0, |value| __protocol.struct_field_len(Some(2), value))
            + self
                .revoke_grant_option
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(3), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
