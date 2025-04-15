pub struct MkThriftHiveMetastoreGenericClient;

pub type ThriftHiveMetastoreClient = ThriftHiveMetastoreGenericClient<
    ::volo::service::BoxCloneService<
        ::volo_thrift::context::ClientContext,
        ThriftHiveMetastoreRequestSend,
        ::std::option::Option<ThriftHiveMetastoreResponseRecv>,
        ::volo_thrift::ClientError,
    >,
>;

impl<S> ::volo::client::MkClient<::volo_thrift::Client<S>> for MkThriftHiveMetastoreGenericClient {
    type Target = ThriftHiveMetastoreGenericClient<S>;
    fn mk_client(&self, service: ::volo_thrift::Client<S>) -> Self::Target {
        ThriftHiveMetastoreGenericClient(service)
    }
}

#[derive(Clone)]
pub struct ThriftHiveMetastoreGenericClient<S>(pub ::volo_thrift::Client<S>);

pub struct ThriftHiveMetastoreOneShotClient<S>(pub ::volo_thrift::Client<S>);

impl<
        S: ::volo::service::Service<
                ::volo_thrift::context::ClientContext,
                ThriftHiveMetastoreRequestSend,
                Response = ::std::option::Option<ThriftHiveMetastoreResponseRecv>,
                Error = ::volo_thrift::ClientError,
            > + Send
            + Sync
            + 'static,
    > ThriftHiveMetastoreGenericClient<S>
{
    pub fn with_callopt<Opt: ::volo::client::Apply<::volo_thrift::context::ClientContext>>(
        self,
        opt: Opt,
    ) -> ThriftHiveMetastoreOneShotClient<::volo::client::WithOptService<S, Opt>> {
        ThriftHiveMetastoreOneShotClient(self.0.with_opt(opt))
    }

    pub async fn get_meta_conf(
        &self,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<::pilota::FastStr, ThriftHiveMetastoreGetMetaConfException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetMetaConf(ThriftHiveMetastoreGetMetaConfArgsSend {
                key,
            });
        let mut cx = self.0.make_cx("getMetaConf", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetMetaConf(
                ThriftHiveMetastoreGetMetaConfResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetMetaConf(
                ThriftHiveMetastoreGetMetaConfResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetMetaConfException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_meta_conf(
        &self,
        key: ::pilota::FastStr,
        value: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreSetMetaConfException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::SetMetaConf(ThriftHiveMetastoreSetMetaConfArgsSend {
                key,
                value,
            });
        let mut cx = self.0.make_cx("setMetaConf", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetMetaConf(
                ThriftHiveMetastoreSetMetaConfResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetMetaConf(
                ThriftHiveMetastoreSetMetaConfResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetMetaConfException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_database(
        &self,
        database: Database,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateDatabase(
            ThriftHiveMetastoreCreateDatabaseArgsSend { database },
        );
        let mut cx = self.0.make_cx("create_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_database(
        &self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Database, ThriftHiveMetastoreGetDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetDatabase(ThriftHiveMetastoreGetDatabaseArgsSend {
                name,
            });
        let mut cx = self.0.make_cx("get_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabaseException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_database(
        &self,
        name: ::pilota::FastStr,
        delete_data: bool,
        cascade: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::DropDatabase(ThriftHiveMetastoreDropDatabaseArgsSend {
                name,
                delete_data,
                cascade,
            });
        let mut cx = self.0.make_cx("drop_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_databases(
        &self,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetDatabasesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetDatabases(ThriftHiveMetastoreGetDatabasesArgsSend {
                pattern,
            });
        let mut cx = self.0.make_cx("get_databases", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDatabases(
                ThriftHiveMetastoreGetDatabasesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabases(
                ThriftHiveMetastoreGetDatabasesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabasesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_databases(
        &self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetAllDatabasesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAllDatabases(
            ThriftHiveMetastoreGetAllDatabasesArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_databases", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllDatabases(
                ThriftHiveMetastoreGetAllDatabasesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllDatabases(
                ThriftHiveMetastoreGetAllDatabasesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllDatabasesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_database(
        &self,
        dbname: ::pilota::FastStr,
        db: Database,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterDatabase(
            ThriftHiveMetastoreAlterDatabaseArgsSend { dbname, db },
        );
        let mut cx = self.0.make_cx("alter_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterDatabaseException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_type(
        &self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Type, ThriftHiveMetastoreGetTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetType(ThriftHiveMetastoreGetTypeArgsSend { name });
        let mut cx = self.0.make_cx("get_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_type(
        &self,
        r#type: Type,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateType(ThriftHiveMetastoreCreateTypeArgsSend {
                r#type,
            });
        let mut cx = self.0.make_cx("create_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_type(
        &self,
        r#type: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropType(ThriftHiveMetastoreDropTypeArgsSend {
            r#type,
        });
        let mut cx = self.0.make_cx("drop_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTypeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_type_all(
        &self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::AHashMap<::pilota::FastStr, Type>,
            ThriftHiveMetastoreGetTypeAllException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTypeAll(ThriftHiveMetastoreGetTypeAllArgsSend {
                name,
            });
        let mut cx = self.0.make_cx("get_type_all", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTypeAll(
                ThriftHiveMetastoreGetTypeAllResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTypeAll(
                ThriftHiveMetastoreGetTypeAllResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeAllException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_fields(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetFieldsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetFields(ThriftHiveMetastoreGetFieldsArgsSend {
            db_name,
            table_name,
        });
        let mut cx = self.0.make_cx("get_fields", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_fields_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetFieldsWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetFieldsWithEnvironmentContext(
            ThriftHiveMetastoreGetFieldsWithEnvironmentContextArgsSend {
                db_name,
                table_name,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("get_fields_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_schema(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetSchemaException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetSchema(ThriftHiveMetastoreGetSchemaArgsSend {
            db_name,
            table_name,
        });
        let mut cx = self.0.make_cx("get_schema", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_schema_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetSchemaWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetSchemaWithEnvironmentContext(
            ThriftHiveMetastoreGetSchemaWithEnvironmentContextArgsSend {
                db_name,
                table_name,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("get_schema_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table(
        &self,
        tbl: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateTableException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateTable(ThriftHiveMetastoreCreateTableArgsSend {
                tbl,
            });
        let mut cx = self.0.make_cx("create_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table_with_environment_context(
        &self,
        tbl: Table,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreCreateTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateTableWithEnvironmentContext(
            ThriftHiveMetastoreCreateTableWithEnvironmentContextArgsSend {
                tbl,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("create_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table_with_constraints(
        &self,
        tbl: Table,
        primary_keys: ::std::vec::Vec<SqlPrimaryKey>,
        foreign_keys: ::std::vec::Vec<SqlForeignKey>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateTableWithConstraintsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateTableWithConstraints(
            ThriftHiveMetastoreCreateTableWithConstraintsArgsSend {
                tbl,
                primary_keys,
                foreign_keys,
            },
        );
        let mut cx = self.0.make_cx("create_table_with_constraints", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_constraint(
        &self,
        req: DropConstraintRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropConstraintException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropConstraint(
            ThriftHiveMetastoreDropConstraintArgsSend { req },
        );
        let mut cx = self.0.make_cx("drop_constraint", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropConstraintException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropConstraintException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_primary_key(
        &self,
        req: AddPrimaryKeyRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddPrimaryKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPrimaryKey(
            ThriftHiveMetastoreAddPrimaryKeyArgsSend { req },
        );
        let mut cx = self.0.make_cx("add_primary_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPrimaryKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPrimaryKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_foreign_key(
        &self,
        req: AddForeignKeyRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddForeignKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddForeignKey(
            ThriftHiveMetastoreAddForeignKeyArgsSend { req },
        );
        let mut cx = self.0.make_cx("add_foreign_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddForeignKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddForeignKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_table(
        &self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropTableException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropTable(ThriftHiveMetastoreDropTableArgsSend {
            dbname,
            name,
            delete_data,
        });
        let mut cx = self.0.make_cx("drop_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_table_with_environment_context(
        &self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreDropTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropTableWithEnvironmentContext(
            ThriftHiveMetastoreDropTableWithEnvironmentContextArgsSend {
                dbname,
                name,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("drop_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_tables(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTablesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTables(ThriftHiveMetastoreGetTablesArgsSend {
            db_name,
            pattern,
        });
        let mut cx = self.0.make_cx("get_tables", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTables(
                ThriftHiveMetastoreGetTablesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTables(
                ThriftHiveMetastoreGetTablesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTablesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_tables_by_type(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
        table_type: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTablesByTypeException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTablesByType(
            ThriftHiveMetastoreGetTablesByTypeArgsSend {
                db_name,
                pattern,
                table_type,
            },
        );
        let mut cx = self.0.make_cx("get_tables_by_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTablesByType(
                ThriftHiveMetastoreGetTablesByTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTablesByType(
                ThriftHiveMetastoreGetTablesByTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTablesByTypeException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_meta(
        &self,
        db_patterns: ::pilota::FastStr,
        tbl_patterns: ::pilota::FastStr,
        tbl_types: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<TableMeta>,
            ThriftHiveMetastoreGetTableMetaException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTableMeta(ThriftHiveMetastoreGetTableMetaArgsSend {
                db_patterns,
                tbl_patterns,
                tbl_types,
            });
        let mut cx = self.0.make_cx("get_table_meta", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableMeta(
                ThriftHiveMetastoreGetTableMetaResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableMeta(
                ThriftHiveMetastoreGetTableMetaResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableMetaException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_tables(
        &self,
        db_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetAllTablesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetAllTables(ThriftHiveMetastoreGetAllTablesArgsSend {
                db_name,
            });
        let mut cx = self.0.make_cx("get_all_tables", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllTables(
                ThriftHiveMetastoreGetAllTablesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllTables(
                ThriftHiveMetastoreGetAllTablesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllTablesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Table, ThriftHiveMetastoreGetTableException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTable(ThriftHiveMetastoreGetTableArgsSend {
            dbname,
            tbl_name,
        });
        let mut cx = self.0.make_cx("get_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_objects_by_name(
        &self,
        dbname: ::pilota::FastStr,
        tbl_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Table>,
            ThriftHiveMetastoreGetTableObjectsByNameException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableObjectsByName(
            ThriftHiveMetastoreGetTableObjectsByNameArgsSend { dbname, tbl_names },
        );
        let mut cx = self.0.make_cx("get_table_objects_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_req(
        &self,
        req: GetTableRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<GetTableResult, ThriftHiveMetastoreGetTableReqException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTableReq(ThriftHiveMetastoreGetTableReqArgsSend {
                req,
            });
        let mut cx = self.0.make_cx("get_table_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_objects_by_name_req(
        &self,
        req: GetTablesRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetTablesResult,
            ThriftHiveMetastoreGetTableObjectsByNameReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableObjectsByNameReq(
            ThriftHiveMetastoreGetTableObjectsByNameReqArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_table_objects_by_name_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_names_by_filter(
        &self,
        dbname: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_tables: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTableNamesByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableNamesByFilter(
            ThriftHiveMetastoreGetTableNamesByFilterArgsSend {
                dbname,
                filter,
                max_tables,
            },
        );
        let mut cx = self.0.make_cx("get_table_names_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AlterTable(ThriftHiveMetastoreAlterTableArgsSend {
                dbname,
                tbl_name,
                new_tbl,
            });
        let mut cx = self.0.make_cx("alter_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table_with_environment_context(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterTableWithEnvironmentContext(
            ThriftHiveMetastoreAlterTableWithEnvironmentContextArgsSend {
                dbname,
                tbl_name,
                new_tbl,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table_with_cascade(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        cascade: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableWithCascadeException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterTableWithCascade(
            ThriftHiveMetastoreAlterTableWithCascadeArgsSend {
                dbname,
                tbl_name,
                new_tbl,
                cascade,
            },
        );
        let mut cx = self.0.make_cx("alter_table_with_cascade", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithCascadeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithCascadeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partition(
        &self,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAddPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AddPartition(ThriftHiveMetastoreAddPartitionArgsSend {
                new_part,
            });
        let mut cx = self.0.make_cx("add_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partition_with_environment_context(
        &self,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAddPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAddPartitionWithEnvironmentContextArgsSend {
                new_part,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("add_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions(
        &self,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitions(
            ThriftHiveMetastoreAddPartitionsArgsSend { new_parts },
        );
        let mut cx = self.0.make_cx("add_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions_pspec(
        &self,
        new_parts: ::std::vec::Vec<PartitionSpec>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsPspecException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionsPspec(
            ThriftHiveMetastoreAddPartitionsPspecArgsSend { new_parts },
        );
        let mut cx = self.0.make_cx("add_partitions_pspec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAppendPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartition(
            ThriftHiveMetastoreAppendPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
            },
        );
        let mut cx = self.0.make_cx("append_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions_req(
        &self,
        request: AddPartitionsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            AddPartitionsResult,
            ThriftHiveMetastoreAddPartitionsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionsReq(
            ThriftHiveMetastoreAddPartitionsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("add_partitions_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAppendPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_vals,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("append_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAppendPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionByName(
            ThriftHiveMetastoreAppendPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
            },
        );
        let mut cx = self.0.make_cx("append_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_by_name_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionByNameWithEnvironmentContext(
            ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_name,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("append_partition_by_name_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartition(
            ThriftHiveMetastoreDropPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDropPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionWithEnvironmentContext(
            ThriftHiveMetastoreDropPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_vals,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("drop_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionByName(
            ThriftHiveMetastoreDropPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_by_name_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionByNameWithEnvironmentContext(
            ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_name,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("drop_partition_by_name_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partitions_req(
        &self,
        req: DropPartitionsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            DropPartitionsResult,
            ThriftHiveMetastoreDropPartitionsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionsReq(
            ThriftHiveMetastoreDropPartitionsReqArgsSend { req },
        );
        let mut cx = self.0.make_cx("drop_partitions_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetPartition(ThriftHiveMetastoreGetPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
            });
        let mut cx = self.0.make_cx("get_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn exchange_partition(
        &self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreExchangePartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ExchangePartition(
            ThriftHiveMetastoreExchangePartitionArgsSend {
                partition_specs,
                source_db,
                source_table_name,
                dest_db,
                dest_table_name,
            },
        );
        let mut cx = self.0.make_cx("exchange_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn exchange_partitions(
        &self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreExchangePartitionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ExchangePartitions(
            ThriftHiveMetastoreExchangePartitionsArgsSend {
                partition_specs,
                source_db,
                source_table_name,
                dest_db,
                dest_table_name,
            },
        );
        let mut cx = self.0.make_cx("exchange_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionWithAuthException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionWithAuth(
            ThriftHiveMetastoreGetPartitionWithAuthArgsSend {
                db_name,
                tbl_name,
                part_vals,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partition_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionByName(
            ThriftHiveMetastoreGetPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
            },
        );
        let mut cx = self.0.make_cx("get_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitions(
            ThriftHiveMetastoreGetPartitionsArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsWithAuthException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsWithAuth(
            ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
                db_name,
                tbl_name,
                max_parts,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_pspec(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i32,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<PartitionSpec>,
            ThriftHiveMetastoreGetPartitionsPspecException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPspec(
            ThriftHiveMetastoreGetPartitionsPspecArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_pspec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPspecException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPspecException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetPartitionNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionNames(
            ThriftHiveMetastoreGetPartitionNamesArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partition_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNames(
                ThriftHiveMetastoreGetPartitionNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNames(
                ThriftHiveMetastoreGetPartitionNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_values(
        &self,
        request: PartitionValuesRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionValuesResponse,
            ThriftHiveMetastoreGetPartitionValuesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionValues(
            ThriftHiveMetastoreGetPartitionValuesArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_partition_values", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionValuesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionValuesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_ps(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsPsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPs(
            ThriftHiveMetastoreGetPartitionsPsArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_ps", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_ps_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsPsWithAuthException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPsWithAuth(
            ThriftHiveMetastoreGetPartitionsPsWithAuthArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_ps_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_names_ps(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetPartitionNamesPsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionNamesPs(
            ThriftHiveMetastoreGetPartitionNamesPsArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partition_names_ps", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesPsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesPsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByFilter(
            ThriftHiveMetastoreGetPartitionsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_part_specs_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i32,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<PartitionSpec>,
            ThriftHiveMetastoreGetPartSpecsByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartSpecsByFilter(
            ThriftHiveMetastoreGetPartSpecsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_part_specs_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartSpecsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartSpecsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_expr(
        &self,
        req: PartitionsByExprRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionsByExprResult,
            ThriftHiveMetastoreGetPartitionsByExprException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByExpr(
            ThriftHiveMetastoreGetPartitionsByExprArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_partitions_by_expr", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByExprException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByExprException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_num_partitions_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreGetNumPartitionsByFilterException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetNumPartitionsByFilter(
            ThriftHiveMetastoreGetNumPartitionsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
            },
        );
        let mut cx = self.0.make_cx("get_num_partitions_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetNumPartitionsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetNumPartitionsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsByNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByNames(
            ThriftHiveMetastoreGetPartitionsByNamesArgsSend {
                db_name,
                tbl_name,
                names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_by_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByNamesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartition(
            ThriftHiveMetastoreAlterPartitionArgsSend {
                db_name,
                tbl_name,
                new_part,
            },
        );
        let mut cx = self.0.make_cx("alter_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partitions(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitions(
            ThriftHiveMetastoreAlterPartitionsArgsSend {
                db_name,
                tbl_name,
                new_parts,
            },
        );
        let mut cx = self.0.make_cx("alter_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partitions_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitionsWithEnvironmentContext(
            ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                new_parts,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_partitions_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAlterPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                new_part,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn rename_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreRenamePartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RenamePartition(
            ThriftHiveMetastoreRenamePartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
                new_part,
            },
        );
        let mut cx = self.0.make_cx("rename_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenamePartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenamePartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_has_valid_characters(
        &self,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        throw_exception: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastorePartitionNameHasValidCharactersException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameHasValidCharacters(
            ThriftHiveMetastorePartitionNameHasValidCharactersArgsSend {
                part_vals,
                throw_exception,
            },
        );
        let mut cx = self.0.make_cx("partition_name_has_valid_characters", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameHasValidCharacters(
                ThriftHiveMetastorePartitionNameHasValidCharactersResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameHasValidCharacters(
                ThriftHiveMetastorePartitionNameHasValidCharactersResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameHasValidCharactersException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_config_value(
        &self,
        name: ::pilota::FastStr,
        default_value: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::FastStr,
            ThriftHiveMetastoreGetConfigValueException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetConfigValue(
            ThriftHiveMetastoreGetConfigValueArgsSend {
                name,
                default_value,
            },
        );
        let mut cx = self.0.make_cx("get_config_value", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetConfigValue(
                ThriftHiveMetastoreGetConfigValueResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetConfigValue(
                ThriftHiveMetastoreGetConfigValueResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetConfigValueException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_to_vals(
        &self,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastorePartitionNameToValsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameToVals(
            ThriftHiveMetastorePartitionNameToValsArgsSend { part_name },
        );
        let mut cx = self.0.make_cx("partition_name_to_vals", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToVals(
                ThriftHiveMetastorePartitionNameToValsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToVals(
                ThriftHiveMetastorePartitionNameToValsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameToValsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_to_spec(
        &self,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
            ThriftHiveMetastorePartitionNameToSpecException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameToSpec(
            ThriftHiveMetastorePartitionNameToSpecArgsSend { part_name },
        );
        let mut cx = self.0.make_cx("partition_name_to_spec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToSpec(
                ThriftHiveMetastorePartitionNameToSpecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToSpec(
                ThriftHiveMetastorePartitionNameToSpecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameToSpecException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn mark_partition_for_event(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreMarkPartitionForEventException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::MarkPartitionForEvent(
            ThriftHiveMetastoreMarkPartitionForEventArgsSend {
                db_name,
                tbl_name,
                part_vals,
                event_type,
            },
        );
        let mut cx = self.0.make_cx("markPartitionForEvent", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O4(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O5(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O5(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O6(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O6(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn is_partition_marked_for_event(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreIsPartitionMarkedForEventException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::IsPartitionMarkedForEvent(
            ThriftHiveMetastoreIsPartitionMarkedForEventArgsSend {
                db_name,
                tbl_name,
                part_vals,
                event_type,
            },
        );
        let mut cx = self.0.make_cx("isPartitionMarkedForEvent", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O4(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O5(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O5(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O6(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O6(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_index(
        &self,
        new_index: Index,
        index_table: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreAddIndexException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddIndex(ThriftHiveMetastoreAddIndexArgsSend {
            new_index,
            index_table,
        });
        let mut cx = self.0.make_cx("add_index", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_index(
        &self,
        dbname: ::pilota::FastStr,
        base_tbl_name: ::pilota::FastStr,
        idx_name: ::pilota::FastStr,
        new_idx: Index,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterIndexException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AlterIndex(ThriftHiveMetastoreAlterIndexArgsSend {
                dbname,
                base_tbl_name,
                idx_name,
                new_idx,
            });
        let mut cx = self.0.make_cx("alter_index", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterIndexException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterIndexException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_index_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropIndexByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropIndexByName(
            ThriftHiveMetastoreDropIndexByNameArgsSend {
                db_name,
                tbl_name,
                index_name,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_index_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropIndexByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropIndexByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_index_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreGetIndexByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetIndexByName(
            ThriftHiveMetastoreGetIndexByNameArgsSend {
                db_name,
                tbl_name,
                index_name,
            },
        );
        let mut cx = self.0.make_cx("get_index_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_indexes(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Index>,
            ThriftHiveMetastoreGetIndexesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetIndexes(ThriftHiveMetastoreGetIndexesArgsSend {
                db_name,
                tbl_name,
                max_indexes,
            });
        let mut cx = self.0.make_cx("get_indexes", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_index_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetIndexNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetIndexNames(
            ThriftHiveMetastoreGetIndexNamesArgsSend {
                db_name,
                tbl_name,
                max_indexes,
            },
        );
        let mut cx = self.0.make_cx("get_index_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexNames(
                ThriftHiveMetastoreGetIndexNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexNames(
                ThriftHiveMetastoreGetIndexNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_primary_keys(
        &self,
        request: PrimaryKeysRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PrimaryKeysResponse,
            ThriftHiveMetastoreGetPrimaryKeysException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrimaryKeys(
            ThriftHiveMetastoreGetPrimaryKeysArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_primary_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrimaryKeysException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrimaryKeysException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_foreign_keys(
        &self,
        request: ForeignKeysRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ForeignKeysResponse,
            ThriftHiveMetastoreGetForeignKeysException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetForeignKeys(
            ThriftHiveMetastoreGetForeignKeysArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_foreign_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetForeignKeysException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetForeignKeysException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_table_column_statistics(
        &self,
        stats_obj: ColumnStatistics,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreUpdateTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdateTableColumnStatistics(
            ThriftHiveMetastoreUpdateTableColumnStatisticsArgsSend { stats_obj },
        );
        let mut cx = self.0.make_cx("update_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_partition_column_statistics(
        &self,
        stats_obj: ColumnStatistics,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdatePartitionColumnStatistics(
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsArgsSend { stats_obj },
        );
        let mut cx = self.0.make_cx("update_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ColumnStatistics,
            ThriftHiveMetastoreGetTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableColumnStatistics(
            ThriftHiveMetastoreGetTableColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("get_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ColumnStatistics,
            ThriftHiveMetastoreGetPartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionColumnStatistics(
            ThriftHiveMetastoreGetPartitionColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                part_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("get_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_statistics_req(
        &self,
        request: TableStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            TableStatsResult,
            ThriftHiveMetastoreGetTableStatisticsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableStatisticsReq(
            ThriftHiveMetastoreGetTableStatisticsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_table_statistics_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableStatisticsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableStatisticsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_statistics_req(
        &self,
        request: PartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionsStatsResult,
            ThriftHiveMetastoreGetPartitionsStatisticsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsStatisticsReq(
            ThriftHiveMetastoreGetPartitionsStatisticsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_partitions_statistics_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsStatisticsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsStatisticsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_aggr_stats_for(
        &self,
        request: PartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<AggrStats, ThriftHiveMetastoreGetAggrStatsForException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAggrStatsFor(
            ThriftHiveMetastoreGetAggrStatsForArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_aggr_stats_for", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAggrStatsForException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAggrStatsForException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_aggr_stats_for(
        &self,
        request: SetPartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreSetAggrStatsForException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::SetAggrStatsFor(
            ThriftHiveMetastoreSetAggrStatsForArgsSend { request },
        );
        let mut cx = self.0.make_cx("set_aggr_stats_for", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn delete_partition_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDeletePartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DeletePartitionColumnStatistics(
            ThriftHiveMetastoreDeletePartitionColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                part_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("delete_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn delete_table_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDeleteTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DeleteTableColumnStatistics(
            ThriftHiveMetastoreDeleteTableColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("delete_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_function(
        &self,
        func: Function,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateFunction(
            ThriftHiveMetastoreCreateFunctionArgsSend { func },
        );
        let mut cx = self.0.make_cx("create_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::DropFunction(ThriftHiveMetastoreDropFunctionArgsSend {
                db_name,
                func_name,
            });
        let mut cx = self.0.make_cx("drop_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropFunctionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
        new_func: Function,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterFunction(
            ThriftHiveMetastoreAlterFunctionArgsSend {
                db_name,
                func_name,
                new_func,
            },
        );
        let mut cx = self.0.make_cx("alter_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterFunctionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_functions(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetFunctionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetFunctions(ThriftHiveMetastoreGetFunctionsArgsSend {
                db_name,
                pattern,
            });
        let mut cx = self.0.make_cx("get_functions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFunctions(
                ThriftHiveMetastoreGetFunctionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFunctions(
                ThriftHiveMetastoreGetFunctionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Function, ThriftHiveMetastoreGetFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetFunction(ThriftHiveMetastoreGetFunctionArgsSend {
                db_name,
                func_name,
            });
        let mut cx = self.0.make_cx("get_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_functions(
        &self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetAllFunctionsResponse,
            ThriftHiveMetastoreGetAllFunctionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAllFunctions(
            ThriftHiveMetastoreGetAllFunctionsArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_functions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllFunctions(
                ThriftHiveMetastoreGetAllFunctionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllFunctions(
                ThriftHiveMetastoreGetAllFunctionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllFunctionsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_role(
        &self,
        role: Role,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateRole(ThriftHiveMetastoreCreateRoleArgsSend {
                role,
            });
        let mut cx = self.0.make_cx("create_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateRole(
                ThriftHiveMetastoreCreateRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateRole(
                ThriftHiveMetastoreCreateRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_role(
        &self,
        role_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropRole(ThriftHiveMetastoreDropRoleArgsSend {
            role_name,
        });
        let mut cx = self.0.make_cx("drop_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropRole(
                ThriftHiveMetastoreDropRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropRole(
                ThriftHiveMetastoreDropRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_role_names(
        &self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetRoleNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetRoleNames(
            ThriftHiveMetastoreGetRoleNamesArgsSend {},
        );
        let mut cx = self.0.make_cx("get_role_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetRoleNames(
                ThriftHiveMetastoreGetRoleNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetRoleNames(
                ThriftHiveMetastoreGetRoleNamesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetRoleNamesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_role(
        &self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        grantor: ::pilota::FastStr,
        grantor_type: PrincipalType,
        grant_option: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRole(ThriftHiveMetastoreGrantRoleArgsSend {
            role_name,
            principal_name,
            principal_type,
            grantor,
            grantor_type,
            grant_option,
        });
        let mut cx = self.0.make_cx("grant_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRole(
                ThriftHiveMetastoreGrantRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRole(
                ThriftHiveMetastoreGrantRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn revoke_role(
        &self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokeRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::RevokeRole(ThriftHiveMetastoreRevokeRoleArgsSend {
                role_name,
                principal_name,
                principal_type,
            });
        let mut cx = self.0.make_cx("revoke_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RevokeRole(
                ThriftHiveMetastoreRevokeRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RevokeRole(
                ThriftHiveMetastoreRevokeRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRevokeRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn list_roles(
        &self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<::std::vec::Vec<Role>, ThriftHiveMetastoreListRolesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ListRoles(ThriftHiveMetastoreListRolesArgsSend {
            principal_name,
            principal_type,
        });
        let mut cx = self.0.make_cx("list_roles", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ListRoles(
                ThriftHiveMetastoreListRolesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ListRoles(
                ThriftHiveMetastoreListRolesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreListRolesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_revoke_role(
        &self,
        request: GrantRevokeRoleRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GrantRevokeRoleResponse,
            ThriftHiveMetastoreGrantRevokeRoleException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRevokeRole(
            ThriftHiveMetastoreGrantRevokeRoleArgsSend { request },
        );
        let mut cx = self.0.make_cx("grant_revoke_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokeRole(
                ThriftHiveMetastoreGrantRevokeRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokeRole(
                ThriftHiveMetastoreGrantRevokeRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRevokeRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_principals_in_role(
        &self,
        request: GetPrincipalsInRoleRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetPrincipalsInRoleResponse,
            ThriftHiveMetastoreGetPrincipalsInRoleException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrincipalsInRole(
            ThriftHiveMetastoreGetPrincipalsInRoleArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_principals_in_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrincipalsInRole(
                ThriftHiveMetastoreGetPrincipalsInRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrincipalsInRole(
                ThriftHiveMetastoreGetPrincipalsInRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrincipalsInRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_role_grants_for_principal(
        &self,
        request: GetRoleGrantsForPrincipalRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetRoleGrantsForPrincipalResponse,
            ThriftHiveMetastoreGetRoleGrantsForPrincipalException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetRoleGrantsForPrincipal(
            ThriftHiveMetastoreGetRoleGrantsForPrincipalArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_role_grants_for_principal", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetRoleGrantsForPrincipal(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetRoleGrantsForPrincipal(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_privilege_set(
        &self,
        hive_object: HiveObjectRef,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PrincipalPrivilegeSet,
            ThriftHiveMetastoreGetPrivilegeSetException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrivilegeSet(
            ThriftHiveMetastoreGetPrivilegeSetArgsSend {
                hive_object,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_privilege_set", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrivilegeSet(
                ThriftHiveMetastoreGetPrivilegeSetResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrivilegeSet(
                ThriftHiveMetastoreGetPrivilegeSetResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrivilegeSetException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn list_privileges(
        &self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        hive_object: HiveObjectRef,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<HiveObjectPrivilege>,
            ThriftHiveMetastoreListPrivilegesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ListPrivileges(
            ThriftHiveMetastoreListPrivilegesArgsSend {
                principal_name,
                principal_type,
                hive_object,
            },
        );
        let mut cx = self.0.make_cx("list_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ListPrivileges(
                ThriftHiveMetastoreListPrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ListPrivileges(
                ThriftHiveMetastoreListPrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreListPrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_privileges(
        &self,
        privileges: PrivilegeBag,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantPrivilegesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantPrivileges(
            ThriftHiveMetastoreGrantPrivilegesArgsSend { privileges },
        );
        let mut cx = self.0.make_cx("grant_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantPrivileges(
                ThriftHiveMetastoreGrantPrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantPrivileges(
                ThriftHiveMetastoreGrantPrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantPrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn revoke_privileges(
        &self,
        privileges: PrivilegeBag,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokePrivilegesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RevokePrivileges(
            ThriftHiveMetastoreRevokePrivilegesArgsSend { privileges },
        );
        let mut cx = self.0.make_cx("revoke_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RevokePrivileges(
                ThriftHiveMetastoreRevokePrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RevokePrivileges(
                ThriftHiveMetastoreRevokePrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRevokePrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_revoke_privileges(
        &self,
        request: GrantRevokePrivilegeRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GrantRevokePrivilegeResponse,
            ThriftHiveMetastoreGrantRevokePrivilegesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRevokePrivileges(
            ThriftHiveMetastoreGrantRevokePrivilegesArgsSend { request },
        );
        let mut cx = self.0.make_cx("grant_revoke_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokePrivileges(
                ThriftHiveMetastoreGrantRevokePrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokePrivileges(
                ThriftHiveMetastoreGrantRevokePrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRevokePrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_ugi(
        &self,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreSetUgiException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::SetUgi(ThriftHiveMetastoreSetUgiArgsSend {
            user_name,
            group_names,
        });
        let mut cx = self.0.make_cx("set_ugi", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetUgi(
                ThriftHiveMetastoreSetUgiResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetUgi(
                ThriftHiveMetastoreSetUgiResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetUgiException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_delegation_token(
        &self,
        token_owner: ::pilota::FastStr,
        renewer_kerberos_principal_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::FastStr,
            ThriftHiveMetastoreGetDelegationTokenException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetDelegationToken(
            ThriftHiveMetastoreGetDelegationTokenArgsSend {
                token_owner,
                renewer_kerberos_principal_name,
            },
        );
        let mut cx = self.0.make_cx("get_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDelegationToken(
                ThriftHiveMetastoreGetDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDelegationToken(
                ThriftHiveMetastoreGetDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn renew_delegation_token(
        &self,
        token_str_form: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i64, ThriftHiveMetastoreRenewDelegationTokenException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RenewDelegationToken(
            ThriftHiveMetastoreRenewDelegationTokenArgsSend { token_str_form },
        );
        let mut cx = self.0.make_cx("renew_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RenewDelegationToken(
                ThriftHiveMetastoreRenewDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RenewDelegationToken(
                ThriftHiveMetastoreRenewDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenewDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn cancel_delegation_token(
        &self,
        token_str_form: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCancelDelegationTokenException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CancelDelegationToken(
            ThriftHiveMetastoreCancelDelegationTokenArgsSend { token_str_form },
        );
        let mut cx = self.0.make_cx("cancel_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CancelDelegationToken(
                ThriftHiveMetastoreCancelDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CancelDelegationToken(
                ThriftHiveMetastoreCancelDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCancelDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_token(
        &self,
        token_identifier: ::pilota::FastStr,
        delegation_token: ::pilota::FastStr,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::AddToken(ThriftHiveMetastoreAddTokenArgsSend {
            token_identifier,
            delegation_token,
        });
        let mut cx = self.0.make_cx("add_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddToken(
                ThriftHiveMetastoreAddTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn remove_token(
        &self,
        token_identifier: ::pilota::FastStr,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::RemoveToken(ThriftHiveMetastoreRemoveTokenArgsSend {
                token_identifier,
            });
        let mut cx = self.0.make_cx("remove_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RemoveToken(
                ThriftHiveMetastoreRemoveTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_token(
        &self,
        token_identifier: ::pilota::FastStr,
    ) -> ::std::result::Result<::pilota::FastStr, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetToken(ThriftHiveMetastoreGetTokenArgsSend {
            token_identifier,
        });
        let mut cx = self.0.make_cx("get_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetToken(
                ThriftHiveMetastoreGetTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_token_identifiers(
        &self,
    ) -> ::std::result::Result<::std::vec::Vec<::pilota::FastStr>, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetAllTokenIdentifiers(
            ThriftHiveMetastoreGetAllTokenIdentifiersArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_token_identifiers", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllTokenIdentifiers(
                ThriftHiveMetastoreGetAllTokenIdentifiersResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_master_key(
        &self,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddMasterKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AddMasterKey(ThriftHiveMetastoreAddMasterKeyArgsSend {
                key,
            });
        let mut cx = self.0.make_cx("add_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddMasterKey(
                ThriftHiveMetastoreAddMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddMasterKey(
                ThriftHiveMetastoreAddMasterKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddMasterKeyException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_master_key(
        &self,
        seq_number: i32,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUpdateMasterKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdateMasterKey(
            ThriftHiveMetastoreUpdateMasterKeyArgsSend { seq_number, key },
        );
        let mut cx = self.0.make_cx("update_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateMasterKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateMasterKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn remove_master_key(
        &self,
        key_seq: i32,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::RemoveMasterKey(
            ThriftHiveMetastoreRemoveMasterKeyArgsSend { key_seq },
        );
        let mut cx = self.0.make_cx("remove_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RemoveMasterKey(
                ThriftHiveMetastoreRemoveMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_master_keys(
        &self,
    ) -> ::std::result::Result<::std::vec::Vec<::pilota::FastStr>, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetMasterKeys(
            ThriftHiveMetastoreGetMasterKeysArgsSend {},
        );
        let mut cx = self.0.make_cx("get_master_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetMasterKeys(
                ThriftHiveMetastoreGetMasterKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_open_txns(
        &self,
    ) -> ::std::result::Result<GetOpenTxnsResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::GetOpenTxns(ThriftHiveMetastoreGetOpenTxnsArgsSend {});
        let mut cx = self.0.make_cx("get_open_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetOpenTxns(
                ThriftHiveMetastoreGetOpenTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_open_txns_info(
        &self,
    ) -> ::std::result::Result<GetOpenTxnsInfoResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetOpenTxnsInfo(
            ThriftHiveMetastoreGetOpenTxnsInfoArgsSend {},
        );
        let mut cx = self.0.make_cx("get_open_txns_info", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetOpenTxnsInfo(
                ThriftHiveMetastoreGetOpenTxnsInfoResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn open_txns(
        &self,
        rqst: OpenTxnRequest,
    ) -> ::std::result::Result<OpenTxnsResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::OpenTxns(ThriftHiveMetastoreOpenTxnsArgsSend { rqst });
        let mut cx = self.0.make_cx("open_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::OpenTxns(
                ThriftHiveMetastoreOpenTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn abort_txn(
        &self,
        rqst: AbortTxnRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AbortTxn(ThriftHiveMetastoreAbortTxnArgsSend { rqst });
        let mut cx = self.0.make_cx("abort_txn", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AbortTxn(
                ThriftHiveMetastoreAbortTxnResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AbortTxn(
                ThriftHiveMetastoreAbortTxnResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAbortTxnException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn abort_txns(
        &self,
        rqst: AbortTxnsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AbortTxns(ThriftHiveMetastoreAbortTxnsArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("abort_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AbortTxns(
                ThriftHiveMetastoreAbortTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AbortTxns(
                ThriftHiveMetastoreAbortTxnsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAbortTxnsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn commit_txn(
        &self,
        rqst: CommitTxnRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCommitTxnException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CommitTxn(ThriftHiveMetastoreCommitTxnArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("commit_txn", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCommitTxnException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCommitTxnException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn lock(
        &self,
        rqst: LockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreLockException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::Lock(ThriftHiveMetastoreLockArgsSend { rqst });
        let mut cx = self.0.make_cx("lock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::Ok(
                resp,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::O1(
                ex,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreLockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::O2(
                ex,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreLockException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn check_lock(
        &self,
        rqst: CheckLockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreCheckLockException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CheckLock(ThriftHiveMetastoreCheckLockArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("check_lock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn unlock(
        &self,
        rqst: UnlockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUnlockException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::Unlock(ThriftHiveMetastoreUnlockArgsSend { rqst });
        let mut cx = self.0.make_cx("unlock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUnlockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUnlockException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn show_locks(
        &self,
        rqst: ShowLocksRequest,
    ) -> ::std::result::Result<ShowLocksResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::ShowLocks(ThriftHiveMetastoreShowLocksArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("show_locks", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ShowLocks(
                ThriftHiveMetastoreShowLocksResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn heartbeat(
        &self,
        ids: HeartbeatRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreHeartbeatException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::Heartbeat(ThriftHiveMetastoreHeartbeatArgsSend { ids });
        let mut cx = self.0.make_cx("heartbeat", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn heartbeat_txn_range(
        &self,
        txns: HeartbeatTxnRangeRequest,
    ) -> ::std::result::Result<HeartbeatTxnRangeResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::HeartbeatTxnRange(
            ThriftHiveMetastoreHeartbeatTxnRangeArgsSend { txns },
        );
        let mut cx = self.0.make_cx("heartbeat_txn_range", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::HeartbeatTxnRange(
                ThriftHiveMetastoreHeartbeatTxnRangeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn compact(
        &self,
        rqst: CompactionRequest,
    ) -> ::std::result::Result<(), ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::Compact(ThriftHiveMetastoreCompactArgsSend { rqst });
        let mut cx = self.0.make_cx("compact", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Compact(
                ThriftHiveMetastoreCompactResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn compact2(
        &self,
        rqst: CompactionRequest,
    ) -> ::std::result::Result<CompactionResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::Compact2(ThriftHiveMetastoreCompact2ArgsSend { rqst });
        let mut cx = self.0.make_cx("compact2", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Compact2(
                ThriftHiveMetastoreCompact2ResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn show_compact(
        &self,
        rqst: ShowCompactRequest,
    ) -> ::std::result::Result<ShowCompactResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::ShowCompact(ThriftHiveMetastoreShowCompactArgsSend {
                rqst,
            });
        let mut cx = self.0.make_cx("show_compact", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ShowCompact(
                ThriftHiveMetastoreShowCompactResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_dynamic_partitions(
        &self,
        rqst: AddDynamicPartitions,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddDynamicPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddDynamicPartitions(
            ThriftHiveMetastoreAddDynamicPartitionsArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("add_dynamic_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddDynamicPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddDynamicPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_next_notification(
        &self,
        rqst: NotificationEventRequest,
    ) -> ::std::result::Result<NotificationEventResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetNextNotification(
            ThriftHiveMetastoreGetNextNotificationArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("get_next_notification", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetNextNotification(
                ThriftHiveMetastoreGetNextNotificationResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_current_notification_event_id(
        &self,
    ) -> ::std::result::Result<CurrentNotificationEventId, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetCurrentNotificationEventId(
            ThriftHiveMetastoreGetCurrentNotificationEventIdArgsSend {},
        );
        let mut cx = self.0.make_cx("get_current_notificationEventId", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetCurrentNotificationEventId(
                ThriftHiveMetastoreGetCurrentNotificationEventIdResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn fire_listener_event(
        &self,
        rqst: FireEventRequest,
    ) -> ::std::result::Result<FireEventResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::FireListenerEvent(
            ThriftHiveMetastoreFireListenerEventArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("fire_listener_event", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::FireListenerEvent(
                ThriftHiveMetastoreFireListenerEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn flush_cache(&self) -> ::std::result::Result<(), ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::FlushCache(ThriftHiveMetastoreFlushCacheArgsSend {});
        let mut cx = self.0.make_cx("flushCache", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::FlushCache(
                ThriftHiveMetastoreFlushCacheResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_file_metadata_by_expr(
        &self,
        req: GetFileMetadataByExprRequest,
    ) -> ::std::result::Result<GetFileMetadataByExprResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetFileMetadataByExpr(
            ThriftHiveMetastoreGetFileMetadataByExprArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_file_metadata_by_expr", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFileMetadataByExpr(
                ThriftHiveMetastoreGetFileMetadataByExprResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_file_metadata(
        &self,
        req: GetFileMetadataRequest,
    ) -> ::std::result::Result<GetFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetFileMetadata(
            ThriftHiveMetastoreGetFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFileMetadata(
                ThriftHiveMetastoreGetFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn put_file_metadata(
        &self,
        req: PutFileMetadataRequest,
    ) -> ::std::result::Result<PutFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::PutFileMetadata(
            ThriftHiveMetastorePutFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("put_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PutFileMetadata(
                ThriftHiveMetastorePutFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn clear_file_metadata(
        &self,
        req: ClearFileMetadataRequest,
    ) -> ::std::result::Result<ClearFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::ClearFileMetadata(
            ThriftHiveMetastoreClearFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("clear_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ClearFileMetadata(
                ThriftHiveMetastoreClearFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn cache_file_metadata(
        &self,
        req: CacheFileMetadataRequest,
    ) -> ::std::result::Result<CacheFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::CacheFileMetadata(
            ThriftHiveMetastoreCacheFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("cache_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::service::Service::call(&self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CacheFileMetadata(
                ThriftHiveMetastoreCacheFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
}

impl<
        S: ::volo::client::OneShotService<
                ::volo_thrift::context::ClientContext,
                ThriftHiveMetastoreRequestSend,
                Response = ::std::option::Option<ThriftHiveMetastoreResponseRecv>,
                Error = ::volo_thrift::ClientError,
            > + Send
            + Sync
            + 'static,
    > ThriftHiveMetastoreOneShotClient<S>
{
    pub async fn get_meta_conf(
        self,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<::pilota::FastStr, ThriftHiveMetastoreGetMetaConfException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetMetaConf(ThriftHiveMetastoreGetMetaConfArgsSend {
                key,
            });
        let mut cx = self.0.make_cx("getMetaConf", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetMetaConf(
                ThriftHiveMetastoreGetMetaConfResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetMetaConf(
                ThriftHiveMetastoreGetMetaConfResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetMetaConfException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_meta_conf(
        self,
        key: ::pilota::FastStr,
        value: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreSetMetaConfException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::SetMetaConf(ThriftHiveMetastoreSetMetaConfArgsSend {
                key,
                value,
            });
        let mut cx = self.0.make_cx("setMetaConf", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetMetaConf(
                ThriftHiveMetastoreSetMetaConfResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetMetaConf(
                ThriftHiveMetastoreSetMetaConfResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetMetaConfException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_database(
        self,
        database: Database,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateDatabase(
            ThriftHiveMetastoreCreateDatabaseArgsSend { database },
        );
        let mut cx = self.0.make_cx("create_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateDatabase(
                ThriftHiveMetastoreCreateDatabaseResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateDatabaseException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_database(
        self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Database, ThriftHiveMetastoreGetDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetDatabase(ThriftHiveMetastoreGetDatabaseArgsSend {
                name,
            });
        let mut cx = self.0.make_cx("get_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabase(
                ThriftHiveMetastoreGetDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabaseException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_database(
        self,
        name: ::pilota::FastStr,
        delete_data: bool,
        cascade: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::DropDatabase(ThriftHiveMetastoreDropDatabaseArgsSend {
                name,
                delete_data,
                cascade,
            });
        let mut cx = self.0.make_cx("drop_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropDatabase(
                ThriftHiveMetastoreDropDatabaseResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropDatabaseException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_databases(
        self,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetDatabasesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetDatabases(ThriftHiveMetastoreGetDatabasesArgsSend {
                pattern,
            });
        let mut cx = self.0.make_cx("get_databases", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDatabases(
                ThriftHiveMetastoreGetDatabasesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDatabases(
                ThriftHiveMetastoreGetDatabasesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDatabasesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_databases(
        self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetAllDatabasesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAllDatabases(
            ThriftHiveMetastoreGetAllDatabasesArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_databases", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllDatabases(
                ThriftHiveMetastoreGetAllDatabasesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllDatabases(
                ThriftHiveMetastoreGetAllDatabasesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllDatabasesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_database(
        self,
        dbname: ::pilota::FastStr,
        db: Database,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterDatabaseException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterDatabase(
            ThriftHiveMetastoreAlterDatabaseArgsSend { dbname, db },
        );
        let mut cx = self.0.make_cx("alter_database", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterDatabaseException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterDatabase(
                ThriftHiveMetastoreAlterDatabaseResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterDatabaseException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_type(
        self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Type, ThriftHiveMetastoreGetTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetType(ThriftHiveMetastoreGetTypeArgsSend { name });
        let mut cx = self.0.make_cx("get_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetType(
                ThriftHiveMetastoreGetTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_type(
        self,
        r#type: Type,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateType(ThriftHiveMetastoreCreateTypeArgsSend {
                r#type,
            });
        let mut cx = self.0.make_cx("create_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateType(
                ThriftHiveMetastoreCreateTypeResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTypeException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_type(
        self,
        r#type: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropTypeException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropType(ThriftHiveMetastoreDropTypeArgsSend {
            r#type,
        });
        let mut cx = self.0.make_cx("drop_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTypeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropType(
                ThriftHiveMetastoreDropTypeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTypeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_type_all(
        self,
        name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::AHashMap<::pilota::FastStr, Type>,
            ThriftHiveMetastoreGetTypeAllException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTypeAll(ThriftHiveMetastoreGetTypeAllArgsSend {
                name,
            });
        let mut cx = self.0.make_cx("get_type_all", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTypeAll(
                ThriftHiveMetastoreGetTypeAllResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTypeAll(
                ThriftHiveMetastoreGetTypeAllResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTypeAllException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_fields(
        self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetFieldsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetFields(ThriftHiveMetastoreGetFieldsArgsSend {
            db_name,
            table_name,
        });
        let mut cx = self.0.make_cx("get_fields", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFields(
                ThriftHiveMetastoreGetFieldsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_fields_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetFieldsWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetFieldsWithEnvironmentContext(
            ThriftHiveMetastoreGetFieldsWithEnvironmentContextArgsSend {
                db_name,
                table_name,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("get_fields_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFieldsWithEnvironmentContext(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_schema(
        self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetSchemaException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetSchema(ThriftHiveMetastoreGetSchemaArgsSend {
            db_name,
            table_name,
        });
        let mut cx = self.0.make_cx("get_schema", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchema(
                ThriftHiveMetastoreGetSchemaResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_schema_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<FieldSchema>,
            ThriftHiveMetastoreGetSchemaWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetSchemaWithEnvironmentContext(
            ThriftHiveMetastoreGetSchemaWithEnvironmentContextArgsSend {
                db_name,
                table_name,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("get_schema_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetSchemaWithEnvironmentContext(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table(
        self,
        tbl: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateTableException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateTable(ThriftHiveMetastoreCreateTableArgsSend {
                tbl,
            });
        let mut cx = self.0.make_cx("create_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTable(
                ThriftHiveMetastoreCreateTableResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table_with_environment_context(
        self,
        tbl: Table,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreCreateTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateTableWithEnvironmentContext(
            ThriftHiveMetastoreCreateTableWithEnvironmentContextArgsSend {
                tbl,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("create_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithEnvironmentContext(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_table_with_constraints(
        self,
        tbl: Table,
        primary_keys: ::std::vec::Vec<SqlPrimaryKey>,
        foreign_keys: ::std::vec::Vec<SqlForeignKey>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateTableWithConstraintsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateTableWithConstraints(
            ThriftHiveMetastoreCreateTableWithConstraintsArgsSend {
                tbl,
                primary_keys,
                foreign_keys,
            },
        );
        let mut cx = self.0.make_cx("create_table_with_constraints", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateTableWithConstraints(
                ThriftHiveMetastoreCreateTableWithConstraintsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateTableWithConstraintsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_constraint(
        self,
        req: DropConstraintRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropConstraintException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropConstraint(
            ThriftHiveMetastoreDropConstraintArgsSend { req },
        );
        let mut cx = self.0.make_cx("drop_constraint", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropConstraintException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropConstraint(
                ThriftHiveMetastoreDropConstraintResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropConstraintException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_primary_key(
        self,
        req: AddPrimaryKeyRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddPrimaryKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPrimaryKey(
            ThriftHiveMetastoreAddPrimaryKeyArgsSend { req },
        );
        let mut cx = self.0.make_cx("add_primary_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPrimaryKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPrimaryKey(
                ThriftHiveMetastoreAddPrimaryKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPrimaryKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_foreign_key(
        self,
        req: AddForeignKeyRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddForeignKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddForeignKey(
            ThriftHiveMetastoreAddForeignKeyArgsSend { req },
        );
        let mut cx = self.0.make_cx("add_foreign_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddForeignKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddForeignKey(
                ThriftHiveMetastoreAddForeignKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddForeignKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_table(
        self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropTableException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropTable(ThriftHiveMetastoreDropTableArgsSend {
            dbname,
            name,
            delete_data,
        });
        let mut cx = self.0.make_cx("drop_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropTable(
                ThriftHiveMetastoreDropTableResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_table_with_environment_context(
        self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreDropTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropTableWithEnvironmentContext(
            ThriftHiveMetastoreDropTableWithEnvironmentContextArgsSend {
                dbname,
                name,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self.0.make_cx("drop_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropTableWithEnvironmentContext(
                ThriftHiveMetastoreDropTableWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropTableWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_tables(
        self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTablesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTables(ThriftHiveMetastoreGetTablesArgsSend {
            db_name,
            pattern,
        });
        let mut cx = self.0.make_cx("get_tables", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTables(
                ThriftHiveMetastoreGetTablesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTables(
                ThriftHiveMetastoreGetTablesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTablesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_tables_by_type(
        self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
        table_type: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTablesByTypeException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTablesByType(
            ThriftHiveMetastoreGetTablesByTypeArgsSend {
                db_name,
                pattern,
                table_type,
            },
        );
        let mut cx = self.0.make_cx("get_tables_by_type", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTablesByType(
                ThriftHiveMetastoreGetTablesByTypeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTablesByType(
                ThriftHiveMetastoreGetTablesByTypeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTablesByTypeException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_meta(
        self,
        db_patterns: ::pilota::FastStr,
        tbl_patterns: ::pilota::FastStr,
        tbl_types: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<TableMeta>,
            ThriftHiveMetastoreGetTableMetaException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTableMeta(ThriftHiveMetastoreGetTableMetaArgsSend {
                db_patterns,
                tbl_patterns,
                tbl_types,
            });
        let mut cx = self.0.make_cx("get_table_meta", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableMeta(
                ThriftHiveMetastoreGetTableMetaResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableMeta(
                ThriftHiveMetastoreGetTableMetaResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableMetaException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_tables(
        self,
        db_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetAllTablesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetAllTables(ThriftHiveMetastoreGetAllTablesArgsSend {
                db_name,
            });
        let mut cx = self.0.make_cx("get_all_tables", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllTables(
                ThriftHiveMetastoreGetAllTablesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllTables(
                ThriftHiveMetastoreGetAllTablesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllTablesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table(
        self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Table, ThriftHiveMetastoreGetTableException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTable(ThriftHiveMetastoreGetTableArgsSend {
            dbname,
            tbl_name,
        });
        let mut cx = self.0.make_cx("get_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTable(
                ThriftHiveMetastoreGetTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_objects_by_name(
        self,
        dbname: ::pilota::FastStr,
        tbl_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Table>,
            ThriftHiveMetastoreGetTableObjectsByNameException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableObjectsByName(
            ThriftHiveMetastoreGetTableObjectsByNameArgsSend { dbname, tbl_names },
        );
        let mut cx = self.0.make_cx("get_table_objects_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByName(
                ThriftHiveMetastoreGetTableObjectsByNameResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_req(
        self,
        req: GetTableRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<GetTableResult, ThriftHiveMetastoreGetTableReqException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetTableReq(ThriftHiveMetastoreGetTableReqArgsSend {
                req,
            });
        let mut cx = self.0.make_cx("get_table_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableReq(
                ThriftHiveMetastoreGetTableReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_objects_by_name_req(
        self,
        req: GetTablesRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetTablesResult,
            ThriftHiveMetastoreGetTableObjectsByNameReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableObjectsByNameReq(
            ThriftHiveMetastoreGetTableObjectsByNameReqArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_table_objects_by_name_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableObjectsByNameReq(
                ThriftHiveMetastoreGetTableObjectsByNameReqResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableObjectsByNameReqException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_names_by_filter(
        self,
        dbname: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_tables: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetTableNamesByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableNamesByFilter(
            ThriftHiveMetastoreGetTableNamesByFilterArgsSend {
                dbname,
                filter,
                max_tables,
            },
        );
        let mut cx = self.0.make_cx("get_table_names_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableNamesByFilter(
                ThriftHiveMetastoreGetTableNamesByFilterResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableNamesByFilterException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table(
        self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AlterTable(ThriftHiveMetastoreAlterTableArgsSend {
                dbname,
                tbl_name,
                new_tbl,
            });
        let mut cx = self.0.make_cx("alter_table", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTable(
                ThriftHiveMetastoreAlterTableResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table_with_environment_context(
        self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterTableWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterTableWithEnvironmentContext(
            ThriftHiveMetastoreAlterTableWithEnvironmentContextArgsSend {
                dbname,
                tbl_name,
                new_tbl,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_table_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithEnvironmentContext(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_table_with_cascade(
        self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        cascade: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableWithCascadeException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterTableWithCascade(
            ThriftHiveMetastoreAlterTableWithCascadeArgsSend {
                dbname,
                tbl_name,
                new_tbl,
                cascade,
            },
        );
        let mut cx = self.0.make_cx("alter_table_with_cascade", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithCascadeException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterTableWithCascade(
                ThriftHiveMetastoreAlterTableWithCascadeResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterTableWithCascadeException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partition(
        self,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAddPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AddPartition(ThriftHiveMetastoreAddPartitionArgsSend {
                new_part,
            });
        let mut cx = self.0.make_cx("add_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartition(
                ThriftHiveMetastoreAddPartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partition_with_environment_context(
        self,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAddPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAddPartitionWithEnvironmentContextArgsSend {
                new_part,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("add_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions(
        self,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitions(
            ThriftHiveMetastoreAddPartitionsArgsSend { new_parts },
        );
        let mut cx = self.0.make_cx("add_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitions(
                ThriftHiveMetastoreAddPartitionsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions_pspec(
        self,
        new_parts: ::std::vec::Vec<PartitionSpec>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsPspecException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionsPspec(
            ThriftHiveMetastoreAddPartitionsPspecArgsSend { new_parts },
        );
        let mut cx = self.0.make_cx("add_partitions_pspec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsPspec(
                ThriftHiveMetastoreAddPartitionsPspecResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsPspecException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAppendPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartition(
            ThriftHiveMetastoreAppendPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
            },
        );
        let mut cx = self.0.make_cx("append_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartition(
                ThriftHiveMetastoreAppendPartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_partitions_req(
        self,
        request: AddPartitionsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            AddPartitionsResult,
            ThriftHiveMetastoreAddPartitionsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddPartitionsReq(
            ThriftHiveMetastoreAddPartitionsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("add_partitions_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddPartitionsReq(
                ThriftHiveMetastoreAddPartitionsReqResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddPartitionsReqException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAppendPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_vals,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("append_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_by_name(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAppendPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionByName(
            ThriftHiveMetastoreAppendPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
            },
        );
        let mut cx = self.0.make_cx("append_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByName(
                ThriftHiveMetastoreAppendPartitionByNameResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn append_partition_by_name_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            Partition,
            ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AppendPartitionByNameWithEnvironmentContext(
            ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_name,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("append_partition_by_name_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AppendPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartition(
            ThriftHiveMetastoreDropPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartition(
                ThriftHiveMetastoreDropPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDropPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionWithEnvironmentContext(
            ThriftHiveMetastoreDropPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_vals,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("drop_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_by_name(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionByName(
            ThriftHiveMetastoreDropPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByName(
                ThriftHiveMetastoreDropPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partition_by_name_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionByNameWithEnvironmentContext(
            ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                part_name,
                delete_data,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("drop_partition_by_name_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionByNameWithEnvironmentContext(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_partitions_req(
        self,
        req: DropPartitionsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            DropPartitionsResult,
            ThriftHiveMetastoreDropPartitionsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropPartitionsReq(
            ThriftHiveMetastoreDropPartitionsReqArgsSend { req },
        );
        let mut cx = self.0.make_cx("drop_partitions_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropPartitionsReq(
                ThriftHiveMetastoreDropPartitionsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropPartitionsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetPartition(ThriftHiveMetastoreGetPartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
            });
        let mut cx = self.0.make_cx("get_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartition(
                ThriftHiveMetastoreGetPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn exchange_partition(
        self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreExchangePartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ExchangePartition(
            ThriftHiveMetastoreExchangePartitionArgsSend {
                partition_specs,
                source_db,
                source_table_name,
                dest_db,
                dest_table_name,
            },
        );
        let mut cx = self.0.make_cx("exchange_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartition(
                ThriftHiveMetastoreExchangePartitionResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn exchange_partitions(
        self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreExchangePartitionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ExchangePartitions(
            ThriftHiveMetastoreExchangePartitionsArgsSend {
                partition_specs,
                source_db,
                source_table_name,
                dest_db,
                dest_table_name,
            },
        );
        let mut cx = self.0.make_cx("exchange_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::ExchangePartitions(
                ThriftHiveMetastoreExchangePartitionsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreExchangePartitionsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_with_auth(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionWithAuthException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionWithAuth(
            ThriftHiveMetastoreGetPartitionWithAuthArgsSend {
                db_name,
                tbl_name,
                part_vals,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partition_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionWithAuth(
                ThriftHiveMetastoreGetPartitionWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_by_name(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionByName(
            ThriftHiveMetastoreGetPartitionByNameArgsSend {
                db_name,
                tbl_name,
                part_name,
            },
        );
        let mut cx = self.0.make_cx("get_partition_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionByName(
                ThriftHiveMetastoreGetPartitionByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitions(
            ThriftHiveMetastoreGetPartitionsArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitions(
                ThriftHiveMetastoreGetPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_with_auth(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsWithAuthException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsWithAuth(
            ThriftHiveMetastoreGetPartitionsWithAuthArgsSend {
                db_name,
                tbl_name,
                max_parts,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsWithAuth(
                ThriftHiveMetastoreGetPartitionsWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_pspec(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i32,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<PartitionSpec>,
            ThriftHiveMetastoreGetPartitionsPspecException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPspec(
            ThriftHiveMetastoreGetPartitionsPspecArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_pspec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPspecException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPspec(
                ThriftHiveMetastoreGetPartitionsPspecResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPspecException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_names(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetPartitionNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionNames(
            ThriftHiveMetastoreGetPartitionNamesArgsSend {
                db_name,
                tbl_name,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partition_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNames(
                ThriftHiveMetastoreGetPartitionNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNames(
                ThriftHiveMetastoreGetPartitionNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_values(
        self,
        request: PartitionValuesRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionValuesResponse,
            ThriftHiveMetastoreGetPartitionValuesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionValues(
            ThriftHiveMetastoreGetPartitionValuesArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_partition_values", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionValuesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionValues(
                ThriftHiveMetastoreGetPartitionValuesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionValuesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_ps(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsPsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPs(
            ThriftHiveMetastoreGetPartitionsPsArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_ps", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPs(
                ThriftHiveMetastoreGetPartitionsPsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_ps_with_auth(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsPsWithAuthException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsPsWithAuth(
            ThriftHiveMetastoreGetPartitionsPsWithAuthArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_ps_with_auth", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsWithAuthException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsPsWithAuth(
                ThriftHiveMetastoreGetPartitionsPsWithAuthResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsPsWithAuthException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_names_ps(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetPartitionNamesPsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionNamesPs(
            ThriftHiveMetastoreGetPartitionNamesPsArgsSend {
                db_name,
                tbl_name,
                part_vals,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partition_names_ps", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesPsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionNamesPs(
                ThriftHiveMetastoreGetPartitionNamesPsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionNamesPsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_filter(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByFilter(
            ThriftHiveMetastoreGetPartitionsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByFilter(
                ThriftHiveMetastoreGetPartitionsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_part_specs_by_filter(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i32,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<PartitionSpec>,
            ThriftHiveMetastoreGetPartSpecsByFilterException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartSpecsByFilter(
            ThriftHiveMetastoreGetPartSpecsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
                max_parts,
            },
        );
        let mut cx = self.0.make_cx("get_part_specs_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartSpecsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartSpecsByFilter(
                ThriftHiveMetastoreGetPartSpecsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartSpecsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_expr(
        self,
        req: PartitionsByExprRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionsByExprResult,
            ThriftHiveMetastoreGetPartitionsByExprException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByExpr(
            ThriftHiveMetastoreGetPartitionsByExprArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_partitions_by_expr", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByExprException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByExpr(
                ThriftHiveMetastoreGetPartitionsByExprResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByExprException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_num_partitions_by_filter(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreGetNumPartitionsByFilterException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetNumPartitionsByFilter(
            ThriftHiveMetastoreGetNumPartitionsByFilterArgsSend {
                db_name,
                tbl_name,
                filter,
            },
        );
        let mut cx = self.0.make_cx("get_num_partitions_by_filter", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetNumPartitionsByFilterException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetNumPartitionsByFilter(
                ThriftHiveMetastoreGetNumPartitionsByFilterResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetNumPartitionsByFilterException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_by_names(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Partition>,
            ThriftHiveMetastoreGetPartitionsByNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsByNames(
            ThriftHiveMetastoreGetPartitionsByNamesArgsSend {
                db_name,
                tbl_name,
                names,
            },
        );
        let mut cx = self.0.make_cx("get_partitions_by_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByNamesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsByNames(
                ThriftHiveMetastoreGetPartitionsByNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsByNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partition(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartition(
            ThriftHiveMetastoreAlterPartitionArgsSend {
                db_name,
                tbl_name,
                new_part,
            },
        );
        let mut cx = self.0.make_cx("alter_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartition(
                ThriftHiveMetastoreAlterPartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partitions(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitions(
            ThriftHiveMetastoreAlterPartitionsArgsSend {
                db_name,
                tbl_name,
                new_parts,
            },
        );
        let mut cx = self.0.make_cx("alter_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitions(
                ThriftHiveMetastoreAlterPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partitions_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitionsWithEnvironmentContext(
            ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                new_parts,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_partitions_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionsWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_partition_with_environment_context(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            (),
            ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterPartitionWithEnvironmentContext(
            ThriftHiveMetastoreAlterPartitionWithEnvironmentContextArgsSend {
                db_name,
                tbl_name,
                new_part,
                environment_context,
            },
        );
        let mut cx = self
            .0
            .make_cx("alter_partition_with_environment_context", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterPartitionWithEnvironmentContext(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn rename_partition(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        new_part: Partition,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreRenamePartitionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RenamePartition(
            ThriftHiveMetastoreRenamePartitionArgsSend {
                db_name,
                tbl_name,
                part_vals,
                new_part,
            },
        );
        let mut cx = self.0.make_cx("rename_partition", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenamePartitionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::RenamePartition(
                ThriftHiveMetastoreRenamePartitionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenamePartitionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_has_valid_characters(
        self,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        throw_exception: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastorePartitionNameHasValidCharactersException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameHasValidCharacters(
            ThriftHiveMetastorePartitionNameHasValidCharactersArgsSend {
                part_vals,
                throw_exception,
            },
        );
        let mut cx = self.0.make_cx("partition_name_has_valid_characters", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameHasValidCharacters(
                ThriftHiveMetastorePartitionNameHasValidCharactersResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameHasValidCharacters(
                ThriftHiveMetastorePartitionNameHasValidCharactersResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameHasValidCharactersException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_config_value(
        self,
        name: ::pilota::FastStr,
        default_value: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::FastStr,
            ThriftHiveMetastoreGetConfigValueException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetConfigValue(
            ThriftHiveMetastoreGetConfigValueArgsSend {
                name,
                default_value,
            },
        );
        let mut cx = self.0.make_cx("get_config_value", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetConfigValue(
                ThriftHiveMetastoreGetConfigValueResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetConfigValue(
                ThriftHiveMetastoreGetConfigValueResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetConfigValueException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_to_vals(
        self,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastorePartitionNameToValsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameToVals(
            ThriftHiveMetastorePartitionNameToValsArgsSend { part_name },
        );
        let mut cx = self.0.make_cx("partition_name_to_vals", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToVals(
                ThriftHiveMetastorePartitionNameToValsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToVals(
                ThriftHiveMetastorePartitionNameToValsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameToValsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn partition_name_to_spec(
        self,
        part_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
            ThriftHiveMetastorePartitionNameToSpecException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::PartitionNameToSpec(
            ThriftHiveMetastorePartitionNameToSpecArgsSend { part_name },
        );
        let mut cx = self.0.make_cx("partition_name_to_spec", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToSpec(
                ThriftHiveMetastorePartitionNameToSpecResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::PartitionNameToSpec(
                ThriftHiveMetastorePartitionNameToSpecResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastorePartitionNameToSpecException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn mark_partition_for_event(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreMarkPartitionForEventException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::MarkPartitionForEvent(
            ThriftHiveMetastoreMarkPartitionForEventArgsSend {
                db_name,
                tbl_name,
                part_vals,
                event_type,
            },
        );
        let mut cx = self.0.make_cx("markPartitionForEvent", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O4(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O5(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O5(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::MarkPartitionForEvent(
                ThriftHiveMetastoreMarkPartitionForEventResultRecv::O6(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreMarkPartitionForEventException::O6(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn is_partition_marked_for_event(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreIsPartitionMarkedForEventException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::IsPartitionMarkedForEvent(
            ThriftHiveMetastoreIsPartitionMarkedForEventArgsSend {
                db_name,
                tbl_name,
                part_vals,
                event_type,
            },
        );
        let mut cx = self.0.make_cx("isPartitionMarkedForEvent", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O4(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O5(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O5(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::IsPartitionMarkedForEvent(
                ThriftHiveMetastoreIsPartitionMarkedForEventResultRecv::O6(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreIsPartitionMarkedForEventException::O6(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_index(
        self,
        new_index: Index,
        index_table: Table,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreAddIndexException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddIndex(ThriftHiveMetastoreAddIndexArgsSend {
            new_index,
            index_table,
        });
        let mut cx = self.0.make_cx("add_index", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddIndex(
                ThriftHiveMetastoreAddIndexResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddIndexException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_index(
        self,
        dbname: ::pilota::FastStr,
        base_tbl_name: ::pilota::FastStr,
        idx_name: ::pilota::FastStr,
        new_idx: Index,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterIndexException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AlterIndex(ThriftHiveMetastoreAlterIndexArgsSend {
                dbname,
                base_tbl_name,
                idx_name,
                new_idx,
            });
        let mut cx = self.0.make_cx("alter_index", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterIndexException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterIndex(
                ThriftHiveMetastoreAlterIndexResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterIndexException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_index_by_name(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropIndexByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropIndexByName(
            ThriftHiveMetastoreDropIndexByNameArgsSend {
                db_name,
                tbl_name,
                index_name,
                delete_data,
            },
        );
        let mut cx = self.0.make_cx("drop_index_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropIndexByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropIndexByName(
                ThriftHiveMetastoreDropIndexByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropIndexByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_index_by_name(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreGetIndexByNameException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetIndexByName(
            ThriftHiveMetastoreGetIndexByNameArgsSend {
                db_name,
                tbl_name,
                index_name,
            },
        );
        let mut cx = self.0.make_cx("get_index_by_name", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexByNameException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexByName(
                ThriftHiveMetastoreGetIndexByNameResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexByNameException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_indexes(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<Index>,
            ThriftHiveMetastoreGetIndexesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetIndexes(ThriftHiveMetastoreGetIndexesArgsSend {
                db_name,
                tbl_name,
                max_indexes,
            });
        let mut cx = self.0.make_cx("get_indexes", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexesException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexes(
                ThriftHiveMetastoreGetIndexesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_index_names(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetIndexNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetIndexNames(
            ThriftHiveMetastoreGetIndexNamesArgsSend {
                db_name,
                tbl_name,
                max_indexes,
            },
        );
        let mut cx = self.0.make_cx("get_index_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetIndexNames(
                ThriftHiveMetastoreGetIndexNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetIndexNames(
                ThriftHiveMetastoreGetIndexNamesResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetIndexNamesException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_primary_keys(
        self,
        request: PrimaryKeysRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PrimaryKeysResponse,
            ThriftHiveMetastoreGetPrimaryKeysException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrimaryKeys(
            ThriftHiveMetastoreGetPrimaryKeysArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_primary_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrimaryKeysException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPrimaryKeys(
                ThriftHiveMetastoreGetPrimaryKeysResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrimaryKeysException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_foreign_keys(
        self,
        request: ForeignKeysRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ForeignKeysResponse,
            ThriftHiveMetastoreGetForeignKeysException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetForeignKeys(
            ThriftHiveMetastoreGetForeignKeysArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_foreign_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetForeignKeysException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetForeignKeys(
                ThriftHiveMetastoreGetForeignKeysResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetForeignKeysException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_table_column_statistics(
        self,
        stats_obj: ColumnStatistics,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreUpdateTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdateTableColumnStatistics(
            ThriftHiveMetastoreUpdateTableColumnStatisticsArgsSend { stats_obj },
        );
        let mut cx = self.0.make_cx("update_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateTableColumnStatistics(
                ThriftHiveMetastoreUpdateTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_partition_column_statistics(
        self,
        stats_obj: ColumnStatistics,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdatePartitionColumnStatistics(
            ThriftHiveMetastoreUpdatePartitionColumnStatisticsArgsSend { stats_obj },
        );
        let mut cx = self.0.make_cx("update_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdatePartitionColumnStatistics(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_column_statistics(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ColumnStatistics,
            ThriftHiveMetastoreGetTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableColumnStatistics(
            ThriftHiveMetastoreGetTableColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("get_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableColumnStatistics(
                ThriftHiveMetastoreGetTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partition_column_statistics(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ColumnStatistics,
            ThriftHiveMetastoreGetPartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionColumnStatistics(
            ThriftHiveMetastoreGetPartitionColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                part_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("get_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionColumnStatistics(
                ThriftHiveMetastoreGetPartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_table_statistics_req(
        self,
        request: TableStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            TableStatsResult,
            ThriftHiveMetastoreGetTableStatisticsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetTableStatisticsReq(
            ThriftHiveMetastoreGetTableStatisticsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_table_statistics_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableStatisticsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetTableStatisticsReq(
                ThriftHiveMetastoreGetTableStatisticsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetTableStatisticsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_partitions_statistics_req(
        self,
        request: PartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PartitionsStatsResult,
            ThriftHiveMetastoreGetPartitionsStatisticsReqException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPartitionsStatisticsReq(
            ThriftHiveMetastoreGetPartitionsStatisticsReqArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_partitions_statistics_req", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsStatisticsReqException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetPartitionsStatisticsReq(
                ThriftHiveMetastoreGetPartitionsStatisticsReqResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPartitionsStatisticsReqException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_aggr_stats_for(
        self,
        request: PartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<AggrStats, ThriftHiveMetastoreGetAggrStatsForException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAggrStatsFor(
            ThriftHiveMetastoreGetAggrStatsForArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_aggr_stats_for", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAggrStatsForException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetAggrStatsFor(
                ThriftHiveMetastoreGetAggrStatsForResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAggrStatsForException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_aggr_stats_for(
        self,
        request: SetPartitionsStatsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreSetAggrStatsForException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::SetAggrStatsFor(
            ThriftHiveMetastoreSetAggrStatsForArgsSend { request },
        );
        let mut cx = self.0.make_cx("set_aggr_stats_for", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::SetAggrStatsFor(
                ThriftHiveMetastoreSetAggrStatsForResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetAggrStatsForException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn delete_partition_column_statistics(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDeletePartitionColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DeletePartitionColumnStatistics(
            ThriftHiveMetastoreDeletePartitionColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                part_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("delete_partition_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeletePartitionColumnStatistics(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn delete_table_column_statistics(
        self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            bool,
            ThriftHiveMetastoreDeleteTableColumnStatisticsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DeleteTableColumnStatistics(
            ThriftHiveMetastoreDeleteTableColumnStatisticsArgsSend {
                db_name,
                tbl_name,
                col_name,
            },
        );
        let mut cx = self.0.make_cx("delete_table_column_statistics", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DeleteTableColumnStatistics(
                ThriftHiveMetastoreDeleteTableColumnStatisticsResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDeleteTableColumnStatisticsException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_function(
        self,
        func: Function,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CreateFunction(
            ThriftHiveMetastoreCreateFunctionArgsSend { func },
        );
        let mut cx = self.0.make_cx("create_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O3(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CreateFunction(
                ThriftHiveMetastoreCreateFunctionResultRecv::O4(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateFunctionException::O4(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_function(
        self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::DropFunction(ThriftHiveMetastoreDropFunctionArgsSend {
                db_name,
                func_name,
            });
        let mut cx = self.0.make_cx("drop_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::DropFunction(
                ThriftHiveMetastoreDropFunctionResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropFunctionException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn alter_function(
        self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
        new_func: Function,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AlterFunction(
            ThriftHiveMetastoreAlterFunctionArgsSend {
                db_name,
                func_name,
                new_func,
            },
        );
        let mut cx = self.0.make_cx("alter_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AlterFunction(
                ThriftHiveMetastoreAlterFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAlterFunctionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_functions(
        self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetFunctionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetFunctions(ThriftHiveMetastoreGetFunctionsArgsSend {
                db_name,
                pattern,
            });
        let mut cx = self.0.make_cx("get_functions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFunctions(
                ThriftHiveMetastoreGetFunctionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFunctions(
                ThriftHiveMetastoreGetFunctionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_function(
        self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<Function, ThriftHiveMetastoreGetFunctionException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::GetFunction(ThriftHiveMetastoreGetFunctionArgsSend {
                db_name,
                func_name,
            });
        let mut cx = self.0.make_cx("get_function", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::GetFunction(
                ThriftHiveMetastoreGetFunctionResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetFunctionException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_functions(
        self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetAllFunctionsResponse,
            ThriftHiveMetastoreGetAllFunctionsException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetAllFunctions(
            ThriftHiveMetastoreGetAllFunctionsArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_functions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllFunctions(
                ThriftHiveMetastoreGetAllFunctionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetAllFunctions(
                ThriftHiveMetastoreGetAllFunctionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetAllFunctionsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn create_role(
        self,
        role: Role,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::CreateRole(ThriftHiveMetastoreCreateRoleArgsSend {
                role,
            });
        let mut cx = self.0.make_cx("create_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CreateRole(
                ThriftHiveMetastoreCreateRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CreateRole(
                ThriftHiveMetastoreCreateRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCreateRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn drop_role(
        self,
        role_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::DropRole(ThriftHiveMetastoreDropRoleArgsSend {
            role_name,
        });
        let mut cx = self.0.make_cx("drop_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::DropRole(
                ThriftHiveMetastoreDropRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::DropRole(
                ThriftHiveMetastoreDropRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreDropRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_role_names(
        self,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreGetRoleNamesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetRoleNames(
            ThriftHiveMetastoreGetRoleNamesArgsSend {},
        );
        let mut cx = self.0.make_cx("get_role_names", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetRoleNames(
                ThriftHiveMetastoreGetRoleNamesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetRoleNames(
                ThriftHiveMetastoreGetRoleNamesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetRoleNamesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_role(
        self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        grantor: ::pilota::FastStr,
        grantor_type: PrincipalType,
        grant_option: bool,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRole(ThriftHiveMetastoreGrantRoleArgsSend {
            role_name,
            principal_name,
            principal_type,
            grantor,
            grantor_type,
            grant_option,
        });
        let mut cx = self.0.make_cx("grant_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRole(
                ThriftHiveMetastoreGrantRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRole(
                ThriftHiveMetastoreGrantRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn revoke_role(
        self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokeRoleException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::RevokeRole(ThriftHiveMetastoreRevokeRoleArgsSend {
                role_name,
                principal_name,
                principal_type,
            });
        let mut cx = self.0.make_cx("revoke_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RevokeRole(
                ThriftHiveMetastoreRevokeRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RevokeRole(
                ThriftHiveMetastoreRevokeRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRevokeRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn list_roles(
        self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<::std::vec::Vec<Role>, ThriftHiveMetastoreListRolesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ListRoles(ThriftHiveMetastoreListRolesArgsSend {
            principal_name,
            principal_type,
        });
        let mut cx = self.0.make_cx("list_roles", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ListRoles(
                ThriftHiveMetastoreListRolesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ListRoles(
                ThriftHiveMetastoreListRolesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreListRolesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_revoke_role(
        self,
        request: GrantRevokeRoleRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GrantRevokeRoleResponse,
            ThriftHiveMetastoreGrantRevokeRoleException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRevokeRole(
            ThriftHiveMetastoreGrantRevokeRoleArgsSend { request },
        );
        let mut cx = self.0.make_cx("grant_revoke_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokeRole(
                ThriftHiveMetastoreGrantRevokeRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokeRole(
                ThriftHiveMetastoreGrantRevokeRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRevokeRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_principals_in_role(
        self,
        request: GetPrincipalsInRoleRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetPrincipalsInRoleResponse,
            ThriftHiveMetastoreGetPrincipalsInRoleException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrincipalsInRole(
            ThriftHiveMetastoreGetPrincipalsInRoleArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_principals_in_role", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrincipalsInRole(
                ThriftHiveMetastoreGetPrincipalsInRoleResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrincipalsInRole(
                ThriftHiveMetastoreGetPrincipalsInRoleResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrincipalsInRoleException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_role_grants_for_principal(
        self,
        request: GetRoleGrantsForPrincipalRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GetRoleGrantsForPrincipalResponse,
            ThriftHiveMetastoreGetRoleGrantsForPrincipalException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetRoleGrantsForPrincipal(
            ThriftHiveMetastoreGetRoleGrantsForPrincipalArgsSend { request },
        );
        let mut cx = self.0.make_cx("get_role_grants_for_principal", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetRoleGrantsForPrincipal(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetRoleGrantsForPrincipal(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetRoleGrantsForPrincipalException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_privilege_set(
        self,
        hive_object: HiveObjectRef,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            PrincipalPrivilegeSet,
            ThriftHiveMetastoreGetPrivilegeSetException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetPrivilegeSet(
            ThriftHiveMetastoreGetPrivilegeSetArgsSend {
                hive_object,
                user_name,
                group_names,
            },
        );
        let mut cx = self.0.make_cx("get_privilege_set", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetPrivilegeSet(
                ThriftHiveMetastoreGetPrivilegeSetResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetPrivilegeSet(
                ThriftHiveMetastoreGetPrivilegeSetResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetPrivilegeSetException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn list_privileges(
        self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        hive_object: HiveObjectRef,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<HiveObjectPrivilege>,
            ThriftHiveMetastoreListPrivilegesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::ListPrivileges(
            ThriftHiveMetastoreListPrivilegesArgsSend {
                principal_name,
                principal_type,
                hive_object,
            },
        );
        let mut cx = self.0.make_cx("list_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ListPrivileges(
                ThriftHiveMetastoreListPrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::ListPrivileges(
                ThriftHiveMetastoreListPrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreListPrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_privileges(
        self,
        privileges: PrivilegeBag,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantPrivilegesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantPrivileges(
            ThriftHiveMetastoreGrantPrivilegesArgsSend { privileges },
        );
        let mut cx = self.0.make_cx("grant_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantPrivileges(
                ThriftHiveMetastoreGrantPrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantPrivileges(
                ThriftHiveMetastoreGrantPrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantPrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn revoke_privileges(
        self,
        privileges: PrivilegeBag,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokePrivilegesException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RevokePrivileges(
            ThriftHiveMetastoreRevokePrivilegesArgsSend { privileges },
        );
        let mut cx = self.0.make_cx("revoke_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RevokePrivileges(
                ThriftHiveMetastoreRevokePrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RevokePrivileges(
                ThriftHiveMetastoreRevokePrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRevokePrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn grant_revoke_privileges(
        self,
        request: GrantRevokePrivilegeRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            GrantRevokePrivilegeResponse,
            ThriftHiveMetastoreGrantRevokePrivilegesException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GrantRevokePrivileges(
            ThriftHiveMetastoreGrantRevokePrivilegesArgsSend { request },
        );
        let mut cx = self.0.make_cx("grant_revoke_privileges", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokePrivileges(
                ThriftHiveMetastoreGrantRevokePrivilegesResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GrantRevokePrivileges(
                ThriftHiveMetastoreGrantRevokePrivilegesResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGrantRevokePrivilegesException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn set_ugi(
        self,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::std::vec::Vec<::pilota::FastStr>,
            ThriftHiveMetastoreSetUgiException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::SetUgi(ThriftHiveMetastoreSetUgiArgsSend {
            user_name,
            group_names,
        });
        let mut cx = self.0.make_cx("set_ugi", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::SetUgi(
                ThriftHiveMetastoreSetUgiResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::SetUgi(
                ThriftHiveMetastoreSetUgiResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreSetUgiException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_delegation_token(
        self,
        token_owner: ::pilota::FastStr,
        renewer_kerberos_principal_name: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<
            ::pilota::FastStr,
            ThriftHiveMetastoreGetDelegationTokenException,
        >,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::GetDelegationToken(
            ThriftHiveMetastoreGetDelegationTokenArgsSend {
                token_owner,
                renewer_kerberos_principal_name,
            },
        );
        let mut cx = self.0.make_cx("get_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetDelegationToken(
                ThriftHiveMetastoreGetDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::GetDelegationToken(
                ThriftHiveMetastoreGetDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreGetDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn renew_delegation_token(
        self,
        token_str_form: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i64, ThriftHiveMetastoreRenewDelegationTokenException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::RenewDelegationToken(
            ThriftHiveMetastoreRenewDelegationTokenArgsSend { token_str_form },
        );
        let mut cx = self.0.make_cx("renew_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RenewDelegationToken(
                ThriftHiveMetastoreRenewDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::RenewDelegationToken(
                ThriftHiveMetastoreRenewDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreRenewDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn cancel_delegation_token(
        self,
        token_str_form: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCancelDelegationTokenException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CancelDelegationToken(
            ThriftHiveMetastoreCancelDelegationTokenArgsSend { token_str_form },
        );
        let mut cx = self.0.make_cx("cancel_delegation_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CancelDelegationToken(
                ThriftHiveMetastoreCancelDelegationTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CancelDelegationToken(
                ThriftHiveMetastoreCancelDelegationTokenResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCancelDelegationTokenException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_token(
        self,
        token_identifier: ::pilota::FastStr,
        delegation_token: ::pilota::FastStr,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::AddToken(ThriftHiveMetastoreAddTokenArgsSend {
            token_identifier,
            delegation_token,
        });
        let mut cx = self.0.make_cx("add_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddToken(
                ThriftHiveMetastoreAddTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn remove_token(
        self,
        token_identifier: ::pilota::FastStr,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::RemoveToken(ThriftHiveMetastoreRemoveTokenArgsSend {
                token_identifier,
            });
        let mut cx = self.0.make_cx("remove_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RemoveToken(
                ThriftHiveMetastoreRemoveTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_token(
        self,
        token_identifier: ::pilota::FastStr,
    ) -> ::std::result::Result<::pilota::FastStr, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetToken(ThriftHiveMetastoreGetTokenArgsSend {
            token_identifier,
        });
        let mut cx = self.0.make_cx("get_token", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetToken(
                ThriftHiveMetastoreGetTokenResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_all_token_identifiers(
        self,
    ) -> ::std::result::Result<::std::vec::Vec<::pilota::FastStr>, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetAllTokenIdentifiers(
            ThriftHiveMetastoreGetAllTokenIdentifiersArgsSend {},
        );
        let mut cx = self.0.make_cx("get_all_token_identifiers", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetAllTokenIdentifiers(
                ThriftHiveMetastoreGetAllTokenIdentifiersResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_master_key(
        self,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddMasterKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AddMasterKey(ThriftHiveMetastoreAddMasterKeyArgsSend {
                key,
            });
        let mut cx = self.0.make_cx("add_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddMasterKey(
                ThriftHiveMetastoreAddMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddMasterKey(
                ThriftHiveMetastoreAddMasterKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddMasterKeyException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn update_master_key(
        self,
        seq_number: i32,
        key: ::pilota::FastStr,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUpdateMasterKeyException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::UpdateMasterKey(
            ThriftHiveMetastoreUpdateMasterKeyArgsSend { seq_number, key },
        );
        let mut cx = self.0.make_cx("update_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateMasterKeyException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::UpdateMasterKey(
                ThriftHiveMetastoreUpdateMasterKeyResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUpdateMasterKeyException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn remove_master_key(
        self,
        key_seq: i32,
    ) -> ::std::result::Result<bool, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::RemoveMasterKey(
            ThriftHiveMetastoreRemoveMasterKeyArgsSend { key_seq },
        );
        let mut cx = self.0.make_cx("remove_master_key", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::RemoveMasterKey(
                ThriftHiveMetastoreRemoveMasterKeyResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_master_keys(
        self,
    ) -> ::std::result::Result<::std::vec::Vec<::pilota::FastStr>, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetMasterKeys(
            ThriftHiveMetastoreGetMasterKeysArgsSend {},
        );
        let mut cx = self.0.make_cx("get_master_keys", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetMasterKeys(
                ThriftHiveMetastoreGetMasterKeysResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_open_txns(
        self,
    ) -> ::std::result::Result<GetOpenTxnsResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::GetOpenTxns(ThriftHiveMetastoreGetOpenTxnsArgsSend {});
        let mut cx = self.0.make_cx("get_open_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetOpenTxns(
                ThriftHiveMetastoreGetOpenTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_open_txns_info(
        self,
    ) -> ::std::result::Result<GetOpenTxnsInfoResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetOpenTxnsInfo(
            ThriftHiveMetastoreGetOpenTxnsInfoArgsSend {},
        );
        let mut cx = self.0.make_cx("get_open_txns_info", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetOpenTxnsInfo(
                ThriftHiveMetastoreGetOpenTxnsInfoResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn open_txns(
        self,
        rqst: OpenTxnRequest,
    ) -> ::std::result::Result<OpenTxnsResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::OpenTxns(ThriftHiveMetastoreOpenTxnsArgsSend { rqst });
        let mut cx = self.0.make_cx("open_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::OpenTxns(
                ThriftHiveMetastoreOpenTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn abort_txn(
        self,
        rqst: AbortTxnRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::AbortTxn(ThriftHiveMetastoreAbortTxnArgsSend { rqst });
        let mut cx = self.0.make_cx("abort_txn", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AbortTxn(
                ThriftHiveMetastoreAbortTxnResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AbortTxn(
                ThriftHiveMetastoreAbortTxnResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAbortTxnException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn abort_txns(
        self,
        rqst: AbortTxnsRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AbortTxns(ThriftHiveMetastoreAbortTxnsArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("abort_txns", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AbortTxns(
                ThriftHiveMetastoreAbortTxnsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AbortTxns(
                ThriftHiveMetastoreAbortTxnsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAbortTxnsException::O1(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn commit_txn(
        self,
        rqst: CommitTxnRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCommitTxnException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CommitTxn(ThriftHiveMetastoreCommitTxnArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("commit_txn", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCommitTxnException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CommitTxn(
                ThriftHiveMetastoreCommitTxnResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCommitTxnException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn lock(
        self,
        rqst: LockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreLockException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::Lock(ThriftHiveMetastoreLockArgsSend { rqst });
        let mut cx = self.0.make_cx("lock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::Ok(
                resp,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::O1(
                ex,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreLockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Lock(ThriftHiveMetastoreLockResultRecv::O2(
                ex,
            ))) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreLockException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn check_lock(
        self,
        rqst: CheckLockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreCheckLockException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::CheckLock(ThriftHiveMetastoreCheckLockArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("check_lock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::CheckLock(
                ThriftHiveMetastoreCheckLockResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreCheckLockException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn unlock(
        self,
        rqst: UnlockRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUnlockException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::Unlock(ThriftHiveMetastoreUnlockArgsSend { rqst });
        let mut cx = self.0.make_cx("unlock", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUnlockException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Unlock(
                ThriftHiveMetastoreUnlockResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreUnlockException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn show_locks(
        self,
        rqst: ShowLocksRequest,
    ) -> ::std::result::Result<ShowLocksResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::ShowLocks(ThriftHiveMetastoreShowLocksArgsSend {
            rqst,
        });
        let mut cx = self.0.make_cx("show_locks", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ShowLocks(
                ThriftHiveMetastoreShowLocksResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn heartbeat(
        self,
        ids: HeartbeatRequest,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreHeartbeatException>,
        ::volo_thrift::ClientError,
    > {
        let req =
            ThriftHiveMetastoreRequestSend::Heartbeat(ThriftHiveMetastoreHeartbeatArgsSend { ids });
        let mut cx = self.0.make_cx("heartbeat", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O2(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::Heartbeat(
                ThriftHiveMetastoreHeartbeatResultRecv::O3(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreHeartbeatException::O3(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn heartbeat_txn_range(
        self,
        txns: HeartbeatTxnRangeRequest,
    ) -> ::std::result::Result<HeartbeatTxnRangeResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::HeartbeatTxnRange(
            ThriftHiveMetastoreHeartbeatTxnRangeArgsSend { txns },
        );
        let mut cx = self.0.make_cx("heartbeat_txn_range", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::HeartbeatTxnRange(
                ThriftHiveMetastoreHeartbeatTxnRangeResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn compact(
        self,
        rqst: CompactionRequest,
    ) -> ::std::result::Result<(), ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::Compact(ThriftHiveMetastoreCompactArgsSend { rqst });
        let mut cx = self.0.make_cx("compact", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Compact(
                ThriftHiveMetastoreCompactResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn compact2(
        self,
        rqst: CompactionRequest,
    ) -> ::std::result::Result<CompactionResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::Compact2(ThriftHiveMetastoreCompact2ArgsSend { rqst });
        let mut cx = self.0.make_cx("compact2", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::Compact2(
                ThriftHiveMetastoreCompact2ResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn show_compact(
        self,
        rqst: ShowCompactRequest,
    ) -> ::std::result::Result<ShowCompactResponse, ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::ShowCompact(ThriftHiveMetastoreShowCompactArgsSend {
                rqst,
            });
        let mut cx = self.0.make_cx("show_compact", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ShowCompact(
                ThriftHiveMetastoreShowCompactResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn add_dynamic_partitions(
        self,
        rqst: AddDynamicPartitions,
    ) -> ::std::result::Result<
        ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddDynamicPartitionsException>,
        ::volo_thrift::ClientError,
    > {
        let req = ThriftHiveMetastoreRequestSend::AddDynamicPartitions(
            ThriftHiveMetastoreAddDynamicPartitionsArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("add_dynamic_partitions", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)),
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::O1(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddDynamicPartitionsException::O1(ex),
            )),
            Some(ThriftHiveMetastoreResponseRecv::AddDynamicPartitions(
                ThriftHiveMetastoreAddDynamicPartitionsResultRecv::O2(ex),
            )) => ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(
                ThriftHiveMetastoreAddDynamicPartitionsException::O2(ex),
            )),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_next_notification(
        self,
        rqst: NotificationEventRequest,
    ) -> ::std::result::Result<NotificationEventResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetNextNotification(
            ThriftHiveMetastoreGetNextNotificationArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("get_next_notification", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetNextNotification(
                ThriftHiveMetastoreGetNextNotificationResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_current_notification_event_id(
        self,
    ) -> ::std::result::Result<CurrentNotificationEventId, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetCurrentNotificationEventId(
            ThriftHiveMetastoreGetCurrentNotificationEventIdArgsSend {},
        );
        let mut cx = self.0.make_cx("get_current_notificationEventId", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetCurrentNotificationEventId(
                ThriftHiveMetastoreGetCurrentNotificationEventIdResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn fire_listener_event(
        self,
        rqst: FireEventRequest,
    ) -> ::std::result::Result<FireEventResponse, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::FireListenerEvent(
            ThriftHiveMetastoreFireListenerEventArgsSend { rqst },
        );
        let mut cx = self.0.make_cx("fire_listener_event", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::FireListenerEvent(
                ThriftHiveMetastoreFireListenerEventResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn flush_cache(self) -> ::std::result::Result<(), ::volo_thrift::ClientError> {
        let req =
            ThriftHiveMetastoreRequestSend::FlushCache(ThriftHiveMetastoreFlushCacheArgsSend {});
        let mut cx = self.0.make_cx("flushCache", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::FlushCache(
                ThriftHiveMetastoreFlushCacheResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_file_metadata_by_expr(
        self,
        req: GetFileMetadataByExprRequest,
    ) -> ::std::result::Result<GetFileMetadataByExprResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetFileMetadataByExpr(
            ThriftHiveMetastoreGetFileMetadataByExprArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_file_metadata_by_expr", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFileMetadataByExpr(
                ThriftHiveMetastoreGetFileMetadataByExprResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn get_file_metadata(
        self,
        req: GetFileMetadataRequest,
    ) -> ::std::result::Result<GetFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::GetFileMetadata(
            ThriftHiveMetastoreGetFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("get_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::GetFileMetadata(
                ThriftHiveMetastoreGetFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn put_file_metadata(
        self,
        req: PutFileMetadataRequest,
    ) -> ::std::result::Result<PutFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::PutFileMetadata(
            ThriftHiveMetastorePutFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("put_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::PutFileMetadata(
                ThriftHiveMetastorePutFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn clear_file_metadata(
        self,
        req: ClearFileMetadataRequest,
    ) -> ::std::result::Result<ClearFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::ClearFileMetadata(
            ThriftHiveMetastoreClearFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("clear_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::ClearFileMetadata(
                ThriftHiveMetastoreClearFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
    pub async fn cache_file_metadata(
        self,
        req: CacheFileMetadataRequest,
    ) -> ::std::result::Result<CacheFileMetadataResult, ::volo_thrift::ClientError> {
        let req = ThriftHiveMetastoreRequestSend::CacheFileMetadata(
            ThriftHiveMetastoreCacheFileMetadataArgsSend { req },
        );
        let mut cx = self.0.make_cx("cache_file_metadata", false);
        #[allow(unreachable_patterns)]
        let resp = match ::volo::client::OneShotService::call(self.0, &mut cx, req).await? {
            Some(ThriftHiveMetastoreResponseRecv::CacheFileMetadata(
                ThriftHiveMetastoreCacheFileMetadataResultRecv::Ok(resp),
            )) => ::std::result::Result::Ok(resp),
            None => unreachable!(),
            _ => unreachable!(),
        };
        ::volo_thrift::context::CLIENT_CONTEXT_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.len() < cache.capacity() {
                cache.push(cx);
            }
        });
        resp
    }
}

pub struct ThriftHiveMetastoreClientBuilder {}

impl ThriftHiveMetastoreClientBuilder {
    pub fn new(
        service_name: impl AsRef<str>,
    ) -> ::volo_thrift::client::ClientBuilder<
        ::volo::layer::Identity,
        ::volo::layer::Identity,
        MkThriftHiveMetastoreGenericClient,
        ThriftHiveMetastoreRequestSend,
        ThriftHiveMetastoreResponseRecv,
        ::volo::net::dial::DefaultMakeTransport,
        ::volo_thrift::codec::default::DefaultMakeCodec<
            ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                ::volo_thrift::codec::default::framed::MakeFramedCodec<
                    ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                >,
            >,
        >,
        ::volo::loadbalance::LbConfig<
            ::volo::loadbalance::random::WeightedRandomBalance<()>,
            ::volo::discovery::DummyDiscover,
        >,
    > {
        ::volo_thrift::client::ClientBuilder::new(service_name, MkThriftHiveMetastoreGenericClient)
    }
}
