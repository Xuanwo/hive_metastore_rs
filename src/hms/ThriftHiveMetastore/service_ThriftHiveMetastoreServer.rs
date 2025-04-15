pub struct ThriftHiveMetastoreServer<S> {
    inner: S, // handler
}

impl<S> ThriftHiveMetastoreServer<S>
where
    S: ThriftHiveMetastore + ::core::marker::Send + ::core::marker::Sync + 'static,
{
    pub fn new(
        inner: S,
    ) -> ::volo_thrift::server::Server<
        Self,
        ::volo::layer::Identity,
        ThriftHiveMetastoreRequestRecv,
        ::volo_thrift::codec::default::DefaultMakeCodec<
            ::volo_thrift::codec::default::ttheader::MakeTTHeaderCodec<
                ::volo_thrift::codec::default::framed::MakeFramedCodec<
                    ::volo_thrift::codec::default::thrift::MakeThriftCodec,
                >,
            >,
        >,
        ::volo_thrift::tracing::DefaultProvider,
    > {
        ::volo_thrift::server::Server::new(Self { inner })
    }
}

impl<T>
    ::volo::service::Service<::volo_thrift::context::ServerContext, ThriftHiveMetastoreRequestRecv>
    for ThriftHiveMetastoreServer<T>
where
    T: ThriftHiveMetastore + Send + Sync + 'static,
{
    type Response = ThriftHiveMetastoreResponseSend;
    type Error = ::volo_thrift::ServerError;

    async fn call<'s, 'cx>(
        &'s self,
        _cx: &'cx mut ::volo_thrift::context::ServerContext,
        req: ThriftHiveMetastoreRequestRecv,
    ) -> ::std::result::Result<Self::Response, Self::Error> {
        match req {
                        ThriftHiveMetastoreRequestRecv::GetMetaConf(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetMetaConf(
                match self.inner.get_meta_conf(args.key).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetMetaConfResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetMetaConfException::O1(ex))) => ThriftHiveMetastoreGetMetaConfResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::SetMetaConf(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::SetMetaConf(
                match self.inner.set_meta_conf(args.key,args.value).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreSetMetaConfResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetMetaConfException::O1(ex))) => ThriftHiveMetastoreSetMetaConfResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateDatabase(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateDatabase(
                match self.inner.create_database(args.database).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateDatabaseResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateDatabaseException::O1(ex))) => ThriftHiveMetastoreCreateDatabaseResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateDatabaseException::O2(ex))) => ThriftHiveMetastoreCreateDatabaseResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateDatabaseException::O3(ex))) => ThriftHiveMetastoreCreateDatabaseResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetDatabase(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetDatabase(
                match self.inner.get_database(args.name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetDatabaseResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetDatabaseException::O1(ex))) => ThriftHiveMetastoreGetDatabaseResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetDatabaseException::O2(ex))) => ThriftHiveMetastoreGetDatabaseResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropDatabase(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropDatabase(
                match self.inner.drop_database(args.name,args.delete_data,args.cascade).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropDatabaseResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropDatabaseException::O1(ex))) => ThriftHiveMetastoreDropDatabaseResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropDatabaseException::O2(ex))) => ThriftHiveMetastoreDropDatabaseResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropDatabaseException::O3(ex))) => ThriftHiveMetastoreDropDatabaseResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetDatabases(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetDatabases(
                match self.inner.get_databases(args.pattern).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetDatabasesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetDatabasesException::O1(ex))) => ThriftHiveMetastoreGetDatabasesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetAllDatabases(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetAllDatabases(
                match self.inner.get_all_databases().await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetAllDatabasesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetAllDatabasesException::O1(ex))) => ThriftHiveMetastoreGetAllDatabasesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterDatabase(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterDatabase(
                match self.inner.alter_database(args.dbname,args.db).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterDatabaseResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterDatabaseException::O1(ex))) => ThriftHiveMetastoreAlterDatabaseResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterDatabaseException::O2(ex))) => ThriftHiveMetastoreAlterDatabaseResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetType(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetType(
                match self.inner.get_type(args.name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTypeResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTypeException::O1(ex))) => ThriftHiveMetastoreGetTypeResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTypeException::O2(ex))) => ThriftHiveMetastoreGetTypeResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateType(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateType(
                match self.inner.create_type(args.r#type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateTypeResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTypeException::O1(ex))) => ThriftHiveMetastoreCreateTypeResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTypeException::O2(ex))) => ThriftHiveMetastoreCreateTypeResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTypeException::O3(ex))) => ThriftHiveMetastoreCreateTypeResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropType(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropType(
                match self.inner.drop_type(args.r#type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropTypeResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTypeException::O1(ex))) => ThriftHiveMetastoreDropTypeResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTypeException::O2(ex))) => ThriftHiveMetastoreDropTypeResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTypeAll(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTypeAll(
                match self.inner.get_type_all(args.name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTypeAllResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTypeAllException::O2(ex))) => ThriftHiveMetastoreGetTypeAllResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFields(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFields(
                match self.inner.get_fields(args.db_name,args.table_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetFieldsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsException::O1(ex))) => ThriftHiveMetastoreGetFieldsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsException::O2(ex))) => ThriftHiveMetastoreGetFieldsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsException::O3(ex))) => ThriftHiveMetastoreGetFieldsResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFieldsWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFieldsWithEnvironmentContext(
                match self.inner.get_fields_with_environment_context(args.db_name,args.table_name,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFieldsWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetSchema(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetSchema(
                match self.inner.get_schema(args.db_name,args.table_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetSchemaResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaException::O1(ex))) => ThriftHiveMetastoreGetSchemaResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaException::O2(ex))) => ThriftHiveMetastoreGetSchemaResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaException::O3(ex))) => ThriftHiveMetastoreGetSchemaResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetSchemaWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetSchemaWithEnvironmentContext(
                match self.inner.get_schema_with_environment_context(args.db_name,args.table_name,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetSchemaWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateTable(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateTable(
                match self.inner.create_table(args.tbl).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateTableResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableException::O1(ex))) => ThriftHiveMetastoreCreateTableResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableException::O2(ex))) => ThriftHiveMetastoreCreateTableResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableException::O3(ex))) => ThriftHiveMetastoreCreateTableResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableException::O4(ex))) => ThriftHiveMetastoreCreateTableResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateTableWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateTableWithEnvironmentContext(
                match self.inner.create_table_with_environment_context(args.tbl,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithEnvironmentContextException::O4(ex))) => ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateTableWithConstraints(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateTableWithConstraints(
                match self.inner.create_table_with_constraints(args.tbl,args.primary_keys,args.foreign_keys).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateTableWithConstraintsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithConstraintsException::O1(ex))) => ThriftHiveMetastoreCreateTableWithConstraintsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithConstraintsException::O2(ex))) => ThriftHiveMetastoreCreateTableWithConstraintsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithConstraintsException::O3(ex))) => ThriftHiveMetastoreCreateTableWithConstraintsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateTableWithConstraintsException::O4(ex))) => ThriftHiveMetastoreCreateTableWithConstraintsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropConstraint(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropConstraint(
                match self.inner.drop_constraint(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropConstraintResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropConstraintException::O1(ex))) => ThriftHiveMetastoreDropConstraintResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropConstraintException::O3(ex))) => ThriftHiveMetastoreDropConstraintResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPrimaryKey(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPrimaryKey(
                match self.inner.add_primary_key(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPrimaryKeyResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPrimaryKeyException::O1(ex))) => ThriftHiveMetastoreAddPrimaryKeyResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPrimaryKeyException::O2(ex))) => ThriftHiveMetastoreAddPrimaryKeyResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddForeignKey(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddForeignKey(
                match self.inner.add_foreign_key(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddForeignKeyResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddForeignKeyException::O1(ex))) => ThriftHiveMetastoreAddForeignKeyResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddForeignKeyException::O2(ex))) => ThriftHiveMetastoreAddForeignKeyResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropTable(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropTable(
                match self.inner.drop_table(args.dbname,args.name,args.delete_data).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropTableResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTableException::O1(ex))) => ThriftHiveMetastoreDropTableResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTableException::O3(ex))) => ThriftHiveMetastoreDropTableResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropTableWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropTableWithEnvironmentContext(
                match self.inner.drop_table_with_environment_context(args.dbname,args.name,args.delete_data,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropTableWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTableWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreDropTableWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropTableWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreDropTableWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTables(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTables(
                match self.inner.get_tables(args.db_name,args.pattern).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTablesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTablesException::O1(ex))) => ThriftHiveMetastoreGetTablesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTablesByType(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTablesByType(
                match self.inner.get_tables_by_type(args.db_name,args.pattern,args.table_type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTablesByTypeResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTablesByTypeException::O1(ex))) => ThriftHiveMetastoreGetTablesByTypeResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableMeta(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableMeta(
                match self.inner.get_table_meta(args.db_patterns,args.tbl_patterns,args.tbl_types).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableMetaResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableMetaException::O1(ex))) => ThriftHiveMetastoreGetTableMetaResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetAllTables(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetAllTables(
                match self.inner.get_all_tables(args.db_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetAllTablesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetAllTablesException::O1(ex))) => ThriftHiveMetastoreGetAllTablesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTable(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTable(
                match self.inner.get_table(args.dbname,args.tbl_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableException::O1(ex))) => ThriftHiveMetastoreGetTableResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableException::O2(ex))) => ThriftHiveMetastoreGetTableResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableObjectsByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableObjectsByName(
                match self.inner.get_table_objects_by_name(args.dbname,args.tbl_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableObjectsByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameException::O1(ex))) => ThriftHiveMetastoreGetTableObjectsByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameException::O2(ex))) => ThriftHiveMetastoreGetTableObjectsByNameResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameException::O3(ex))) => ThriftHiveMetastoreGetTableObjectsByNameResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableReq(
                match self.inner.get_table_req(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableReqException::O1(ex))) => ThriftHiveMetastoreGetTableReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableReqException::O2(ex))) => ThriftHiveMetastoreGetTableReqResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableObjectsByNameReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableObjectsByNameReq(
                match self.inner.get_table_objects_by_name_req(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableObjectsByNameReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameReqException::O1(ex))) => ThriftHiveMetastoreGetTableObjectsByNameReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameReqException::O2(ex))) => ThriftHiveMetastoreGetTableObjectsByNameReqResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableObjectsByNameReqException::O3(ex))) => ThriftHiveMetastoreGetTableObjectsByNameReqResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableNamesByFilter(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableNamesByFilter(
                match self.inner.get_table_names_by_filter(args.dbname,args.filter,args.max_tables).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableNamesByFilterResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableNamesByFilterException::O1(ex))) => ThriftHiveMetastoreGetTableNamesByFilterResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableNamesByFilterException::O2(ex))) => ThriftHiveMetastoreGetTableNamesByFilterResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableNamesByFilterException::O3(ex))) => ThriftHiveMetastoreGetTableNamesByFilterResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterTable(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterTable(
                match self.inner.alter_table(args.dbname,args.tbl_name,args.new_tbl).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterTableResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableException::O1(ex))) => ThriftHiveMetastoreAlterTableResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableException::O2(ex))) => ThriftHiveMetastoreAlterTableResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterTableWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterTableWithEnvironmentContext(
                match self.inner.alter_table_with_environment_context(args.dbname,args.tbl_name,args.new_tbl,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterTableWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAlterTableWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAlterTableWithEnvironmentContextResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterTableWithCascade(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterTableWithCascade(
                match self.inner.alter_table_with_cascade(args.dbname,args.tbl_name,args.new_tbl,args.cascade).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterTableWithCascadeResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableWithCascadeException::O1(ex))) => ThriftHiveMetastoreAlterTableWithCascadeResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterTableWithCascadeException::O2(ex))) => ThriftHiveMetastoreAlterTableWithCascadeResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPartition(
                match self.inner.add_partition(args.new_part).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionException::O1(ex))) => ThriftHiveMetastoreAddPartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionException::O2(ex))) => ThriftHiveMetastoreAddPartitionResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionException::O3(ex))) => ThriftHiveMetastoreAddPartitionResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPartitionWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPartitionWithEnvironmentContext(
                match self.inner.add_partition_with_environment_context(args.new_part,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPartitions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPartitions(
                match self.inner.add_partitions(args.new_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPartitionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsException::O1(ex))) => ThriftHiveMetastoreAddPartitionsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsException::O2(ex))) => ThriftHiveMetastoreAddPartitionsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsException::O3(ex))) => ThriftHiveMetastoreAddPartitionsResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPartitionsPspec(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPartitionsPspec(
                match self.inner.add_partitions_pspec(args.new_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPartitionsPspecResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsPspecException::O1(ex))) => ThriftHiveMetastoreAddPartitionsPspecResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsPspecException::O2(ex))) => ThriftHiveMetastoreAddPartitionsPspecResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsPspecException::O3(ex))) => ThriftHiveMetastoreAddPartitionsPspecResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AppendPartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AppendPartition(
                match self.inner.append_partition(args.db_name,args.tbl_name,args.part_vals).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAppendPartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionException::O1(ex))) => ThriftHiveMetastoreAppendPartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionException::O2(ex))) => ThriftHiveMetastoreAppendPartitionResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionException::O3(ex))) => ThriftHiveMetastoreAppendPartitionResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddPartitionsReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddPartitionsReq(
                match self.inner.add_partitions_req(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddPartitionsReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsReqException::O1(ex))) => ThriftHiveMetastoreAddPartitionsReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsReqException::O2(ex))) => ThriftHiveMetastoreAddPartitionsReqResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddPartitionsReqException::O3(ex))) => ThriftHiveMetastoreAddPartitionsReqResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AppendPartitionWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AppendPartitionWithEnvironmentContext(
                match self.inner.append_partition_with_environment_context(args.db_name,args.tbl_name,args.part_vals,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AppendPartitionByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AppendPartitionByName(
                match self.inner.append_partition_by_name(args.db_name,args.tbl_name,args.part_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAppendPartitionByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameException::O1(ex))) => ThriftHiveMetastoreAppendPartitionByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameException::O2(ex))) => ThriftHiveMetastoreAppendPartitionByNameResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameException::O3(ex))) => ThriftHiveMetastoreAppendPartitionByNameResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AppendPartitionByNameWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AppendPartitionByNameWithEnvironmentContext(
                match self.inner.append_partition_by_name_with_environment_context(args.db_name,args.tbl_name,args.part_name,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException::O3(ex))) => ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropPartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropPartition(
                match self.inner.drop_partition(args.db_name,args.tbl_name,args.part_vals,args.delete_data).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropPartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionException::O1(ex))) => ThriftHiveMetastoreDropPartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionException::O2(ex))) => ThriftHiveMetastoreDropPartitionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropPartitionWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropPartitionWithEnvironmentContext(
                match self.inner.drop_partition_with_environment_context(args.db_name,args.tbl_name,args.part_vals,args.delete_data,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropPartitionByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropPartitionByName(
                match self.inner.drop_partition_by_name(args.db_name,args.tbl_name,args.part_name,args.delete_data).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropPartitionByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionByNameException::O1(ex))) => ThriftHiveMetastoreDropPartitionByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionByNameException::O2(ex))) => ThriftHiveMetastoreDropPartitionByNameResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropPartitionByNameWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropPartitionByNameWithEnvironmentContext(
                match self.inner.drop_partition_by_name_with_environment_context(args.db_name,args.tbl_name,args.part_name,args.delete_data,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropPartitionsReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropPartitionsReq(
                match self.inner.drop_partitions_req(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropPartitionsReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionsReqException::O1(ex))) => ThriftHiveMetastoreDropPartitionsReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropPartitionsReqException::O2(ex))) => ThriftHiveMetastoreDropPartitionsReqResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartition(
                match self.inner.get_partition(args.db_name,args.tbl_name,args.part_vals).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionException::O1(ex))) => ThriftHiveMetastoreGetPartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionException::O2(ex))) => ThriftHiveMetastoreGetPartitionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ExchangePartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ExchangePartition(
                match self.inner.exchange_partition(args.partition_specs,args.source_db,args.source_table_name,args.dest_db,args.dest_table_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreExchangePartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionException::O1(ex))) => ThriftHiveMetastoreExchangePartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionException::O2(ex))) => ThriftHiveMetastoreExchangePartitionResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionException::O3(ex))) => ThriftHiveMetastoreExchangePartitionResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionException::O4(ex))) => ThriftHiveMetastoreExchangePartitionResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ExchangePartitions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ExchangePartitions(
                match self.inner.exchange_partitions(args.partition_specs,args.source_db,args.source_table_name,args.dest_db,args.dest_table_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreExchangePartitionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionsException::O1(ex))) => ThriftHiveMetastoreExchangePartitionsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionsException::O2(ex))) => ThriftHiveMetastoreExchangePartitionsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionsException::O3(ex))) => ThriftHiveMetastoreExchangePartitionsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreExchangePartitionsException::O4(ex))) => ThriftHiveMetastoreExchangePartitionsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionWithAuth(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionWithAuth(
                match self.inner.get_partition_with_auth(args.db_name,args.tbl_name,args.part_vals,args.user_name,args.group_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionWithAuthResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionWithAuthException::O1(ex))) => ThriftHiveMetastoreGetPartitionWithAuthResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionWithAuthException::O2(ex))) => ThriftHiveMetastoreGetPartitionWithAuthResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionByName(
                match self.inner.get_partition_by_name(args.db_name,args.tbl_name,args.part_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionByNameException::O1(ex))) => ThriftHiveMetastoreGetPartitionByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionByNameException::O2(ex))) => ThriftHiveMetastoreGetPartitionByNameResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitions(
                match self.inner.get_partitions(args.db_name,args.tbl_name,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsException::O1(ex))) => ThriftHiveMetastoreGetPartitionsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsException::O2(ex))) => ThriftHiveMetastoreGetPartitionsResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsWithAuth(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsWithAuth(
                match self.inner.get_partitions_with_auth(args.db_name,args.tbl_name,args.max_parts,args.user_name,args.group_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsWithAuthResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsWithAuthException::O1(ex))) => ThriftHiveMetastoreGetPartitionsWithAuthResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsWithAuthException::O2(ex))) => ThriftHiveMetastoreGetPartitionsWithAuthResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsPspec(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsPspec(
                match self.inner.get_partitions_pspec(args.db_name,args.tbl_name,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsPspecResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPspecException::O1(ex))) => ThriftHiveMetastoreGetPartitionsPspecResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPspecException::O2(ex))) => ThriftHiveMetastoreGetPartitionsPspecResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionNames(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionNames(
                match self.inner.get_partition_names(args.db_name,args.tbl_name,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionNamesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionNamesException::O2(ex))) => ThriftHiveMetastoreGetPartitionNamesResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionValues(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionValues(
                match self.inner.get_partition_values(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionValuesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionValuesException::O1(ex))) => ThriftHiveMetastoreGetPartitionValuesResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionValuesException::O2(ex))) => ThriftHiveMetastoreGetPartitionValuesResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsPs(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsPs(
                match self.inner.get_partitions_ps(args.db_name,args.tbl_name,args.part_vals,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsPsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPsException::O1(ex))) => ThriftHiveMetastoreGetPartitionsPsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPsException::O2(ex))) => ThriftHiveMetastoreGetPartitionsPsResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsPsWithAuth(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsPsWithAuth(
                match self.inner.get_partitions_ps_with_auth(args.db_name,args.tbl_name,args.part_vals,args.max_parts,args.user_name,args.group_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsPsWithAuthResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPsWithAuthException::O1(ex))) => ThriftHiveMetastoreGetPartitionsPsWithAuthResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsPsWithAuthException::O2(ex))) => ThriftHiveMetastoreGetPartitionsPsWithAuthResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionNamesPs(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionNamesPs(
                match self.inner.get_partition_names_ps(args.db_name,args.tbl_name,args.part_vals,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionNamesPsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionNamesPsException::O1(ex))) => ThriftHiveMetastoreGetPartitionNamesPsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionNamesPsException::O2(ex))) => ThriftHiveMetastoreGetPartitionNamesPsResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsByFilter(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsByFilter(
                match self.inner.get_partitions_by_filter(args.db_name,args.tbl_name,args.filter,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsByFilterResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByFilterException::O1(ex))) => ThriftHiveMetastoreGetPartitionsByFilterResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByFilterException::O2(ex))) => ThriftHiveMetastoreGetPartitionsByFilterResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartSpecsByFilter(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartSpecsByFilter(
                match self.inner.get_part_specs_by_filter(args.db_name,args.tbl_name,args.filter,args.max_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartSpecsByFilterResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartSpecsByFilterException::O1(ex))) => ThriftHiveMetastoreGetPartSpecsByFilterResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartSpecsByFilterException::O2(ex))) => ThriftHiveMetastoreGetPartSpecsByFilterResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsByExpr(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsByExpr(
                match self.inner.get_partitions_by_expr(args.req).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsByExprResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByExprException::O1(ex))) => ThriftHiveMetastoreGetPartitionsByExprResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByExprException::O2(ex))) => ThriftHiveMetastoreGetPartitionsByExprResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetNumPartitionsByFilter(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetNumPartitionsByFilter(
                match self.inner.get_num_partitions_by_filter(args.db_name,args.tbl_name,args.filter).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetNumPartitionsByFilterResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetNumPartitionsByFilterException::O1(ex))) => ThriftHiveMetastoreGetNumPartitionsByFilterResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetNumPartitionsByFilterException::O2(ex))) => ThriftHiveMetastoreGetNumPartitionsByFilterResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsByNames(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsByNames(
                match self.inner.get_partitions_by_names(args.db_name,args.tbl_name,args.names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsByNamesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByNamesException::O1(ex))) => ThriftHiveMetastoreGetPartitionsByNamesResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsByNamesException::O2(ex))) => ThriftHiveMetastoreGetPartitionsByNamesResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterPartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterPartition(
                match self.inner.alter_partition(args.db_name,args.tbl_name,args.new_part).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterPartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionException::O1(ex))) => ThriftHiveMetastoreAlterPartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionException::O2(ex))) => ThriftHiveMetastoreAlterPartitionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterPartitions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterPartitions(
                match self.inner.alter_partitions(args.db_name,args.tbl_name,args.new_parts).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterPartitionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionsException::O1(ex))) => ThriftHiveMetastoreAlterPartitionsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionsException::O2(ex))) => ThriftHiveMetastoreAlterPartitionsResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterPartitionsWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterPartitionsWithEnvironmentContext(
                match self.inner.alter_partitions_with_environment_context(args.db_name,args.tbl_name,args.new_parts,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterPartitionWithEnvironmentContext(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterPartitionWithEnvironmentContext(
                match self.inner.alter_partition_with_environment_context(args.db_name,args.tbl_name,args.new_part,args.environment_context).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O1(ex))) => ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException::O2(ex))) => ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RenamePartition(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RenamePartition(
                match self.inner.rename_partition(args.db_name,args.tbl_name,args.part_vals,args.new_part).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreRenamePartitionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreRenamePartitionException::O1(ex))) => ThriftHiveMetastoreRenamePartitionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreRenamePartitionException::O2(ex))) => ThriftHiveMetastoreRenamePartitionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::PartitionNameHasValidCharacters(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::PartitionNameHasValidCharacters(
                match self.inner.partition_name_has_valid_characters(args.part_vals,args.throw_exception).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastorePartitionNameHasValidCharactersResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastorePartitionNameHasValidCharactersException::O1(ex))) => ThriftHiveMetastorePartitionNameHasValidCharactersResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetConfigValue(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetConfigValue(
                match self.inner.get_config_value(args.name,args.default_value).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetConfigValueResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetConfigValueException::O1(ex))) => ThriftHiveMetastoreGetConfigValueResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::PartitionNameToVals(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::PartitionNameToVals(
                match self.inner.partition_name_to_vals(args.part_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastorePartitionNameToValsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastorePartitionNameToValsException::O1(ex))) => ThriftHiveMetastorePartitionNameToValsResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::PartitionNameToSpec(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::PartitionNameToSpec(
                match self.inner.partition_name_to_spec(args.part_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastorePartitionNameToSpecResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastorePartitionNameToSpecException::O1(ex))) => ThriftHiveMetastorePartitionNameToSpecResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::MarkPartitionForEvent(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::MarkPartitionForEvent(
                match self.inner.mark_partition_for_event(args.db_name,args.tbl_name,args.part_vals,args.event_type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreMarkPartitionForEventResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O1(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O2(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O3(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O4(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O4(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O5(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O5(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreMarkPartitionForEventException::O6(ex))) => ThriftHiveMetastoreMarkPartitionForEventResultSend::O6(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::IsPartitionMarkedForEvent(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::IsPartitionMarkedForEvent(
                match self.inner.is_partition_marked_for_event(args.db_name,args.tbl_name,args.part_vals,args.event_type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O1(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O2(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O3(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O4(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O4(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O5(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O5(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreIsPartitionMarkedForEventException::O6(ex))) => ThriftHiveMetastoreIsPartitionMarkedForEventResultSend::O6(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddIndex(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddIndex(
                match self.inner.add_index(args.new_index,args.index_table).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddIndexResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddIndexException::O1(ex))) => ThriftHiveMetastoreAddIndexResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddIndexException::O2(ex))) => ThriftHiveMetastoreAddIndexResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddIndexException::O3(ex))) => ThriftHiveMetastoreAddIndexResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterIndex(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterIndex(
                match self.inner.alter_index(args.dbname,args.base_tbl_name,args.idx_name,args.new_idx).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterIndexResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterIndexException::O1(ex))) => ThriftHiveMetastoreAlterIndexResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterIndexException::O2(ex))) => ThriftHiveMetastoreAlterIndexResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropIndexByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropIndexByName(
                match self.inner.drop_index_by_name(args.db_name,args.tbl_name,args.index_name,args.delete_data).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropIndexByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropIndexByNameException::O1(ex))) => ThriftHiveMetastoreDropIndexByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropIndexByNameException::O2(ex))) => ThriftHiveMetastoreDropIndexByNameResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetIndexByName(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetIndexByName(
                match self.inner.get_index_by_name(args.db_name,args.tbl_name,args.index_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetIndexByNameResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetIndexByNameException::O1(ex))) => ThriftHiveMetastoreGetIndexByNameResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetIndexByNameException::O2(ex))) => ThriftHiveMetastoreGetIndexByNameResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetIndexes(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetIndexes(
                match self.inner.get_indexes(args.db_name,args.tbl_name,args.max_indexes).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetIndexesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetIndexesException::O1(ex))) => ThriftHiveMetastoreGetIndexesResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetIndexesException::O2(ex))) => ThriftHiveMetastoreGetIndexesResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetIndexNames(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetIndexNames(
                match self.inner.get_index_names(args.db_name,args.tbl_name,args.max_indexes).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetIndexNamesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetIndexNamesException::O2(ex))) => ThriftHiveMetastoreGetIndexNamesResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPrimaryKeys(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPrimaryKeys(
                match self.inner.get_primary_keys(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPrimaryKeysResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPrimaryKeysException::O1(ex))) => ThriftHiveMetastoreGetPrimaryKeysResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPrimaryKeysException::O2(ex))) => ThriftHiveMetastoreGetPrimaryKeysResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetForeignKeys(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetForeignKeys(
                match self.inner.get_foreign_keys(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetForeignKeysResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetForeignKeysException::O1(ex))) => ThriftHiveMetastoreGetForeignKeysResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetForeignKeysException::O2(ex))) => ThriftHiveMetastoreGetForeignKeysResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::UpdateTableColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::UpdateTableColumnStatistics(
                match self.inner.update_table_column_statistics(args.stats_obj).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateTableColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateTableColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateTableColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateTableColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::UpdatePartitionColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::UpdatePartitionColumnStatistics(
                match self.inner.update_partition_column_statistics(args.stats_obj).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdatePartitionColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableColumnStatistics(
                match self.inner.get_table_column_statistics(args.db_name,args.tbl_name,args.col_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreGetTableColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreGetTableColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreGetTableColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreGetTableColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionColumnStatistics(
                match self.inner.get_partition_column_statistics(args.db_name,args.tbl_name,args.part_name,args.col_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetTableStatisticsReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetTableStatisticsReq(
                match self.inner.get_table_statistics_req(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetTableStatisticsReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableStatisticsReqException::O1(ex))) => ThriftHiveMetastoreGetTableStatisticsReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetTableStatisticsReqException::O2(ex))) => ThriftHiveMetastoreGetTableStatisticsReqResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPartitionsStatisticsReq(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPartitionsStatisticsReq(
                match self.inner.get_partitions_statistics_req(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPartitionsStatisticsReqResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsStatisticsReqException::O1(ex))) => ThriftHiveMetastoreGetPartitionsStatisticsReqResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPartitionsStatisticsReqException::O2(ex))) => ThriftHiveMetastoreGetPartitionsStatisticsReqResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetAggrStatsFor(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetAggrStatsFor(
                match self.inner.get_aggr_stats_for(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetAggrStatsForResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetAggrStatsForException::O1(ex))) => ThriftHiveMetastoreGetAggrStatsForResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetAggrStatsForException::O2(ex))) => ThriftHiveMetastoreGetAggrStatsForResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::SetAggrStatsFor(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::SetAggrStatsFor(
                match self.inner.set_aggr_stats_for(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreSetAggrStatsForResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetAggrStatsForException::O1(ex))) => ThriftHiveMetastoreSetAggrStatsForResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetAggrStatsForException::O2(ex))) => ThriftHiveMetastoreSetAggrStatsForResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetAggrStatsForException::O3(ex))) => ThriftHiveMetastoreSetAggrStatsForResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetAggrStatsForException::O4(ex))) => ThriftHiveMetastoreSetAggrStatsForResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DeletePartitionColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DeletePartitionColumnStatistics(
                match self.inner.delete_partition_column_statistics(args.db_name,args.tbl_name,args.part_name,args.col_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeletePartitionColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DeleteTableColumnStatistics(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DeleteTableColumnStatistics(
                match self.inner.delete_table_column_statistics(args.db_name,args.tbl_name,args.col_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeleteTableColumnStatisticsException::O1(ex))) => ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeleteTableColumnStatisticsException::O2(ex))) => ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeleteTableColumnStatisticsException::O3(ex))) => ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDeleteTableColumnStatisticsException::O4(ex))) => ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateFunction(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateFunction(
                match self.inner.create_function(args.func).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateFunctionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateFunctionException::O1(ex))) => ThriftHiveMetastoreCreateFunctionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateFunctionException::O2(ex))) => ThriftHiveMetastoreCreateFunctionResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateFunctionException::O3(ex))) => ThriftHiveMetastoreCreateFunctionResultSend::O3(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateFunctionException::O4(ex))) => ThriftHiveMetastoreCreateFunctionResultSend::O4(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropFunction(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropFunction(
                match self.inner.drop_function(args.db_name,args.func_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropFunctionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropFunctionException::O1(ex))) => ThriftHiveMetastoreDropFunctionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropFunctionException::O3(ex))) => ThriftHiveMetastoreDropFunctionResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AlterFunction(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AlterFunction(
                match self.inner.alter_function(args.db_name,args.func_name,args.new_func).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAlterFunctionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterFunctionException::O1(ex))) => ThriftHiveMetastoreAlterFunctionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAlterFunctionException::O2(ex))) => ThriftHiveMetastoreAlterFunctionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFunctions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFunctions(
                match self.inner.get_functions(args.db_name,args.pattern).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetFunctionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFunctionsException::O1(ex))) => ThriftHiveMetastoreGetFunctionsResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFunction(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFunction(
                match self.inner.get_function(args.db_name,args.func_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetFunctionResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFunctionException::O1(ex))) => ThriftHiveMetastoreGetFunctionResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetFunctionException::O2(ex))) => ThriftHiveMetastoreGetFunctionResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetAllFunctions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetAllFunctions(
                match self.inner.get_all_functions().await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetAllFunctionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetAllFunctionsException::O1(ex))) => ThriftHiveMetastoreGetAllFunctionsResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CreateRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CreateRole(
                match self.inner.create_role(args.role).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCreateRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCreateRoleException::O1(ex))) => ThriftHiveMetastoreCreateRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::DropRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::DropRole(
                match self.inner.drop_role(args.role_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreDropRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreDropRoleException::O1(ex))) => ThriftHiveMetastoreDropRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetRoleNames(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetRoleNames(
                match self.inner.get_role_names().await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetRoleNamesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetRoleNamesException::O1(ex))) => ThriftHiveMetastoreGetRoleNamesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GrantRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GrantRole(
                match self.inner.grant_role(args.role_name,args.principal_name,args.principal_type,args.grantor,args.grantor_type,args.grant_option).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGrantRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGrantRoleException::O1(ex))) => ThriftHiveMetastoreGrantRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RevokeRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RevokeRole(
                match self.inner.revoke_role(args.role_name,args.principal_name,args.principal_type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreRevokeRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreRevokeRoleException::O1(ex))) => ThriftHiveMetastoreRevokeRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ListRoles(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ListRoles(
                match self.inner.list_roles(args.principal_name,args.principal_type).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreListRolesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreListRolesException::O1(ex))) => ThriftHiveMetastoreListRolesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GrantRevokeRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GrantRevokeRole(
                match self.inner.grant_revoke_role(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGrantRevokeRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGrantRevokeRoleException::O1(ex))) => ThriftHiveMetastoreGrantRevokeRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPrincipalsInRole(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPrincipalsInRole(
                match self.inner.get_principals_in_role(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPrincipalsInRoleResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPrincipalsInRoleException::O1(ex))) => ThriftHiveMetastoreGetPrincipalsInRoleResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetRoleGrantsForPrincipal(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetRoleGrantsForPrincipal(
                match self.inner.get_role_grants_for_principal(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetRoleGrantsForPrincipalResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetRoleGrantsForPrincipalException::O1(ex))) => ThriftHiveMetastoreGetRoleGrantsForPrincipalResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetPrivilegeSet(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetPrivilegeSet(
                match self.inner.get_privilege_set(args.hive_object,args.user_name,args.group_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetPrivilegeSetResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetPrivilegeSetException::O1(ex))) => ThriftHiveMetastoreGetPrivilegeSetResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ListPrivileges(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ListPrivileges(
                match self.inner.list_privileges(args.principal_name,args.principal_type,args.hive_object).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreListPrivilegesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreListPrivilegesException::O1(ex))) => ThriftHiveMetastoreListPrivilegesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GrantPrivileges(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GrantPrivileges(
                match self.inner.grant_privileges(args.privileges).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGrantPrivilegesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGrantPrivilegesException::O1(ex))) => ThriftHiveMetastoreGrantPrivilegesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RevokePrivileges(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RevokePrivileges(
                match self.inner.revoke_privileges(args.privileges).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreRevokePrivilegesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreRevokePrivilegesException::O1(ex))) => ThriftHiveMetastoreRevokePrivilegesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GrantRevokePrivileges(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GrantRevokePrivileges(
                match self.inner.grant_revoke_privileges(args.request).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGrantRevokePrivilegesResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGrantRevokePrivilegesException::O1(ex))) => ThriftHiveMetastoreGrantRevokePrivilegesResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::SetUgi(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::SetUgi(
                match self.inner.set_ugi(args.user_name,args.group_names).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreSetUgiResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreSetUgiException::O1(ex))) => ThriftHiveMetastoreSetUgiResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetDelegationToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetDelegationToken(
                match self.inner.get_delegation_token(args.token_owner,args.renewer_kerberos_principal_name).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreGetDelegationTokenResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreGetDelegationTokenException::O1(ex))) => ThriftHiveMetastoreGetDelegationTokenResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RenewDelegationToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RenewDelegationToken(
                match self.inner.renew_delegation_token(args.token_str_form).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreRenewDelegationTokenResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreRenewDelegationTokenException::O1(ex))) => ThriftHiveMetastoreRenewDelegationTokenResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CancelDelegationToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CancelDelegationToken(
                match self.inner.cancel_delegation_token(args.token_str_form).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCancelDelegationTokenResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCancelDelegationTokenException::O1(ex))) => ThriftHiveMetastoreCancelDelegationTokenResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddToken(
                match self.inner.add_token(args.token_identifier,args.delegation_token).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreAddTokenResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RemoveToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RemoveToken(
                match self.inner.remove_token(args.token_identifier).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreRemoveTokenResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetToken(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetToken(
                match self.inner.get_token(args.token_identifier).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetTokenResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetAllTokenIdentifiers(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetAllTokenIdentifiers(
                match self.inner.get_all_token_identifiers().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetAllTokenIdentifiersResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddMasterKey(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddMasterKey(
                match self.inner.add_master_key(args.key).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddMasterKeyResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddMasterKeyException::O1(ex))) => ThriftHiveMetastoreAddMasterKeyResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::UpdateMasterKey(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::UpdateMasterKey(
                match self.inner.update_master_key(args.seq_number,args.key).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreUpdateMasterKeyResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateMasterKeyException::O1(ex))) => ThriftHiveMetastoreUpdateMasterKeyResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUpdateMasterKeyException::O2(ex))) => ThriftHiveMetastoreUpdateMasterKeyResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::RemoveMasterKey(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::RemoveMasterKey(
                match self.inner.remove_master_key(args.key_seq).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreRemoveMasterKeyResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetMasterKeys(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetMasterKeys(
                match self.inner.get_master_keys().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetMasterKeysResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetOpenTxns(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetOpenTxns(
                match self.inner.get_open_txns().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetOpenTxnsResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetOpenTxnsInfo(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetOpenTxnsInfo(
                match self.inner.get_open_txns_info().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetOpenTxnsInfoResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::OpenTxns(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::OpenTxns(
                match self.inner.open_txns(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreOpenTxnsResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AbortTxn(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AbortTxn(
                match self.inner.abort_txn(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAbortTxnResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAbortTxnException::O1(ex))) => ThriftHiveMetastoreAbortTxnResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AbortTxns(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AbortTxns(
                match self.inner.abort_txns(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAbortTxnsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAbortTxnsException::O1(ex))) => ThriftHiveMetastoreAbortTxnsResultSend::O1(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CommitTxn(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CommitTxn(
                match self.inner.commit_txn(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCommitTxnResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCommitTxnException::O1(ex))) => ThriftHiveMetastoreCommitTxnResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCommitTxnException::O2(ex))) => ThriftHiveMetastoreCommitTxnResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::Lock(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::Lock(
                match self.inner.lock(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreLockResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreLockException::O1(ex))) => ThriftHiveMetastoreLockResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreLockException::O2(ex))) => ThriftHiveMetastoreLockResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CheckLock(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CheckLock(
                match self.inner.check_lock(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreCheckLockResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCheckLockException::O1(ex))) => ThriftHiveMetastoreCheckLockResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCheckLockException::O2(ex))) => ThriftHiveMetastoreCheckLockResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreCheckLockException::O3(ex))) => ThriftHiveMetastoreCheckLockResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::Unlock(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::Unlock(
                match self.inner.unlock(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreUnlockResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUnlockException::O1(ex))) => ThriftHiveMetastoreUnlockResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreUnlockException::O2(ex))) => ThriftHiveMetastoreUnlockResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ShowLocks(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ShowLocks(
                match self.inner.show_locks(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreShowLocksResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::Heartbeat(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::Heartbeat(
                match self.inner.heartbeat(args.ids).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreHeartbeatResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreHeartbeatException::O1(ex))) => ThriftHiveMetastoreHeartbeatResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreHeartbeatException::O2(ex))) => ThriftHiveMetastoreHeartbeatResultSend::O2(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreHeartbeatException::O3(ex))) => ThriftHiveMetastoreHeartbeatResultSend::O3(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::HeartbeatTxnRange(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::HeartbeatTxnRange(
                match self.inner.heartbeat_txn_range(args.txns).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreHeartbeatTxnRangeResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::Compact(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::Compact(
                match self.inner.compact(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreCompactResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::Compact2(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::Compact2(
                match self.inner.compact2(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreCompact2ResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ShowCompact(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ShowCompact(
                match self.inner.show_compact(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreShowCompactResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::AddDynamicPartitions(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::AddDynamicPartitions(
                match self.inner.add_dynamic_partitions(args.rqst).await {
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Ok(resp)) => ThriftHiveMetastoreAddDynamicPartitionsResultSend::Ok(resp),
                        ::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddDynamicPartitionsException::O1(ex))) => ThriftHiveMetastoreAddDynamicPartitionsResultSend::O1(ex),::std::result::Result::Ok(::volo_thrift::MaybeException::Exception(ThriftHiveMetastoreAddDynamicPartitionsException::O2(ex))) => ThriftHiveMetastoreAddDynamicPartitionsResultSend::O2(ex),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetNextNotification(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetNextNotification(
                match self.inner.get_next_notification(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetNextNotificationResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetCurrentNotificationEventId(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetCurrentNotificationEventId(
                match self.inner.get_current_notification_event_id().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetCurrentNotificationEventIdResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::FireListenerEvent(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::FireListenerEvent(
                match self.inner.fire_listener_event(args.rqst).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreFireListenerEventResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::FlushCache(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::FlushCache(
                match self.inner.flush_cache().await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreFlushCacheResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFileMetadataByExpr(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFileMetadataByExpr(
                match self.inner.get_file_metadata_by_expr(args.req).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetFileMetadataByExprResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::GetFileMetadata(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::GetFileMetadata(
                match self.inner.get_file_metadata(args.req).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreGetFileMetadataResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::PutFileMetadata(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::PutFileMetadata(
                match self.inner.put_file_metadata(args.req).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastorePutFileMetadataResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::ClearFileMetadata(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::ClearFileMetadata(
                match self.inner.clear_file_metadata(args.req).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreClearFileMetadataResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),ThriftHiveMetastoreRequestRecv::CacheFileMetadata(args) => ::std::result::Result::Ok(
            ThriftHiveMetastoreResponseSend::CacheFileMetadata(
                match self.inner.cache_file_metadata(args.req).await {
                        ::std::result::Result::Ok(resp) => ThriftHiveMetastoreCacheFileMetadataResultSend::Ok(resp),
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    }
            )),
                    }
    }
}
