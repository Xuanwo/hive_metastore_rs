#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
pub struct SqlForeignKey {
    pub pktable_db: ::std::option::Option<::pilota::FastStr>,

    pub pktable_name: ::std::option::Option<::pilota::FastStr>,

    pub pkcolumn_name: ::std::option::Option<::pilota::FastStr>,

    pub fktable_db: ::std::option::Option<::pilota::FastStr>,

    pub fktable_name: ::std::option::Option<::pilota::FastStr>,

    pub fkcolumn_name: ::std::option::Option<::pilota::FastStr>,

    pub key_seq: ::std::option::Option<i32>,

    pub update_rule: ::std::option::Option<i32>,

    pub delete_rule: ::std::option::Option<i32>,

    pub fk_name: ::std::option::Option<::pilota::FastStr>,

    pub pk_name: ::std::option::Option<::pilota::FastStr>,

    pub enable_cstr: ::std::option::Option<bool>,

    pub validate_cstr: ::std::option::Option<bool>,

    pub rely_cstr: ::std::option::Option<bool>,
}
impl ::pilota::thrift::Message for SqlForeignKey {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        let struct_ident = ::pilota::thrift::TStructIdentifier {
            name: "SqlForeignKey",
        };

        __protocol.write_struct_begin(&struct_ident)?;
        if let Some(value) = self.pktable_db.as_ref() {
            __protocol.write_faststr_field(1, (value).clone())?;
        }
        if let Some(value) = self.pktable_name.as_ref() {
            __protocol.write_faststr_field(2, (value).clone())?;
        }
        if let Some(value) = self.pkcolumn_name.as_ref() {
            __protocol.write_faststr_field(3, (value).clone())?;
        }
        if let Some(value) = self.fktable_db.as_ref() {
            __protocol.write_faststr_field(4, (value).clone())?;
        }
        if let Some(value) = self.fktable_name.as_ref() {
            __protocol.write_faststr_field(5, (value).clone())?;
        }
        if let Some(value) = self.fkcolumn_name.as_ref() {
            __protocol.write_faststr_field(6, (value).clone())?;
        }
        if let Some(value) = self.key_seq.as_ref() {
            __protocol.write_i32_field(7, *value)?;
        }
        if let Some(value) = self.update_rule.as_ref() {
            __protocol.write_i32_field(8, *value)?;
        }
        if let Some(value) = self.delete_rule.as_ref() {
            __protocol.write_i32_field(9, *value)?;
        }
        if let Some(value) = self.fk_name.as_ref() {
            __protocol.write_faststr_field(10, (value).clone())?;
        }
        if let Some(value) = self.pk_name.as_ref() {
            __protocol.write_faststr_field(11, (value).clone())?;
        }
        if let Some(value) = self.enable_cstr.as_ref() {
            __protocol.write_bool_field(12, *value)?;
        }
        if let Some(value) = self.validate_cstr.as_ref() {
            __protocol.write_bool_field(13, *value)?;
        }
        if let Some(value) = self.rely_cstr.as_ref() {
            __protocol.write_bool_field(14, *value)?;
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
        let mut var_14 = None;

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
                    Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_5 = Some(__protocol.read_faststr()?);
                    }
                    Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_6 = Some(__protocol.read_faststr()?);
                    }
                    Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_7 = Some(__protocol.read_i32()?);
                    }
                    Some(8) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_8 = Some(__protocol.read_i32()?);
                    }
                    Some(9) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                        var_9 = Some(__protocol.read_i32()?);
                    }
                    Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_10 = Some(__protocol.read_faststr()?);
                    }
                    Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                        var_11 = Some(__protocol.read_faststr()?);
                    }
                    Some(12) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_12 = Some(__protocol.read_bool()?);
                    }
                    Some(13) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_13 = Some(__protocol.read_bool()?);
                    }
                    Some(14) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                        var_14 = Some(__protocol.read_bool()?);
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
                    "decode struct `SqlForeignKey` field(#{}) failed, caused by: ",
                    field_id
                ));
            }
            return ::std::result::Result::Err(err);
        };
        __protocol.read_struct_end()?;

        let data = Self {
            pktable_db: var_1,
            pktable_name: var_2,
            pkcolumn_name: var_3,
            fktable_db: var_4,
            fktable_name: var_5,
            fkcolumn_name: var_6,
            key_seq: var_7,
            update_rule: var_8,
            delete_rule: var_9,
            fk_name: var_10,
            pk_name: var_11,
            enable_cstr: var_12,
            validate_cstr: var_13,
            rely_cstr: var_14,
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
            let mut var_14 = None;

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
                        Some(5) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_5 = Some(__protocol.read_faststr().await?);
                        }
                        Some(6) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_6 = Some(__protocol.read_faststr().await?);
                        }
                        Some(7) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_7 = Some(__protocol.read_i32().await?);
                        }
                        Some(8) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_8 = Some(__protocol.read_i32().await?);
                        }
                        Some(9) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                            var_9 = Some(__protocol.read_i32().await?);
                        }
                        Some(10) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_10 = Some(__protocol.read_faststr().await?);
                        }
                        Some(11) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            var_11 = Some(__protocol.read_faststr().await?);
                        }
                        Some(12) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_12 = Some(__protocol.read_bool().await?);
                        }
                        Some(13) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_13 = Some(__protocol.read_bool().await?);
                        }
                        Some(14) if field_ident.field_type == ::pilota::thrift::TType::Bool => {
                            var_14 = Some(__protocol.read_bool().await?);
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
                        "decode struct `SqlForeignKey` field(#{}) failed, caused by: ",
                        field_id
                    ));
                }
                return ::std::result::Result::Err(err);
            };
            __protocol.read_struct_end().await?;

            let data = Self {
                pktable_db: var_1,
                pktable_name: var_2,
                pkcolumn_name: var_3,
                fktable_db: var_4,
                fktable_name: var_5,
                fkcolumn_name: var_6,
                key_seq: var_7,
                update_rule: var_8,
                delete_rule: var_9,
                fk_name: var_10,
                pk_name: var_11,
                enable_cstr: var_12,
                validate_cstr: var_13,
                rely_cstr: var_14,
            };
            ::std::result::Result::Ok(data)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
            name: "SqlForeignKey",
        }) + self
            .pktable_db
            .as_ref()
            .map_or(0, |value| __protocol.faststr_field_len(Some(1), value))
            + self
                .pktable_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(2), value))
            + self
                .pkcolumn_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(3), value))
            + self
                .fktable_db
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(4), value))
            + self
                .fktable_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(5), value))
            + self
                .fkcolumn_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(6), value))
            + self
                .key_seq
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(7), *value))
            + self
                .update_rule
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(8), *value))
            + self
                .delete_rule
                .as_ref()
                .map_or(0, |value| __protocol.i32_field_len(Some(9), *value))
            + self
                .fk_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(10), value))
            + self
                .pk_name
                .as_ref()
                .map_or(0, |value| __protocol.faststr_field_len(Some(11), value))
            + self
                .enable_cstr
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(12), *value))
            + self
                .validate_cstr
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(13), *value))
            + self
                .rely_cstr
                .as_ref()
                .map_or(0, |value| __protocol.bool_field_len(Some(14), *value))
            + __protocol.field_stop_len()
            + __protocol.struct_end_len()
    }
}
