#[derive(Debug, Clone)]
pub enum ThriftHiveMetastoreResponseSend {
    GetMetaConf(ThriftHiveMetastoreGetMetaConfResultSend),
    SetMetaConf(ThriftHiveMetastoreSetMetaConfResultSend),
    CreateDatabase(ThriftHiveMetastoreCreateDatabaseResultSend),
    GetDatabase(ThriftHiveMetastoreGetDatabaseResultSend),
    DropDatabase(ThriftHiveMetastoreDropDatabaseResultSend),
    GetDatabases(ThriftHiveMetastoreGetDatabasesResultSend),
    GetAllDatabases(ThriftHiveMetastoreGetAllDatabasesResultSend),
    AlterDatabase(ThriftHiveMetastoreAlterDatabaseResultSend),
    GetType(ThriftHiveMetastoreGetTypeResultSend),
    CreateType(ThriftHiveMetastoreCreateTypeResultSend),
    DropType(ThriftHiveMetastoreDropTypeResultSend),
    GetTypeAll(ThriftHiveMetastoreGetTypeAllResultSend),
    GetFields(ThriftHiveMetastoreGetFieldsResultSend),
    GetFieldsWithEnvironmentContext(ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend),
    GetSchema(ThriftHiveMetastoreGetSchemaResultSend),
    GetSchemaWithEnvironmentContext(ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend),
    CreateTable(ThriftHiveMetastoreCreateTableResultSend),
    CreateTableWithEnvironmentContext(
        ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend,
    ),
    CreateTableWithConstraints(ThriftHiveMetastoreCreateTableWithConstraintsResultSend),
    DropConstraint(ThriftHiveMetastoreDropConstraintResultSend),
    AddPrimaryKey(ThriftHiveMetastoreAddPrimaryKeyResultSend),
    AddForeignKey(ThriftHiveMetastoreAddForeignKeyResultSend),
    DropTable(ThriftHiveMetastoreDropTableResultSend),
    DropTableWithEnvironmentContext(ThriftHiveMetastoreDropTableWithEnvironmentContextResultSend),
    GetTables(ThriftHiveMetastoreGetTablesResultSend),
    GetTablesByType(ThriftHiveMetastoreGetTablesByTypeResultSend),
    GetTableMeta(ThriftHiveMetastoreGetTableMetaResultSend),
    GetAllTables(ThriftHiveMetastoreGetAllTablesResultSend),
    GetTable(ThriftHiveMetastoreGetTableResultSend),
    GetTableObjectsByName(ThriftHiveMetastoreGetTableObjectsByNameResultSend),
    GetTableReq(ThriftHiveMetastoreGetTableReqResultSend),
    GetTableObjectsByNameReq(ThriftHiveMetastoreGetTableObjectsByNameReqResultSend),
    GetTableNamesByFilter(ThriftHiveMetastoreGetTableNamesByFilterResultSend),
    AlterTable(ThriftHiveMetastoreAlterTableResultSend),
    AlterTableWithEnvironmentContext(ThriftHiveMetastoreAlterTableWithEnvironmentContextResultSend),
    AlterTableWithCascade(ThriftHiveMetastoreAlterTableWithCascadeResultSend),
    AddPartition(ThriftHiveMetastoreAddPartitionResultSend),
    AddPartitionWithEnvironmentContext(
        ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend,
    ),
    AddPartitions(ThriftHiveMetastoreAddPartitionsResultSend),
    AddPartitionsPspec(ThriftHiveMetastoreAddPartitionsPspecResultSend),
    AppendPartition(ThriftHiveMetastoreAppendPartitionResultSend),
    AddPartitionsReq(ThriftHiveMetastoreAddPartitionsReqResultSend),
    AppendPartitionWithEnvironmentContext(
        ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend,
    ),
    AppendPartitionByName(ThriftHiveMetastoreAppendPartitionByNameResultSend),
    AppendPartitionByNameWithEnvironmentContext(
        ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend,
    ),
    DropPartition(ThriftHiveMetastoreDropPartitionResultSend),
    DropPartitionWithEnvironmentContext(
        ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultSend,
    ),
    DropPartitionByName(ThriftHiveMetastoreDropPartitionByNameResultSend),
    DropPartitionByNameWithEnvironmentContext(
        ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultSend,
    ),
    DropPartitionsReq(ThriftHiveMetastoreDropPartitionsReqResultSend),
    GetPartition(ThriftHiveMetastoreGetPartitionResultSend),
    ExchangePartition(ThriftHiveMetastoreExchangePartitionResultSend),
    ExchangePartitions(ThriftHiveMetastoreExchangePartitionsResultSend),
    GetPartitionWithAuth(ThriftHiveMetastoreGetPartitionWithAuthResultSend),
    GetPartitionByName(ThriftHiveMetastoreGetPartitionByNameResultSend),
    GetPartitions(ThriftHiveMetastoreGetPartitionsResultSend),
    GetPartitionsWithAuth(ThriftHiveMetastoreGetPartitionsWithAuthResultSend),
    GetPartitionsPspec(ThriftHiveMetastoreGetPartitionsPspecResultSend),
    GetPartitionNames(ThriftHiveMetastoreGetPartitionNamesResultSend),
    GetPartitionValues(ThriftHiveMetastoreGetPartitionValuesResultSend),
    GetPartitionsPs(ThriftHiveMetastoreGetPartitionsPsResultSend),
    GetPartitionsPsWithAuth(ThriftHiveMetastoreGetPartitionsPsWithAuthResultSend),
    GetPartitionNamesPs(ThriftHiveMetastoreGetPartitionNamesPsResultSend),
    GetPartitionsByFilter(ThriftHiveMetastoreGetPartitionsByFilterResultSend),
    GetPartSpecsByFilter(ThriftHiveMetastoreGetPartSpecsByFilterResultSend),
    GetPartitionsByExpr(ThriftHiveMetastoreGetPartitionsByExprResultSend),
    GetNumPartitionsByFilter(ThriftHiveMetastoreGetNumPartitionsByFilterResultSend),
    GetPartitionsByNames(ThriftHiveMetastoreGetPartitionsByNamesResultSend),
    AlterPartition(ThriftHiveMetastoreAlterPartitionResultSend),
    AlterPartitions(ThriftHiveMetastoreAlterPartitionsResultSend),
    AlterPartitionsWithEnvironmentContext(
        ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultSend,
    ),
    AlterPartitionWithEnvironmentContext(
        ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultSend,
    ),
    RenamePartition(ThriftHiveMetastoreRenamePartitionResultSend),
    PartitionNameHasValidCharacters(ThriftHiveMetastorePartitionNameHasValidCharactersResultSend),
    GetConfigValue(ThriftHiveMetastoreGetConfigValueResultSend),
    PartitionNameToVals(ThriftHiveMetastorePartitionNameToValsResultSend),
    PartitionNameToSpec(ThriftHiveMetastorePartitionNameToSpecResultSend),
    MarkPartitionForEvent(ThriftHiveMetastoreMarkPartitionForEventResultSend),
    IsPartitionMarkedForEvent(ThriftHiveMetastoreIsPartitionMarkedForEventResultSend),
    AddIndex(ThriftHiveMetastoreAddIndexResultSend),
    AlterIndex(ThriftHiveMetastoreAlterIndexResultSend),
    DropIndexByName(ThriftHiveMetastoreDropIndexByNameResultSend),
    GetIndexByName(ThriftHiveMetastoreGetIndexByNameResultSend),
    GetIndexes(ThriftHiveMetastoreGetIndexesResultSend),
    GetIndexNames(ThriftHiveMetastoreGetIndexNamesResultSend),
    GetPrimaryKeys(ThriftHiveMetastoreGetPrimaryKeysResultSend),
    GetForeignKeys(ThriftHiveMetastoreGetForeignKeysResultSend),
    UpdateTableColumnStatistics(ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend),
    UpdatePartitionColumnStatistics(ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend),
    GetTableColumnStatistics(ThriftHiveMetastoreGetTableColumnStatisticsResultSend),
    GetPartitionColumnStatistics(ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend),
    GetTableStatisticsReq(ThriftHiveMetastoreGetTableStatisticsReqResultSend),
    GetPartitionsStatisticsReq(ThriftHiveMetastoreGetPartitionsStatisticsReqResultSend),
    GetAggrStatsFor(ThriftHiveMetastoreGetAggrStatsForResultSend),
    SetAggrStatsFor(ThriftHiveMetastoreSetAggrStatsForResultSend),
    DeletePartitionColumnStatistics(ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend),
    DeleteTableColumnStatistics(ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend),
    CreateFunction(ThriftHiveMetastoreCreateFunctionResultSend),
    DropFunction(ThriftHiveMetastoreDropFunctionResultSend),
    AlterFunction(ThriftHiveMetastoreAlterFunctionResultSend),
    GetFunctions(ThriftHiveMetastoreGetFunctionsResultSend),
    GetFunction(ThriftHiveMetastoreGetFunctionResultSend),
    GetAllFunctions(ThriftHiveMetastoreGetAllFunctionsResultSend),
    CreateRole(ThriftHiveMetastoreCreateRoleResultSend),
    DropRole(ThriftHiveMetastoreDropRoleResultSend),
    GetRoleNames(ThriftHiveMetastoreGetRoleNamesResultSend),
    GrantRole(ThriftHiveMetastoreGrantRoleResultSend),
    RevokeRole(ThriftHiveMetastoreRevokeRoleResultSend),
    ListRoles(ThriftHiveMetastoreListRolesResultSend),
    GrantRevokeRole(ThriftHiveMetastoreGrantRevokeRoleResultSend),
    GetPrincipalsInRole(ThriftHiveMetastoreGetPrincipalsInRoleResultSend),
    GetRoleGrantsForPrincipal(ThriftHiveMetastoreGetRoleGrantsForPrincipalResultSend),
    GetPrivilegeSet(ThriftHiveMetastoreGetPrivilegeSetResultSend),
    ListPrivileges(ThriftHiveMetastoreListPrivilegesResultSend),
    GrantPrivileges(ThriftHiveMetastoreGrantPrivilegesResultSend),
    RevokePrivileges(ThriftHiveMetastoreRevokePrivilegesResultSend),
    GrantRevokePrivileges(ThriftHiveMetastoreGrantRevokePrivilegesResultSend),
    SetUgi(ThriftHiveMetastoreSetUgiResultSend),
    GetDelegationToken(ThriftHiveMetastoreGetDelegationTokenResultSend),
    RenewDelegationToken(ThriftHiveMetastoreRenewDelegationTokenResultSend),
    CancelDelegationToken(ThriftHiveMetastoreCancelDelegationTokenResultSend),
    AddToken(ThriftHiveMetastoreAddTokenResultSend),
    RemoveToken(ThriftHiveMetastoreRemoveTokenResultSend),
    GetToken(ThriftHiveMetastoreGetTokenResultSend),
    GetAllTokenIdentifiers(ThriftHiveMetastoreGetAllTokenIdentifiersResultSend),
    AddMasterKey(ThriftHiveMetastoreAddMasterKeyResultSend),
    UpdateMasterKey(ThriftHiveMetastoreUpdateMasterKeyResultSend),
    RemoveMasterKey(ThriftHiveMetastoreRemoveMasterKeyResultSend),
    GetMasterKeys(ThriftHiveMetastoreGetMasterKeysResultSend),
    GetOpenTxns(ThriftHiveMetastoreGetOpenTxnsResultSend),
    GetOpenTxnsInfo(ThriftHiveMetastoreGetOpenTxnsInfoResultSend),
    OpenTxns(ThriftHiveMetastoreOpenTxnsResultSend),
    AbortTxn(ThriftHiveMetastoreAbortTxnResultSend),
    AbortTxns(ThriftHiveMetastoreAbortTxnsResultSend),
    CommitTxn(ThriftHiveMetastoreCommitTxnResultSend),
    Lock(ThriftHiveMetastoreLockResultSend),
    CheckLock(ThriftHiveMetastoreCheckLockResultSend),
    Unlock(ThriftHiveMetastoreUnlockResultSend),
    ShowLocks(ThriftHiveMetastoreShowLocksResultSend),
    Heartbeat(ThriftHiveMetastoreHeartbeatResultSend),
    HeartbeatTxnRange(ThriftHiveMetastoreHeartbeatTxnRangeResultSend),
    Compact(ThriftHiveMetastoreCompactResultSend),
    Compact2(ThriftHiveMetastoreCompact2ResultSend),
    ShowCompact(ThriftHiveMetastoreShowCompactResultSend),
    AddDynamicPartitions(ThriftHiveMetastoreAddDynamicPartitionsResultSend),
    GetNextNotification(ThriftHiveMetastoreGetNextNotificationResultSend),
    GetCurrentNotificationEventId(ThriftHiveMetastoreGetCurrentNotificationEventIdResultSend),
    FireListenerEvent(ThriftHiveMetastoreFireListenerEventResultSend),
    FlushCache(ThriftHiveMetastoreFlushCacheResultSend),
    GetFileMetadataByExpr(ThriftHiveMetastoreGetFileMetadataByExprResultSend),
    GetFileMetadata(ThriftHiveMetastoreGetFileMetadataResultSend),
    PutFileMetadata(ThriftHiveMetastorePutFileMetadataResultSend),
    ClearFileMetadata(ThriftHiveMetastoreClearFileMetadataResultSend),
    CacheFileMetadata(ThriftHiveMetastoreCacheFileMetadataResultSend),
}

