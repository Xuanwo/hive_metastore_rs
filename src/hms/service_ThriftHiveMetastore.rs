
pub trait ThriftHiveMetastore {
    fn get_meta_conf(
        &self,
        key: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::pilota::FastStr,
                ThriftHiveMetastoreGetMetaConfException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn set_meta_conf(
        &self,
        key: ::pilota::FastStr,
        value: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreSetMetaConfException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_database(
        &self,
        database: Database,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateDatabaseException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_database(
        &self,
        name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Database, ThriftHiveMetastoreGetDatabaseException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_database(
        &self,
        name: ::pilota::FastStr,
        delete_data: bool,
        cascade: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropDatabaseException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_databases(
        &self,
        pattern: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetDatabasesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_all_databases(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetAllDatabasesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_database(
        &self,
        dbname: ::pilota::FastStr,
        db: Database,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterDatabaseException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_type(
        &self,
        name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Type, ThriftHiveMetastoreGetTypeException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_type(
        &self,
        r#type: Type,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateTypeException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_type(
        &self,
        r#type: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropTypeException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_type_all(
        &self,
        name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::pilota::AHashMap<::pilota::FastStr, Type>,
                ThriftHiveMetastoreGetTypeAllException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_fields(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<FieldSchema>,
                ThriftHiveMetastoreGetFieldsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_fields_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<FieldSchema>,
                ThriftHiveMetastoreGetFieldsWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_schema(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<FieldSchema>,
                ThriftHiveMetastoreGetSchemaException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_schema_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        table_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<FieldSchema>,
                ThriftHiveMetastoreGetSchemaWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_table(
        &self,
        tbl: Table,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateTableException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_table_with_environment_context(
        &self,
        tbl: Table,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreCreateTableWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_table_with_constraints(
        &self,
        tbl: Table,
        primary_keys: ::std::vec::Vec<SqlPrimaryKey>,
        foreign_keys: ::std::vec::Vec<SqlForeignKey>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreCreateTableWithConstraintsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_constraint(
        &self,
        req: DropConstraintRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropConstraintException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_primary_key(
        &self,
        req: AddPrimaryKeyRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddPrimaryKeyException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_foreign_key(
        &self,
        req: AddForeignKeyRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddForeignKeyException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_table(
        &self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropTableException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_table_with_environment_context(
        &self,
        dbname: ::pilota::FastStr,
        name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreDropTableWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_tables(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetTablesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_tables_by_type(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
        table_type: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetTablesByTypeException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_meta(
        &self,
        db_patterns: ::pilota::FastStr,
        tbl_patterns: ::pilota::FastStr,
        tbl_types: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<TableMeta>,
                ThriftHiveMetastoreGetTableMetaException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_all_tables(
        &self,
        db_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetAllTablesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Table, ThriftHiveMetastoreGetTableException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_objects_by_name(
        &self,
        dbname: ::pilota::FastStr,
        tbl_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Table>,
                ThriftHiveMetastoreGetTableObjectsByNameException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_req(
        &self,
        req: GetTableRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<GetTableResult, ThriftHiveMetastoreGetTableReqException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_objects_by_name_req(
        &self,
        req: GetTablesRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GetTablesResult,
                ThriftHiveMetastoreGetTableObjectsByNameReqException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_names_by_filter(
        &self,
        dbname: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_tables: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetTableNamesByFilterException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_table(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_table_with_environment_context(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreAlterTableWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_table_with_cascade(
        &self,
        dbname: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_tbl: Table,
        cascade: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterTableWithCascadeException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_partition(
        &self,
        new_part: Partition,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAddPartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_partition_with_environment_context(
        &self,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreAddPartitionWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_partitions(
        &self,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_partitions_pspec(
        &self,
        new_parts: ::std::vec::Vec<PartitionSpec>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddPartitionsPspecException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn append_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreAppendPartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_partitions_req(
        &self,
        request: AddPartitionsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                AddPartitionsResult,
                ThriftHiveMetastoreAddPartitionsReqException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn append_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreAppendPartitionWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn append_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreAppendPartitionByNameException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn append_partition_by_name_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreDropPartitionWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropPartitionByNameException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_partition_by_name_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        delete_data: bool,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_partitions_req(
        &self,
        req: DropPartitionsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                DropPartitionsResult,
                ThriftHiveMetastoreDropPartitionsReqException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreGetPartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn exchange_partition(
        &self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Partition, ThriftHiveMetastoreExchangePartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn exchange_partitions(
        &self,
        partition_specs: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        source_db: ::pilota::FastStr,
        source_table_name: ::pilota::FastStr,
        dest_db: ::pilota::FastStr,
        dest_table_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreExchangePartitionsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreGetPartitionWithAuthException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                Partition,
                ThriftHiveMetastoreGetPartitionByNameException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsWithAuthException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_pspec(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i32,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<PartitionSpec>,
                ThriftHiveMetastoreGetPartitionsPspecException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_parts: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetPartitionNamesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_values(
        &self,
        request: PartitionValuesRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                PartitionValuesResponse,
                ThriftHiveMetastoreGetPartitionValuesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_ps(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsPsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_ps_with_auth(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsPsWithAuthException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_names_ps(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        max_parts: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetPartitionNamesPsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsByFilterException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_part_specs_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
        max_parts: i32,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<PartitionSpec>,
                ThriftHiveMetastoreGetPartSpecsByFilterException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_by_expr(
        &self,
        req: PartitionsByExprRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                PartitionsByExprResult,
                ThriftHiveMetastoreGetPartitionsByExprException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_num_partitions_by_filter(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        filter: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                i32,
                ThriftHiveMetastoreGetNumPartitionsByFilterException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_by_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Partition>,
                ThriftHiveMetastoreGetPartitionsByNamesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_partitions(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterPartitionsException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_partitions_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_parts: ::std::vec::Vec<Partition>,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_partition_with_environment_context(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        new_part: Partition,
        environment_context: EnvironmentContext,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                (),
                ThriftHiveMetastoreAlterPartitionWithEnvironmentContextException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn rename_partition(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        new_part: Partition,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreRenamePartitionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn partition_name_has_valid_characters(
        &self,
        part_vals: ::std::vec::Vec<::pilota::FastStr>,
        throw_exception: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastorePartitionNameHasValidCharactersException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_config_value(
        &self,
        name: ::pilota::FastStr,
        default_value: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::pilota::FastStr,
                ThriftHiveMetastoreGetConfigValueException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn partition_name_to_vals(
        &self,
        part_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastorePartitionNameToValsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn partition_name_to_spec(
        &self,
        part_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
                ThriftHiveMetastorePartitionNameToSpecException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn mark_partition_for_event(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreMarkPartitionForEventException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn is_partition_marked_for_event(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_vals: ::pilota::AHashMap<::pilota::FastStr, ::pilota::FastStr>,
        event_type: PartitionEventType,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreIsPartitionMarkedForEventException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_index(
        &self,
        new_index: Index,
        index_table: Table,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreAddIndexException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_index(
        &self,
        dbname: ::pilota::FastStr,
        base_tbl_name: ::pilota::FastStr,
        idx_name: ::pilota::FastStr,
        new_idx: Index,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterIndexException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_index_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
        delete_data: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropIndexByNameException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_index_by_name(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        index_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Index, ThriftHiveMetastoreGetIndexByNameException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_indexes(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Index>,
                ThriftHiveMetastoreGetIndexesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_index_names(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        max_indexes: i16,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetIndexNamesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_primary_keys(
        &self,
        request: PrimaryKeysRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                PrimaryKeysResponse,
                ThriftHiveMetastoreGetPrimaryKeysException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_foreign_keys(
        &self,
        request: ForeignKeysRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ForeignKeysResponse,
                ThriftHiveMetastoreGetForeignKeysException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn update_table_column_statistics(
        &self,
        stats_obj: ColumnStatistics,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreUpdateTableColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn update_partition_column_statistics(
        &self,
        stats_obj: ColumnStatistics,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreUpdatePartitionColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ColumnStatistics,
                ThriftHiveMetastoreGetTableColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partition_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ColumnStatistics,
                ThriftHiveMetastoreGetPartitionColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_table_statistics_req(
        &self,
        request: TableStatsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                TableStatsResult,
                ThriftHiveMetastoreGetTableStatisticsReqException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_partitions_statistics_req(
        &self,
        request: PartitionsStatsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                PartitionsStatsResult,
                ThriftHiveMetastoreGetPartitionsStatisticsReqException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_aggr_stats_for(
        &self,
        request: PartitionsStatsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<AggrStats, ThriftHiveMetastoreGetAggrStatsForException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn set_aggr_stats_for(
        &self,
        request: SetPartitionsStatsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreSetAggrStatsForException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn delete_partition_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        part_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreDeletePartitionColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn delete_table_column_statistics(
        &self,
        db_name: ::pilota::FastStr,
        tbl_name: ::pilota::FastStr,
        col_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                bool,
                ThriftHiveMetastoreDeleteTableColumnStatisticsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_function(
        &self,
        func: Function,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCreateFunctionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreDropFunctionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn alter_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
        new_func: Function,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAlterFunctionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_functions(
        &self,
        db_name: ::pilota::FastStr,
        pattern: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetFunctionsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_function(
        &self,
        db_name: ::pilota::FastStr,
        func_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<Function, ThriftHiveMetastoreGetFunctionException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_all_functions(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GetAllFunctionsResponse,
                ThriftHiveMetastoreGetAllFunctionsException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn create_role(
        &self,
        role: Role,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreCreateRoleException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn drop_role(
        &self,
        role_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreDropRoleException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_role_names(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreGetRoleNamesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn grant_role(
        &self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        grantor: ::pilota::FastStr,
        grantor_type: PrincipalType,
        grant_option: bool,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantRoleException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn revoke_role(
        &self,
        role_name: ::pilota::FastStr,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokeRoleException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn list_roles(
        &self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<Role>,
                ThriftHiveMetastoreListRolesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn grant_revoke_role(
        &self,
        request: GrantRevokeRoleRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GrantRevokeRoleResponse,
                ThriftHiveMetastoreGrantRevokeRoleException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_principals_in_role(
        &self,
        request: GetPrincipalsInRoleRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GetPrincipalsInRoleResponse,
                ThriftHiveMetastoreGetPrincipalsInRoleException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_role_grants_for_principal(
        &self,
        request: GetRoleGrantsForPrincipalRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GetRoleGrantsForPrincipalResponse,
                ThriftHiveMetastoreGetRoleGrantsForPrincipalException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_privilege_set(
        &self,
        hive_object: HiveObjectRef,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                PrincipalPrivilegeSet,
                ThriftHiveMetastoreGetPrivilegeSetException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn list_privileges(
        &self,
        principal_name: ::pilota::FastStr,
        principal_type: PrincipalType,
        hive_object: HiveObjectRef,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<HiveObjectPrivilege>,
                ThriftHiveMetastoreListPrivilegesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn grant_privileges(
        &self,
        privileges: PrivilegeBag,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreGrantPrivilegesException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn revoke_privileges(
        &self,
        privileges: PrivilegeBag,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<bool, ThriftHiveMetastoreRevokePrivilegesException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn grant_revoke_privileges(
        &self,
        request: GrantRevokePrivilegeRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                GrantRevokePrivilegeResponse,
                ThriftHiveMetastoreGrantRevokePrivilegesException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn set_ugi(
        &self,
        user_name: ::pilota::FastStr,
        group_names: ::std::vec::Vec<::pilota::FastStr>,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::std::vec::Vec<::pilota::FastStr>,
                ThriftHiveMetastoreSetUgiException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_delegation_token(
        &self,
        token_owner: ::pilota::FastStr,
        renewer_kerberos_principal_name: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<
                ::pilota::FastStr,
                ThriftHiveMetastoreGetDelegationTokenException,
            >,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn renew_delegation_token(
        &self,
        token_str_form: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<i64, ThriftHiveMetastoreRenewDelegationTokenException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn cancel_delegation_token(
        &self,
        token_str_form: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCancelDelegationTokenException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_token(
        &self,
        token_identifier: ::pilota::FastStr,
        delegation_token: ::pilota::FastStr,
    ) -> impl ::std::future::Future<Output = ::core::result::Result<bool, ::volo_thrift::ServerError>>
           + Send;
    fn remove_token(
        &self,
        token_identifier: ::pilota::FastStr,
    ) -> impl ::std::future::Future<Output = ::core::result::Result<bool, ::volo_thrift::ServerError>>
           + Send;
    fn get_token(
        &self,
        token_identifier: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<::pilota::FastStr, ::volo_thrift::ServerError>,
    > + Send;
    fn get_all_token_identifiers(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::std::vec::Vec<::pilota::FastStr>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn add_master_key(
        &self,
        key: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<i32, ThriftHiveMetastoreAddMasterKeyException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn update_master_key(
        &self,
        seq_number: i32,
        key: ::pilota::FastStr,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUpdateMasterKeyException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn remove_master_key(
        &self,
        key_seq: i32,
    ) -> impl ::std::future::Future<Output = ::core::result::Result<bool, ::volo_thrift::ServerError>>
           + Send;
    fn get_master_keys(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::std::vec::Vec<::pilota::FastStr>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_open_txns(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<GetOpenTxnsResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn get_open_txns_info(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<GetOpenTxnsInfoResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn open_txns(
        &self,
        rqst: OpenTxnRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<OpenTxnsResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn abort_txn(
        &self,
        rqst: AbortTxnRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn abort_txns(
        &self,
        rqst: AbortTxnsRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAbortTxnsException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn commit_txn(
        &self,
        rqst: CommitTxnRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreCommitTxnException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn lock(
        &self,
        rqst: LockRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreLockException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn check_lock(
        &self,
        rqst: CheckLockRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<LockResponse, ThriftHiveMetastoreCheckLockException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn unlock(
        &self,
        rqst: UnlockRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreUnlockException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn show_locks(
        &self,
        rqst: ShowLocksRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<ShowLocksResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn heartbeat(
        &self,
        ids: HeartbeatRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreHeartbeatException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn heartbeat_txn_range(
        &self,
        txns: HeartbeatTxnRangeRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<HeartbeatTxnRangeResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn compact(
        &self,
        rqst: CompactionRequest,
    ) -> impl ::std::future::Future<Output = ::core::result::Result<(), ::volo_thrift::ServerError>> + Send;
    fn compact2(
        &self,
        rqst: CompactionRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<CompactionResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn show_compact(
        &self,
        rqst: ShowCompactRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<ShowCompactResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn add_dynamic_partitions(
        &self,
        rqst: AddDynamicPartitions,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<
            ::volo_thrift::MaybeException<(), ThriftHiveMetastoreAddDynamicPartitionsException>,
            ::volo_thrift::ServerError,
        >,
    > + Send;
    fn get_next_notification(
        &self,
        rqst: NotificationEventRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<NotificationEventResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn get_current_notification_event_id(
        &self,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<CurrentNotificationEventId, ::volo_thrift::ServerError>,
    > + Send;
    fn fire_listener_event(
        &self,
        rqst: FireEventRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<FireEventResponse, ::volo_thrift::ServerError>,
    > + Send;
    fn flush_cache(
        &self,
    ) -> impl ::std::future::Future<Output = ::core::result::Result<(), ::volo_thrift::ServerError>> + Send;
    fn get_file_metadata_by_expr(
        &self,
        req: GetFileMetadataByExprRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<GetFileMetadataByExprResult, ::volo_thrift::ServerError>,
    > + Send;
    fn get_file_metadata(
        &self,
        req: GetFileMetadataRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<GetFileMetadataResult, ::volo_thrift::ServerError>,
    > + Send;
    fn put_file_metadata(
        &self,
        req: PutFileMetadataRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<PutFileMetadataResult, ::volo_thrift::ServerError>,
    > + Send;
    fn clear_file_metadata(
        &self,
        req: ClearFileMetadataRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<ClearFileMetadataResult, ::volo_thrift::ServerError>,
    > + Send;
    fn cache_file_metadata(
        &self,
        req: CacheFileMetadataRequest,
    ) -> impl ::std::future::Future<
        Output = ::core::result::Result<CacheFileMetadataResult, ::volo_thrift::ServerError>,
    > + Send;
}
include!("ThriftHiveMetastore/mod.rs");