impl ::volo_thrift::EntryMessage for ThriftHiveMetastoreResponseSend {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::core::result::Result<(), ::pilota::thrift::ThriftException> {
        match self {
            Self::GetMetaConf(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::SetMetaConf(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateDatabase(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetDatabase(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropDatabase(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetDatabases(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetAllDatabases(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterDatabase(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetType(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateType(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropType(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTypeAll(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFields(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFieldsWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetSchema(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetSchemaWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateTable(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateTableWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateTableWithConstraints(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropConstraint(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPrimaryKey(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddForeignKey(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropTable(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropTableWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTables(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTablesByType(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableMeta(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetAllTables(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTable(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableObjectsByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableObjectsByNameReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableNamesByFilter(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterTable(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterTableWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterTableWithCascade(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPartitionWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPartitions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPartitionsPspec(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AppendPartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddPartitionsReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AppendPartitionWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AppendPartitionByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AppendPartitionByNameWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropPartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropPartitionWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropPartitionByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropPartitionByNameWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropPartitionsReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ExchangePartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ExchangePartitions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionWithAuth(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsWithAuth(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsPspec(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionNames(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionValues(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsPs(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsPsWithAuth(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionNamesPs(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsByFilter(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartSpecsByFilter(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsByExpr(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetNumPartitionsByFilter(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsByNames(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterPartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterPartitions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterPartitionsWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterPartitionWithEnvironmentContext(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RenamePartition(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::PartitionNameHasValidCharacters(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetConfigValue(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::PartitionNameToVals(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::PartitionNameToSpec(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::MarkPartitionForEvent(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::IsPartitionMarkedForEvent(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddIndex(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterIndex(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropIndexByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetIndexByName(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetIndexes(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetIndexNames(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPrimaryKeys(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetForeignKeys(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::UpdateTableColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::UpdatePartitionColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetTableStatisticsReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPartitionsStatisticsReq(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetAggrStatsFor(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::SetAggrStatsFor(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DeletePartitionColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DeleteTableColumnStatistics(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateFunction(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropFunction(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AlterFunction(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFunctions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFunction(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetAllFunctions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CreateRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::DropRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetRoleNames(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GrantRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RevokeRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ListRoles(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GrantRevokeRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPrincipalsInRole(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetRoleGrantsForPrincipal(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetPrivilegeSet(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ListPrivileges(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GrantPrivileges(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RevokePrivileges(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GrantRevokePrivileges(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::SetUgi(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetDelegationToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RenewDelegationToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CancelDelegationToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RemoveToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetToken(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetAllTokenIdentifiers(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddMasterKey(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::UpdateMasterKey(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::RemoveMasterKey(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetMasterKeys(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetOpenTxns(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetOpenTxnsInfo(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::OpenTxns(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AbortTxn(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AbortTxns(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CommitTxn(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::Lock(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CheckLock(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::Unlock(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ShowLocks(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::Heartbeat(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::HeartbeatTxnRange(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::Compact(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::Compact2(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ShowCompact(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::AddDynamicPartitions(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetNextNotification(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetCurrentNotificationEventId(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::FireListenerEvent(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::FlushCache(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFileMetadataByExpr(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::GetFileMetadata(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::PutFileMetadata(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::ClearFileMetadata(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
            Self::CacheFileMetadata(value) => {
                ::pilota::thrift::Message::encode(value, __protocol).map_err(|err| err.into())
            }
        }
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        __protocol: &mut T,
        msg_ident: &::pilota::thrift::TMessageIdentifier,
    ) -> ::core::result::Result<Self, ::pilota::thrift::ThriftException> {
        ::std::result::Result::Ok(match &*msg_ident.name {
            "getMetaConf" => Self::GetMetaConf(::pilota::thrift::Message::decode(__protocol)?),
            "setMetaConf" => Self::SetMetaConf(::pilota::thrift::Message::decode(__protocol)?),
            "create_database" => {
                Self::CreateDatabase(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_database" => Self::GetDatabase(::pilota::thrift::Message::decode(__protocol)?),
            "drop_database" => Self::DropDatabase(::pilota::thrift::Message::decode(__protocol)?),
            "get_databases" => Self::GetDatabases(::pilota::thrift::Message::decode(__protocol)?),
            "get_all_databases" => {
                Self::GetAllDatabases(::pilota::thrift::Message::decode(__protocol)?)
            }
            "alter_database" => Self::AlterDatabase(::pilota::thrift::Message::decode(__protocol)?),
            "get_type" => Self::GetType(::pilota::thrift::Message::decode(__protocol)?),
            "create_type" => Self::CreateType(::pilota::thrift::Message::decode(__protocol)?),
            "drop_type" => Self::DropType(::pilota::thrift::Message::decode(__protocol)?),
            "get_type_all" => Self::GetTypeAll(::pilota::thrift::Message::decode(__protocol)?),
            "get_fields" => Self::GetFields(::pilota::thrift::Message::decode(__protocol)?),
            "get_fields_with_environment_context" => Self::GetFieldsWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "get_schema" => Self::GetSchema(::pilota::thrift::Message::decode(__protocol)?),
            "get_schema_with_environment_context" => Self::GetSchemaWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "create_table" => Self::CreateTable(::pilota::thrift::Message::decode(__protocol)?),
            "create_table_with_environment_context" => Self::CreateTableWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "create_table_with_constraints" => {
                Self::CreateTableWithConstraints(::pilota::thrift::Message::decode(__protocol)?)
            }
            "drop_constraint" => {
                Self::DropConstraint(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_primary_key" => {
                Self::AddPrimaryKey(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_foreign_key" => {
                Self::AddForeignKey(::pilota::thrift::Message::decode(__protocol)?)
            }
            "drop_table" => Self::DropTable(::pilota::thrift::Message::decode(__protocol)?),
            "drop_table_with_environment_context" => Self::DropTableWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "get_tables" => Self::GetTables(::pilota::thrift::Message::decode(__protocol)?),
            "get_tables_by_type" => {
                Self::GetTablesByType(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_table_meta" => Self::GetTableMeta(::pilota::thrift::Message::decode(__protocol)?),
            "get_all_tables" => Self::GetAllTables(::pilota::thrift::Message::decode(__protocol)?),
            "get_table" => Self::GetTable(::pilota::thrift::Message::decode(__protocol)?),
            "get_table_objects_by_name" => {
                Self::GetTableObjectsByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_table_req" => Self::GetTableReq(::pilota::thrift::Message::decode(__protocol)?),
            "get_table_objects_by_name_req" => {
                Self::GetTableObjectsByNameReq(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_table_names_by_filter" => {
                Self::GetTableNamesByFilter(::pilota::thrift::Message::decode(__protocol)?)
            }
            "alter_table" => Self::AlterTable(::pilota::thrift::Message::decode(__protocol)?),
            "alter_table_with_environment_context" => Self::AlterTableWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "alter_table_with_cascade" => {
                Self::AlterTableWithCascade(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_partition" => Self::AddPartition(::pilota::thrift::Message::decode(__protocol)?),
            "add_partition_with_environment_context" => Self::AddPartitionWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "add_partitions" => Self::AddPartitions(::pilota::thrift::Message::decode(__protocol)?),
            "add_partitions_pspec" => {
                Self::AddPartitionsPspec(::pilota::thrift::Message::decode(__protocol)?)
            }
            "append_partition" => {
                Self::AppendPartition(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_partitions_req" => {
                Self::AddPartitionsReq(::pilota::thrift::Message::decode(__protocol)?)
            }
            "append_partition_with_environment_context" => {
                Self::AppendPartitionWithEnvironmentContext(::pilota::thrift::Message::decode(
                    __protocol,
                )?)
            }
            "append_partition_by_name" => {
                Self::AppendPartitionByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "append_partition_by_name_with_environment_context" => {
                Self::AppendPartitionByNameWithEnvironmentContext(
                    ::pilota::thrift::Message::decode(__protocol)?,
                )
            }
            "drop_partition" => Self::DropPartition(::pilota::thrift::Message::decode(__protocol)?),
            "drop_partition_with_environment_context" => Self::DropPartitionWithEnvironmentContext(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "drop_partition_by_name" => {
                Self::DropPartitionByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "drop_partition_by_name_with_environment_context" => {
                Self::DropPartitionByNameWithEnvironmentContext(::pilota::thrift::Message::decode(
                    __protocol,
                )?)
            }
            "drop_partitions_req" => {
                Self::DropPartitionsReq(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition" => Self::GetPartition(::pilota::thrift::Message::decode(__protocol)?),
            "exchange_partition" => {
                Self::ExchangePartition(::pilota::thrift::Message::decode(__protocol)?)
            }
            "exchange_partitions" => {
                Self::ExchangePartitions(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_with_auth" => {
                Self::GetPartitionWithAuth(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_by_name" => {
                Self::GetPartitionByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions" => Self::GetPartitions(::pilota::thrift::Message::decode(__protocol)?),
            "get_partitions_with_auth" => {
                Self::GetPartitionsWithAuth(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_pspec" => {
                Self::GetPartitionsPspec(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_names" => {
                Self::GetPartitionNames(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_values" => {
                Self::GetPartitionValues(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_ps" => {
                Self::GetPartitionsPs(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_ps_with_auth" => {
                Self::GetPartitionsPsWithAuth(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_names_ps" => {
                Self::GetPartitionNamesPs(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_by_filter" => {
                Self::GetPartitionsByFilter(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_part_specs_by_filter" => {
                Self::GetPartSpecsByFilter(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_by_expr" => {
                Self::GetPartitionsByExpr(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_num_partitions_by_filter" => {
                Self::GetNumPartitionsByFilter(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_by_names" => {
                Self::GetPartitionsByNames(::pilota::thrift::Message::decode(__protocol)?)
            }
            "alter_partition" => {
                Self::AlterPartition(::pilota::thrift::Message::decode(__protocol)?)
            }
            "alter_partitions" => {
                Self::AlterPartitions(::pilota::thrift::Message::decode(__protocol)?)
            }
            "alter_partitions_with_environment_context" => {
                Self::AlterPartitionsWithEnvironmentContext(::pilota::thrift::Message::decode(
                    __protocol,
                )?)
            }
            "alter_partition_with_environment_context" => {
                Self::AlterPartitionWithEnvironmentContext(::pilota::thrift::Message::decode(
                    __protocol,
                )?)
            }
            "rename_partition" => {
                Self::RenamePartition(::pilota::thrift::Message::decode(__protocol)?)
            }
            "partition_name_has_valid_characters" => Self::PartitionNameHasValidCharacters(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "get_config_value" => {
                Self::GetConfigValue(::pilota::thrift::Message::decode(__protocol)?)
            }
            "partition_name_to_vals" => {
                Self::PartitionNameToVals(::pilota::thrift::Message::decode(__protocol)?)
            }
            "partition_name_to_spec" => {
                Self::PartitionNameToSpec(::pilota::thrift::Message::decode(__protocol)?)
            }
            "markPartitionForEvent" => {
                Self::MarkPartitionForEvent(::pilota::thrift::Message::decode(__protocol)?)
            }
            "isPartitionMarkedForEvent" => {
                Self::IsPartitionMarkedForEvent(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_index" => Self::AddIndex(::pilota::thrift::Message::decode(__protocol)?),
            "alter_index" => Self::AlterIndex(::pilota::thrift::Message::decode(__protocol)?),
            "drop_index_by_name" => {
                Self::DropIndexByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_index_by_name" => {
                Self::GetIndexByName(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_indexes" => Self::GetIndexes(::pilota::thrift::Message::decode(__protocol)?),
            "get_index_names" => {
                Self::GetIndexNames(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_primary_keys" => {
                Self::GetPrimaryKeys(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_foreign_keys" => {
                Self::GetForeignKeys(::pilota::thrift::Message::decode(__protocol)?)
            }
            "update_table_column_statistics" => {
                Self::UpdateTableColumnStatistics(::pilota::thrift::Message::decode(__protocol)?)
            }
            "update_partition_column_statistics" => Self::UpdatePartitionColumnStatistics(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "get_table_column_statistics" => {
                Self::GetTableColumnStatistics(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partition_column_statistics" => {
                Self::GetPartitionColumnStatistics(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_table_statistics_req" => {
                Self::GetTableStatisticsReq(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_partitions_statistics_req" => {
                Self::GetPartitionsStatisticsReq(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_aggr_stats_for" => {
                Self::GetAggrStatsFor(::pilota::thrift::Message::decode(__protocol)?)
            }
            "set_aggr_stats_for" => {
                Self::SetAggrStatsFor(::pilota::thrift::Message::decode(__protocol)?)
            }
            "delete_partition_column_statistics" => Self::DeletePartitionColumnStatistics(
                ::pilota::thrift::Message::decode(__protocol)?,
            ),
            "delete_table_column_statistics" => {
                Self::DeleteTableColumnStatistics(::pilota::thrift::Message::decode(__protocol)?)
            }
            "create_function" => {
                Self::CreateFunction(::pilota::thrift::Message::decode(__protocol)?)
            }
            "drop_function" => Self::DropFunction(::pilota::thrift::Message::decode(__protocol)?),
            "alter_function" => Self::AlterFunction(::pilota::thrift::Message::decode(__protocol)?),
            "get_functions" => Self::GetFunctions(::pilota::thrift::Message::decode(__protocol)?),
            "get_function" => Self::GetFunction(::pilota::thrift::Message::decode(__protocol)?),
            "get_all_functions" => {
                Self::GetAllFunctions(::pilota::thrift::Message::decode(__protocol)?)
            }
            "create_role" => Self::CreateRole(::pilota::thrift::Message::decode(__protocol)?),
            "drop_role" => Self::DropRole(::pilota::thrift::Message::decode(__protocol)?),
            "get_role_names" => Self::GetRoleNames(::pilota::thrift::Message::decode(__protocol)?),
            "grant_role" => Self::GrantRole(::pilota::thrift::Message::decode(__protocol)?),
            "revoke_role" => Self::RevokeRole(::pilota::thrift::Message::decode(__protocol)?),
            "list_roles" => Self::ListRoles(::pilota::thrift::Message::decode(__protocol)?),
            "grant_revoke_role" => {
                Self::GrantRevokeRole(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_principals_in_role" => {
                Self::GetPrincipalsInRole(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_role_grants_for_principal" => {
                Self::GetRoleGrantsForPrincipal(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_privilege_set" => {
                Self::GetPrivilegeSet(::pilota::thrift::Message::decode(__protocol)?)
            }
            "list_privileges" => {
                Self::ListPrivileges(::pilota::thrift::Message::decode(__protocol)?)
            }
            "grant_privileges" => {
                Self::GrantPrivileges(::pilota::thrift::Message::decode(__protocol)?)
            }
            "revoke_privileges" => {
                Self::RevokePrivileges(::pilota::thrift::Message::decode(__protocol)?)
            }
            "grant_revoke_privileges" => {
                Self::GrantRevokePrivileges(::pilota::thrift::Message::decode(__protocol)?)
            }
            "set_ugi" => Self::SetUgi(::pilota::thrift::Message::decode(__protocol)?),
            "get_delegation_token" => {
                Self::GetDelegationToken(::pilota::thrift::Message::decode(__protocol)?)
            }
            "renew_delegation_token" => {
                Self::RenewDelegationToken(::pilota::thrift::Message::decode(__protocol)?)
            }
            "cancel_delegation_token" => {
                Self::CancelDelegationToken(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_token" => Self::AddToken(::pilota::thrift::Message::decode(__protocol)?),
            "remove_token" => Self::RemoveToken(::pilota::thrift::Message::decode(__protocol)?),
            "get_token" => Self::GetToken(::pilota::thrift::Message::decode(__protocol)?),
            "get_all_token_identifiers" => {
                Self::GetAllTokenIdentifiers(::pilota::thrift::Message::decode(__protocol)?)
            }
            "add_master_key" => Self::AddMasterKey(::pilota::thrift::Message::decode(__protocol)?),
            "update_master_key" => {
                Self::UpdateMasterKey(::pilota::thrift::Message::decode(__protocol)?)
            }
            "remove_master_key" => {
                Self::RemoveMasterKey(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_master_keys" => {
                Self::GetMasterKeys(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_open_txns" => Self::GetOpenTxns(::pilota::thrift::Message::decode(__protocol)?),
            "get_open_txns_info" => {
                Self::GetOpenTxnsInfo(::pilota::thrift::Message::decode(__protocol)?)
            }
            "open_txns" => Self::OpenTxns(::pilota::thrift::Message::decode(__protocol)?),
            "abort_txn" => Self::AbortTxn(::pilota::thrift::Message::decode(__protocol)?),
            "abort_txns" => Self::AbortTxns(::pilota::thrift::Message::decode(__protocol)?),
            "commit_txn" => Self::CommitTxn(::pilota::thrift::Message::decode(__protocol)?),
            "lock" => Self::Lock(::pilota::thrift::Message::decode(__protocol)?),
            "check_lock" => Self::CheckLock(::pilota::thrift::Message::decode(__protocol)?),
            "unlock" => Self::Unlock(::pilota::thrift::Message::decode(__protocol)?),
            "show_locks" => Self::ShowLocks(::pilota::thrift::Message::decode(__protocol)?),
            "heartbeat" => Self::Heartbeat(::pilota::thrift::Message::decode(__protocol)?),
            "heartbeat_txn_range" => {
                Self::HeartbeatTxnRange(::pilota::thrift::Message::decode(__protocol)?)
            }
            "compact" => Self::Compact(::pilota::thrift::Message::decode(__protocol)?),
            "compact2" => Self::Compact2(::pilota::thrift::Message::decode(__protocol)?),
            "show_compact" => Self::ShowCompact(::pilota::thrift::Message::decode(__protocol)?),
            "add_dynamic_partitions" => {
                Self::AddDynamicPartitions(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_next_notification" => {
                Self::GetNextNotification(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_current_notificationEventId" => {
                Self::GetCurrentNotificationEventId(::pilota::thrift::Message::decode(__protocol)?)
            }
            "fire_listener_event" => {
                Self::FireListenerEvent(::pilota::thrift::Message::decode(__protocol)?)
            }
            "flushCache" => Self::FlushCache(::pilota::thrift::Message::decode(__protocol)?),
            "get_file_metadata_by_expr" => {
                Self::GetFileMetadataByExpr(::pilota::thrift::Message::decode(__protocol)?)
            }
            "get_file_metadata" => {
                Self::GetFileMetadata(::pilota::thrift::Message::decode(__protocol)?)
            }
            "put_file_metadata" => {
                Self::PutFileMetadata(::pilota::thrift::Message::decode(__protocol)?)
            }
            "clear_file_metadata" => {
                Self::ClearFileMetadata(::pilota::thrift::Message::decode(__protocol)?)
            }
            "cache_file_metadata" => {
                Self::CacheFileMetadata(::pilota::thrift::Message::decode(__protocol)?)
            }
            _ => {
                return ::std::result::Result::Err(::pilota::thrift::new_application_exception(
                    ::pilota::thrift::ApplicationExceptionKind::UNKNOWN_METHOD,
                    format!("unknown method {}", msg_ident.name),
                ));
            }
        })
    }

    async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
        __protocol: &mut T,
        msg_ident: &::pilota::thrift::TMessageIdentifier,
    ) -> ::core::result::Result<Self, ::pilota::thrift::ThriftException> {
        ::std::result::Result::Ok(match &*msg_ident.name {
                        "getMetaConf" => { Self::GetMetaConf(<ThriftHiveMetastoreGetMetaConfResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"setMetaConf" => { Self::SetMetaConf(<ThriftHiveMetastoreSetMetaConfResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_database" => { Self::CreateDatabase(<ThriftHiveMetastoreCreateDatabaseResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_database" => { Self::GetDatabase(<ThriftHiveMetastoreGetDatabaseResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_database" => { Self::DropDatabase(<ThriftHiveMetastoreDropDatabaseResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_databases" => { Self::GetDatabases(<ThriftHiveMetastoreGetDatabasesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_all_databases" => { Self::GetAllDatabases(<ThriftHiveMetastoreGetAllDatabasesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_database" => { Self::AlterDatabase(<ThriftHiveMetastoreAlterDatabaseResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_type" => { Self::GetType(<ThriftHiveMetastoreGetTypeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_type" => { Self::CreateType(<ThriftHiveMetastoreCreateTypeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_type" => { Self::DropType(<ThriftHiveMetastoreDropTypeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_type_all" => { Self::GetTypeAll(<ThriftHiveMetastoreGetTypeAllResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_fields" => { Self::GetFields(<ThriftHiveMetastoreGetFieldsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_fields_with_environment_context" => { Self::GetFieldsWithEnvironmentContext(<ThriftHiveMetastoreGetFieldsWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_schema" => { Self::GetSchema(<ThriftHiveMetastoreGetSchemaResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_schema_with_environment_context" => { Self::GetSchemaWithEnvironmentContext(<ThriftHiveMetastoreGetSchemaWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_table" => { Self::CreateTable(<ThriftHiveMetastoreCreateTableResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_table_with_environment_context" => { Self::CreateTableWithEnvironmentContext(<ThriftHiveMetastoreCreateTableWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_table_with_constraints" => { Self::CreateTableWithConstraints(<ThriftHiveMetastoreCreateTableWithConstraintsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_constraint" => { Self::DropConstraint(<ThriftHiveMetastoreDropConstraintResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_primary_key" => { Self::AddPrimaryKey(<ThriftHiveMetastoreAddPrimaryKeyResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_foreign_key" => { Self::AddForeignKey(<ThriftHiveMetastoreAddForeignKeyResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_table" => { Self::DropTable(<ThriftHiveMetastoreDropTableResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_table_with_environment_context" => { Self::DropTableWithEnvironmentContext(<ThriftHiveMetastoreDropTableWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_tables" => { Self::GetTables(<ThriftHiveMetastoreGetTablesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_tables_by_type" => { Self::GetTablesByType(<ThriftHiveMetastoreGetTablesByTypeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_meta" => { Self::GetTableMeta(<ThriftHiveMetastoreGetTableMetaResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_all_tables" => { Self::GetAllTables(<ThriftHiveMetastoreGetAllTablesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table" => { Self::GetTable(<ThriftHiveMetastoreGetTableResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_objects_by_name" => { Self::GetTableObjectsByName(<ThriftHiveMetastoreGetTableObjectsByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_req" => { Self::GetTableReq(<ThriftHiveMetastoreGetTableReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_objects_by_name_req" => { Self::GetTableObjectsByNameReq(<ThriftHiveMetastoreGetTableObjectsByNameReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_names_by_filter" => { Self::GetTableNamesByFilter(<ThriftHiveMetastoreGetTableNamesByFilterResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_table" => { Self::AlterTable(<ThriftHiveMetastoreAlterTableResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_table_with_environment_context" => { Self::AlterTableWithEnvironmentContext(<ThriftHiveMetastoreAlterTableWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_table_with_cascade" => { Self::AlterTableWithCascade(<ThriftHiveMetastoreAlterTableWithCascadeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_partition" => { Self::AddPartition(<ThriftHiveMetastoreAddPartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_partition_with_environment_context" => { Self::AddPartitionWithEnvironmentContext(<ThriftHiveMetastoreAddPartitionWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_partitions" => { Self::AddPartitions(<ThriftHiveMetastoreAddPartitionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_partitions_pspec" => { Self::AddPartitionsPspec(<ThriftHiveMetastoreAddPartitionsPspecResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"append_partition" => { Self::AppendPartition(<ThriftHiveMetastoreAppendPartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_partitions_req" => { Self::AddPartitionsReq(<ThriftHiveMetastoreAddPartitionsReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"append_partition_with_environment_context" => { Self::AppendPartitionWithEnvironmentContext(<ThriftHiveMetastoreAppendPartitionWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"append_partition_by_name" => { Self::AppendPartitionByName(<ThriftHiveMetastoreAppendPartitionByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"append_partition_by_name_with_environment_context" => { Self::AppendPartitionByNameWithEnvironmentContext(<ThriftHiveMetastoreAppendPartitionByNameWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_partition" => { Self::DropPartition(<ThriftHiveMetastoreDropPartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_partition_with_environment_context" => { Self::DropPartitionWithEnvironmentContext(<ThriftHiveMetastoreDropPartitionWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_partition_by_name" => { Self::DropPartitionByName(<ThriftHiveMetastoreDropPartitionByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_partition_by_name_with_environment_context" => { Self::DropPartitionByNameWithEnvironmentContext(<ThriftHiveMetastoreDropPartitionByNameWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_partitions_req" => { Self::DropPartitionsReq(<ThriftHiveMetastoreDropPartitionsReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition" => { Self::GetPartition(<ThriftHiveMetastoreGetPartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"exchange_partition" => { Self::ExchangePartition(<ThriftHiveMetastoreExchangePartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"exchange_partitions" => { Self::ExchangePartitions(<ThriftHiveMetastoreExchangePartitionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_with_auth" => { Self::GetPartitionWithAuth(<ThriftHiveMetastoreGetPartitionWithAuthResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_by_name" => { Self::GetPartitionByName(<ThriftHiveMetastoreGetPartitionByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions" => { Self::GetPartitions(<ThriftHiveMetastoreGetPartitionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_with_auth" => { Self::GetPartitionsWithAuth(<ThriftHiveMetastoreGetPartitionsWithAuthResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_pspec" => { Self::GetPartitionsPspec(<ThriftHiveMetastoreGetPartitionsPspecResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_names" => { Self::GetPartitionNames(<ThriftHiveMetastoreGetPartitionNamesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_values" => { Self::GetPartitionValues(<ThriftHiveMetastoreGetPartitionValuesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_ps" => { Self::GetPartitionsPs(<ThriftHiveMetastoreGetPartitionsPsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_ps_with_auth" => { Self::GetPartitionsPsWithAuth(<ThriftHiveMetastoreGetPartitionsPsWithAuthResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_names_ps" => { Self::GetPartitionNamesPs(<ThriftHiveMetastoreGetPartitionNamesPsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_by_filter" => { Self::GetPartitionsByFilter(<ThriftHiveMetastoreGetPartitionsByFilterResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_part_specs_by_filter" => { Self::GetPartSpecsByFilter(<ThriftHiveMetastoreGetPartSpecsByFilterResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_by_expr" => { Self::GetPartitionsByExpr(<ThriftHiveMetastoreGetPartitionsByExprResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_num_partitions_by_filter" => { Self::GetNumPartitionsByFilter(<ThriftHiveMetastoreGetNumPartitionsByFilterResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_by_names" => { Self::GetPartitionsByNames(<ThriftHiveMetastoreGetPartitionsByNamesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_partition" => { Self::AlterPartition(<ThriftHiveMetastoreAlterPartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_partitions" => { Self::AlterPartitions(<ThriftHiveMetastoreAlterPartitionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_partitions_with_environment_context" => { Self::AlterPartitionsWithEnvironmentContext(<ThriftHiveMetastoreAlterPartitionsWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_partition_with_environment_context" => { Self::AlterPartitionWithEnvironmentContext(<ThriftHiveMetastoreAlterPartitionWithEnvironmentContextResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"rename_partition" => { Self::RenamePartition(<ThriftHiveMetastoreRenamePartitionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"partition_name_has_valid_characters" => { Self::PartitionNameHasValidCharacters(<ThriftHiveMetastorePartitionNameHasValidCharactersResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_config_value" => { Self::GetConfigValue(<ThriftHiveMetastoreGetConfigValueResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"partition_name_to_vals" => { Self::PartitionNameToVals(<ThriftHiveMetastorePartitionNameToValsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"partition_name_to_spec" => { Self::PartitionNameToSpec(<ThriftHiveMetastorePartitionNameToSpecResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"markPartitionForEvent" => { Self::MarkPartitionForEvent(<ThriftHiveMetastoreMarkPartitionForEventResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"isPartitionMarkedForEvent" => { Self::IsPartitionMarkedForEvent(<ThriftHiveMetastoreIsPartitionMarkedForEventResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_index" => { Self::AddIndex(<ThriftHiveMetastoreAddIndexResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_index" => { Self::AlterIndex(<ThriftHiveMetastoreAlterIndexResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_index_by_name" => { Self::DropIndexByName(<ThriftHiveMetastoreDropIndexByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_index_by_name" => { Self::GetIndexByName(<ThriftHiveMetastoreGetIndexByNameResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_indexes" => { Self::GetIndexes(<ThriftHiveMetastoreGetIndexesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_index_names" => { Self::GetIndexNames(<ThriftHiveMetastoreGetIndexNamesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_primary_keys" => { Self::GetPrimaryKeys(<ThriftHiveMetastoreGetPrimaryKeysResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_foreign_keys" => { Self::GetForeignKeys(<ThriftHiveMetastoreGetForeignKeysResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"update_table_column_statistics" => { Self::UpdateTableColumnStatistics(<ThriftHiveMetastoreUpdateTableColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"update_partition_column_statistics" => { Self::UpdatePartitionColumnStatistics(<ThriftHiveMetastoreUpdatePartitionColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_column_statistics" => { Self::GetTableColumnStatistics(<ThriftHiveMetastoreGetTableColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partition_column_statistics" => { Self::GetPartitionColumnStatistics(<ThriftHiveMetastoreGetPartitionColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_table_statistics_req" => { Self::GetTableStatisticsReq(<ThriftHiveMetastoreGetTableStatisticsReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_partitions_statistics_req" => { Self::GetPartitionsStatisticsReq(<ThriftHiveMetastoreGetPartitionsStatisticsReqResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_aggr_stats_for" => { Self::GetAggrStatsFor(<ThriftHiveMetastoreGetAggrStatsForResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"set_aggr_stats_for" => { Self::SetAggrStatsFor(<ThriftHiveMetastoreSetAggrStatsForResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"delete_partition_column_statistics" => { Self::DeletePartitionColumnStatistics(<ThriftHiveMetastoreDeletePartitionColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"delete_table_column_statistics" => { Self::DeleteTableColumnStatistics(<ThriftHiveMetastoreDeleteTableColumnStatisticsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_function" => { Self::CreateFunction(<ThriftHiveMetastoreCreateFunctionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_function" => { Self::DropFunction(<ThriftHiveMetastoreDropFunctionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"alter_function" => { Self::AlterFunction(<ThriftHiveMetastoreAlterFunctionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_functions" => { Self::GetFunctions(<ThriftHiveMetastoreGetFunctionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_function" => { Self::GetFunction(<ThriftHiveMetastoreGetFunctionResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_all_functions" => { Self::GetAllFunctions(<ThriftHiveMetastoreGetAllFunctionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"create_role" => { Self::CreateRole(<ThriftHiveMetastoreCreateRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"drop_role" => { Self::DropRole(<ThriftHiveMetastoreDropRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_role_names" => { Self::GetRoleNames(<ThriftHiveMetastoreGetRoleNamesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"grant_role" => { Self::GrantRole(<ThriftHiveMetastoreGrantRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"revoke_role" => { Self::RevokeRole(<ThriftHiveMetastoreRevokeRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"list_roles" => { Self::ListRoles(<ThriftHiveMetastoreListRolesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"grant_revoke_role" => { Self::GrantRevokeRole(<ThriftHiveMetastoreGrantRevokeRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_principals_in_role" => { Self::GetPrincipalsInRole(<ThriftHiveMetastoreGetPrincipalsInRoleResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_role_grants_for_principal" => { Self::GetRoleGrantsForPrincipal(<ThriftHiveMetastoreGetRoleGrantsForPrincipalResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_privilege_set" => { Self::GetPrivilegeSet(<ThriftHiveMetastoreGetPrivilegeSetResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"list_privileges" => { Self::ListPrivileges(<ThriftHiveMetastoreListPrivilegesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"grant_privileges" => { Self::GrantPrivileges(<ThriftHiveMetastoreGrantPrivilegesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"revoke_privileges" => { Self::RevokePrivileges(<ThriftHiveMetastoreRevokePrivilegesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"grant_revoke_privileges" => { Self::GrantRevokePrivileges(<ThriftHiveMetastoreGrantRevokePrivilegesResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"set_ugi" => { Self::SetUgi(<ThriftHiveMetastoreSetUgiResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_delegation_token" => { Self::GetDelegationToken(<ThriftHiveMetastoreGetDelegationTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"renew_delegation_token" => { Self::RenewDelegationToken(<ThriftHiveMetastoreRenewDelegationTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"cancel_delegation_token" => { Self::CancelDelegationToken(<ThriftHiveMetastoreCancelDelegationTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_token" => { Self::AddToken(<ThriftHiveMetastoreAddTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"remove_token" => { Self::RemoveToken(<ThriftHiveMetastoreRemoveTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_token" => { Self::GetToken(<ThriftHiveMetastoreGetTokenResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_all_token_identifiers" => { Self::GetAllTokenIdentifiers(<ThriftHiveMetastoreGetAllTokenIdentifiersResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_master_key" => { Self::AddMasterKey(<ThriftHiveMetastoreAddMasterKeyResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"update_master_key" => { Self::UpdateMasterKey(<ThriftHiveMetastoreUpdateMasterKeyResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"remove_master_key" => { Self::RemoveMasterKey(<ThriftHiveMetastoreRemoveMasterKeyResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_master_keys" => { Self::GetMasterKeys(<ThriftHiveMetastoreGetMasterKeysResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_open_txns" => { Self::GetOpenTxns(<ThriftHiveMetastoreGetOpenTxnsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_open_txns_info" => { Self::GetOpenTxnsInfo(<ThriftHiveMetastoreGetOpenTxnsInfoResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"open_txns" => { Self::OpenTxns(<ThriftHiveMetastoreOpenTxnsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"abort_txn" => { Self::AbortTxn(<ThriftHiveMetastoreAbortTxnResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"abort_txns" => { Self::AbortTxns(<ThriftHiveMetastoreAbortTxnsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"commit_txn" => { Self::CommitTxn(<ThriftHiveMetastoreCommitTxnResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"lock" => { Self::Lock(<ThriftHiveMetastoreLockResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"check_lock" => { Self::CheckLock(<ThriftHiveMetastoreCheckLockResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"unlock" => { Self::Unlock(<ThriftHiveMetastoreUnlockResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"show_locks" => { Self::ShowLocks(<ThriftHiveMetastoreShowLocksResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"heartbeat" => { Self::Heartbeat(<ThriftHiveMetastoreHeartbeatResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"heartbeat_txn_range" => { Self::HeartbeatTxnRange(<ThriftHiveMetastoreHeartbeatTxnRangeResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"compact" => { Self::Compact(<ThriftHiveMetastoreCompactResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"compact2" => { Self::Compact2(<ThriftHiveMetastoreCompact2ResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"show_compact" => { Self::ShowCompact(<ThriftHiveMetastoreShowCompactResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"add_dynamic_partitions" => { Self::AddDynamicPartitions(<ThriftHiveMetastoreAddDynamicPartitionsResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_next_notification" => { Self::GetNextNotification(<ThriftHiveMetastoreGetNextNotificationResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_current_notificationEventId" => { Self::GetCurrentNotificationEventId(<ThriftHiveMetastoreGetCurrentNotificationEventIdResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"fire_listener_event" => { Self::FireListenerEvent(<ThriftHiveMetastoreFireListenerEventResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"flushCache" => { Self::FlushCache(<ThriftHiveMetastoreFlushCacheResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_file_metadata_by_expr" => { Self::GetFileMetadataByExpr(<ThriftHiveMetastoreGetFileMetadataByExprResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"get_file_metadata" => { Self::GetFileMetadata(<ThriftHiveMetastoreGetFileMetadataResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"put_file_metadata" => { Self::PutFileMetadata(<ThriftHiveMetastorePutFileMetadataResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"clear_file_metadata" => { Self::ClearFileMetadata(<ThriftHiveMetastoreClearFileMetadataResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },"cache_file_metadata" => { Self::CacheFileMetadata(<ThriftHiveMetastoreCacheFileMetadataResultSend as ::pilota::thrift::Message>::decode_async(__protocol).await?) },
                        _ => {
                            return ::std::result::Result::Err(::pilota::thrift::new_application_exception(::pilota::thrift::ApplicationExceptionKind::UNKNOWN_METHOD,  format!("unknown method {}", msg_ident.name)));
                        },
                    })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        match self {
            Self::GetMetaConf(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::SetMetaConf(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CreateDatabase(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetDatabase(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropDatabase(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetDatabases(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetAllDatabases(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterDatabase(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetType(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CreateType(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropType(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTypeAll(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFields(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFieldsWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetSchema(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetSchemaWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::CreateTable(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CreateTableWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::CreateTableWithConstraints(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::DropConstraint(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddPrimaryKey(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddForeignKey(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropTable(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropTableWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetTables(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTablesByType(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTableMeta(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetAllTables(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTable(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTableObjectsByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTableReq(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetTableObjectsByNameReq(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetTableNamesByFilter(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterTable(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterTableWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::AlterTableWithCascade(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddPartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddPartitionWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::AddPartitions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddPartitionsPspec(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AppendPartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddPartitionsReq(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AppendPartitionWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::AppendPartitionByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AppendPartitionByNameWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::DropPartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropPartitionWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::DropPartitionByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropPartitionByNameWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::DropPartitionsReq(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ExchangePartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ExchangePartitions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionWithAuth(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsWithAuth(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsPspec(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionNames(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionValues(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsPs(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsPsWithAuth(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionNamesPs(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsByFilter(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartSpecsByFilter(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsByExpr(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetNumPartitionsByFilter(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetPartitionsByNames(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterPartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterPartitions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterPartitionsWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::AlterPartitionWithEnvironmentContext(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::RenamePartition(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::PartitionNameHasValidCharacters(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetConfigValue(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::PartitionNameToVals(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::PartitionNameToSpec(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::MarkPartitionForEvent(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::IsPartitionMarkedForEvent(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::AddIndex(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterIndex(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropIndexByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetIndexByName(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetIndexes(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetIndexNames(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPrimaryKeys(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetForeignKeys(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::UpdateTableColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::UpdatePartitionColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetTableColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetPartitionColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetTableStatisticsReq(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPartitionsStatisticsReq(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetAggrStatsFor(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::SetAggrStatsFor(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DeletePartitionColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::DeleteTableColumnStatistics(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::CreateFunction(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropFunction(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AlterFunction(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFunctions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFunction(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetAllFunctions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CreateRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::DropRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetRoleNames(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GrantRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::RevokeRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ListRoles(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GrantRevokeRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetPrincipalsInRole(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetRoleGrantsForPrincipal(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::GetPrivilegeSet(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ListPrivileges(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GrantPrivileges(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::RevokePrivileges(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GrantRevokePrivileges(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::SetUgi(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetDelegationToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::RenewDelegationToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CancelDelegationToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::RemoveToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetToken(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetAllTokenIdentifiers(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddMasterKey(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::UpdateMasterKey(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::RemoveMasterKey(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetMasterKeys(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetOpenTxns(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetOpenTxnsInfo(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::OpenTxns(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AbortTxn(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AbortTxns(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CommitTxn(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::Lock(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CheckLock(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::Unlock(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ShowLocks(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::Heartbeat(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::HeartbeatTxnRange(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::Compact(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::Compact2(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ShowCompact(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::AddDynamicPartitions(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetNextNotification(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetCurrentNotificationEventId(value) => {
                ::volo_thrift::Message::size(value, __protocol)
            }
            Self::FireListenerEvent(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::FlushCache(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFileMetadataByExpr(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::GetFileMetadata(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::PutFileMetadata(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::ClearFileMetadata(value) => ::volo_thrift::Message::size(value, __protocol),
            Self::CacheFileMetadata(value) => ::volo_thrift::Message::size(value, __protocol),
        }
    }
}
