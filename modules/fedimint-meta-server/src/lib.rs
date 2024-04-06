pub mod db;

use std::collections::BTreeMap;
use std::future;

use async_trait::async_trait;
use db::{
    MetaConsensusKey, MetaDesiredKey, MetaDesiredValue, MetaSubmissionsByKeyPrefix,
    MetaSubmissionsKey,
};
use fedimint_core::config::{
    ConfigGenModuleParams, DkgResult, ServerModuleConfig, ServerModuleConsensusConfig,
    TypedServerModuleConfig, TypedServerModuleConsensusConfig,
};
use fedimint_core::core::ModuleInstanceId;
use fedimint_core::db::{
    Committable, DatabaseTransaction, DatabaseVersion, IDatabaseTransactionOpsCoreTyped,
    NonCommittable, ServerMigrationFn,
};
use fedimint_core::module::audit::Audit;
use fedimint_core::module::{
    api_endpoint, ApiAuth, ApiEndpoint, ApiError, ApiVersion, CoreConsensusVersion, InputMeta,
    ModuleConsensusVersion, ModuleInit, PeerHandle, ServerModuleInit, ServerModuleInitArgs,
    SupportedModuleApiVersions, TransactionItemAmount,
};
use fedimint_core::server::DynServerModule;
use fedimint_core::{
    push_db_pair_items, NumPeers, NumPeersExt as _, OutPoint, PeerId, ServerModule,
};
use fedimint_logging::LOG_MODULE_META;
use fedimint_meta_common::config::{
    MetaClientConfig, MetaConfig, MetaConfigConsensus, MetaConfigLocal, MetaConfigPrivate,
};
pub use fedimint_meta_common::config::{MetaGenParams, MetaGenParamsConsensus, MetaGenParamsLocal};
use fedimint_meta_common::endpoint::{
    GetConsensusRequest, GetSubmissionResponse, GetSubmissionsRequest, SubmitRequest,
    GET_CONSENSUS_ENDPOINT, GET_CONSENSUS_REV_ENDPOINT, GET_SUBMISSIONS_ENDPOINT, SUBMIT_ENDPOINT,
};
use fedimint_meta_common::{
    MetaCommonInit, MetaConsensusItem, MetaConsensusValue, MetaInput, MetaInputError, MetaKey,
    MetaModuleTypes, MetaOutput, MetaOutputError, MetaOutputOutcome, MetaValue, CONSENSUS_VERSION,
};
use futures::StreamExt;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use tracing::{debug, info};

use crate::db::{
    DbKeyPrefix, MetaConsensusKeyPrefix, MetaDesiredKeyPrefix, MetaSubmissionValue,
    MetaSubmissionsKeyPrefix,
};

/// Generates the module
#[derive(Debug, Clone)]
pub struct MetaInit;

// TODO: Boilerplate-code
impl ModuleInit for MetaInit {
    type Common = MetaCommonInit;
    const DATABASE_VERSION: DatabaseVersion = DatabaseVersion(0);

    /// Dumps all database items for debugging
    async fn dump_database(
        &self,
        dbtx: &mut DatabaseTransaction<'_>,
        prefix_names: Vec<String>,
    ) -> Box<dyn Iterator<Item = (String, Box<dyn erased_serde::Serialize + Send>)> + '_> {
        // TODO: Boilerplate-code
        let mut items: BTreeMap<String, Box<dyn erased_serde::Serialize + Send>> = BTreeMap::new();
        let filtered_prefixes = DbKeyPrefix::iter().filter(|f| {
            prefix_names.is_empty() || prefix_names.contains(&f.to_string().to_lowercase())
        });

        for table in filtered_prefixes {
            match table {
                DbKeyPrefix::Desired => {
                    push_db_pair_items!(
                        dbtx,
                        MetaDesiredKeyPrefix,
                        MetaDesiredKey,
                        MetaDesiredValue,
                        items,
                        "Meta Desired"
                    );
                }
                DbKeyPrefix::Consensus => {
                    push_db_pair_items!(
                        dbtx,
                        MetaConsensusKeyPrefix,
                        MetaConsensusKey,
                        MetaConsensusValue,
                        items,
                        "Meta Consensus"
                    );
                }
                DbKeyPrefix::Submissions => {
                    push_db_pair_items!(
                        dbtx,
                        MetaSubmissionsKeyPrefix,
                        MetaSubmissionsKey,
                        MetaSubmissionValue,
                        items,
                        "Meta Submissions"
                    );
                }
            }
        }

        Box::new(items.into_iter())
    }
}

/// Implementation of server module non-consensus functions
#[async_trait]
impl ServerModuleInit for MetaInit {
    type Params = MetaGenParams;

    /// Returns the version of this module
    fn versions(&self, _core: CoreConsensusVersion) -> &[ModuleConsensusVersion] {
        &[CONSENSUS_VERSION]
    }

    fn supported_api_versions(&self) -> SupportedModuleApiVersions {
        SupportedModuleApiVersions::from_raw((u32::MAX, 0), (0, 0), &[(0, 0)])
    }

    /// Initialize the module
    async fn init(&self, args: &ServerModuleInitArgs<Self>) -> anyhow::Result<DynServerModule> {
        Ok(Meta {
            cfg: args.cfg().to_typed()?,
            our_peer_id: args.our_peer_id(),
            num_peers: args.num_peers(),
        }
        .into())
    }

    /// Generates configs for all peers in a trusted manner for testing
    fn trusted_dealer_gen(
        &self,
        peers: &[PeerId],
        params: &ConfigGenModuleParams,
    ) -> BTreeMap<PeerId, ServerModuleConfig> {
        let _params = self.parse_params(params).unwrap();
        // Generate a config for each peer
        peers
            .iter()
            .map(|&peer| {
                let config = MetaConfig {
                    local: MetaConfigLocal {},
                    private: MetaConfigPrivate,
                    consensus: MetaConfigConsensus {},
                };
                (peer, config.to_erased())
            })
            .collect()
    }

    /// Generates configs for all peers in an untrusted manner
    async fn distributed_gen(
        &self,
        _peers: &PeerHandle,
        params: &ConfigGenModuleParams,
    ) -> DkgResult<ServerModuleConfig> {
        let _params = self.parse_params(params).unwrap();

        Ok(MetaConfig {
            local: MetaConfigLocal {},
            private: MetaConfigPrivate,
            consensus: MetaConfigConsensus {},
        }
        .to_erased())
    }

    /// Converts the consensus config into the client config
    fn get_client_config(
        &self,
        config: &ServerModuleConsensusConfig,
    ) -> anyhow::Result<MetaClientConfig> {
        let _config = MetaConfigConsensus::from_erased(config)?;
        Ok(MetaClientConfig {})
    }

    fn validate_config(
        &self,
        _identity: &PeerId,
        _config: ServerModuleConfig,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// DB migrations to move from old to newer versions
    fn get_database_migrations(&self) -> BTreeMap<DatabaseVersion, ServerMigrationFn> {
        BTreeMap::new()
    }
}

/// Meta module
#[derive(Debug)]
pub struct Meta {
    pub cfg: MetaConfig,
    pub our_peer_id: PeerId,
    pub num_peers: NumPeers,
}

impl Meta {
    /// Check the difference between what's desired to be submitted and what's
    /// already submitted or consensus value.
    ///
    /// Returns:
    /// `(items_to_submit, desired_keys_to_delete)`
    async fn desired_submitted_diff(
        &self,
        dbtx: &mut DatabaseTransaction<'_, NonCommittable>,
    ) -> (Vec<MetaConsensusItem>, Vec<MetaKey>) {
        let desired: Vec<_> = Self::get_desired(dbtx).await;

        let mut to_delete: Vec<MetaKey> = vec![];
        let mut to_submit = vec![];

        for (key, MetaDesiredValue { value, salt }) in desired {
            if Self::get_submission(dbtx, key, self.our_peer_id)
                .await
                .as_ref()
                == Some(&MetaSubmissionValue {
                    value: value.clone(),
                    salt,
                })
            {
                to_delete.push(key);
            } else if Self::get_consensus(dbtx, key).await.as_ref() == Some(&value) {
                // When a value is new and equal to current consensus, it should both: be
                // submitted so it can clear/cancel previous submissions, and
                // deleted (afterwards).
                to_delete.push(key);
                to_submit.push(MetaConsensusItem { key, value, salt });
            } else {
                to_submit.push(MetaConsensusItem { key, value, salt });
            }
        }

        (to_submit, to_delete)
    }

    async fn get_desired(dbtx: &mut DatabaseTransaction<'_>) -> Vec<(MetaKey, MetaDesiredValue)> {
        dbtx.find_by_prefix(&MetaDesiredKeyPrefix)
            .await
            .map(|(k, v)| (k.0, v))
            .collect()
            .await
    }

    async fn get_submission(
        dbtx: &mut DatabaseTransaction<'_>,
        key: MetaKey,
        peer_id: PeerId,
    ) -> Option<MetaSubmissionValue> {
        dbtx.get_value(&MetaSubmissionsKey { key, peer_id }).await
    }

    async fn get_consensus(dbtx: &mut DatabaseTransaction<'_>, key: MetaKey) -> Option<MetaValue> {
        dbtx.get_value(&MetaConsensusKey(key))
            .await
            .map(|consensus_value| consensus_value.value)
    }

    async fn change_consensus(
        dbtx: &mut DatabaseTransaction<'_, NonCommittable>,
        key: MetaKey,
        value: MetaValue,
        matching_submissions: Vec<PeerId>,
    ) {
        let value_len = value.as_slice().len();
        let revision = dbtx
            .get_value(&MetaConsensusKey(key))
            .await
            .map(|cv| cv.revision);
        let revision = revision.map(|r| r.wrapping_add(1)).unwrap_or_default();
        dbtx.insert_entry(
            &MetaConsensusKey(key),
            &MetaConsensusValue { value, revision },
        )
        .await;

        info!(target: LOG_MODULE_META, %key, rev = %revision, len = %value_len, "New consensus value");

        for peer_id in matching_submissions {
            dbtx.remove_entry(&MetaSubmissionsKey { key, peer_id })
                .await;
        }
    }
}

/// Implementation of consensus for the server module
#[async_trait]
impl ServerModule for Meta {
    /// Define the consensus types
    type Common = MetaModuleTypes;
    type Init = MetaInit;

    async fn consensus_proposal(
        &self,
        dbtx: &mut DatabaseTransaction<'_>,
    ) -> Vec<MetaConsensusItem> {
        let (to_submit, _to_delete) = self.desired_submitted_diff(dbtx).await;

        // Note: regrettably we can't delete any desired keys here, as it could lead to
        // write-write db conflicts. So we just let the request handler do the
        // cleaning.
        to_submit
    }

    async fn process_consensus_item<'a, 'b>(
        &'a self,
        dbtx: &mut DatabaseTransaction<'b>,
        MetaConsensusItem { key, value, salt }: MetaConsensusItem,
        peer_id: PeerId,
    ) -> anyhow::Result<()> {
        let new_value = MetaSubmissionValue { value, salt };
        // first of all: any new submission overrides previous submission
        if let Some(prev_value) = Self::get_submission(dbtx, key, peer_id).await {
            if prev_value != new_value {
                dbtx.remove_entry(&MetaSubmissionsKey { key, peer_id })
                    .await;
            }
        }
        // then: if the submission is equal to the current consensus, it's ignored
        if Some(&new_value.value) == Self::get_consensus(dbtx, key).await.as_ref() {
            debug!(target: LOG_MODULE_META, %peer_id, %key, "Peer submitted a redundant value");
            return Ok(());
        }

        // otherwise, new submission is recorded
        dbtx.insert_entry(&MetaSubmissionsKey { key, peer_id }, &new_value)
            .await;

        // we check how many peers submitted the same value (including this peer)
        let matching_submissions: Vec<PeerId> = dbtx
            .find_by_prefix(&MetaSubmissionsByKeyPrefix(key))
            .await
            .filter(|(_submission_key, submission_value)| {
                future::ready(new_value.value == submission_value.value)
            })
            .map(|(submission_key, _)| submission_key.peer_id)
            .collect()
            .await;

        let threshold = self.num_peers.threshold();
        info!(target: LOG_MODULE_META,
             %peer_id,
             %key,
            value_len = %new_value.value.as_slice().len(),
             matching = %matching_submissions.len(),
            %threshold, "Peer submitted a value");

        // if threshold or more, change the consensus value
        if threshold <= matching_submissions.len() {
            Self::change_consensus(dbtx, key, new_value.value, matching_submissions).await;
        }

        Ok(())
    }

    async fn process_input<'a, 'b, 'c>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'c>,
        _input: &'b MetaInput,
    ) -> Result<InputMeta, MetaInputError> {
        Err(MetaInputError::NotSupported)
    }

    async fn process_output<'a, 'b>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'b>,
        _output: &'a MetaOutput,
        _out_point: OutPoint,
    ) -> Result<TransactionItemAmount, MetaOutputError> {
        Err(MetaOutputError::NotSupported)
    }

    async fn output_status(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
        _out_point: OutPoint,
    ) -> Option<MetaOutputOutcome> {
        None
    }

    async fn audit(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
        _audit: &mut Audit,
        _module_instance_id: ModuleInstanceId,
    ) {
    }

    fn api_endpoints(&self) -> Vec<ApiEndpoint<Self>> {
        vec![
            api_endpoint! {
                SUBMIT_ENDPOINT,
                ApiVersion::new(0, 0),
                async |module: &Meta, context, request: SubmitRequest| -> () {

                    match context.request_auth() {
                        None => return Err(ApiError::bad_request("Missing password".to_string())),
                        Some(auth) => {
                            module.handle_submit_request(&mut context.dbtx(), &auth, &request).await?;
                        }
                    }

                    Ok(())
                }
            },
            api_endpoint! {
                GET_CONSENSUS_ENDPOINT,
                ApiVersion::new(0, 0),
                async |module: &Meta, context, request: GetConsensusRequest| -> Option<MetaConsensusValue> {
                    module.handle_get_consensus_request(&mut context.dbtx().into_nc(), &request).await
                }
            },
            api_endpoint! {
                GET_CONSENSUS_REV_ENDPOINT,
                ApiVersion::new(0, 0),
                async |module: &Meta, context, request: GetConsensusRequest| -> Option<u64> {
                    module.handle_get_consensus_revision_request(&mut context.dbtx().into_nc(), &request).await
                }
            },
            api_endpoint! {
                GET_SUBMISSIONS_ENDPOINT,
                ApiVersion::new(0, 0),
                async |module: &Meta, context, request: GetSubmissionsRequest| -> GetSubmissionResponse {
                    match context.request_auth() {
                        None => return Err(ApiError::bad_request("Missing password".to_string())),
                        Some(auth) => {
                            module.handle_get_submissions_request(&mut context.dbtx().into_nc(),&auth, &request).await
                        }
                    }
                }
            },
        ]
    }
}

impl Meta {
    async fn handle_submit_request(
        &self,
        dbtx: &mut DatabaseTransaction<'_, Committable>,
        _auth: &ApiAuth,
        req: &SubmitRequest,
    ) -> Result<(), ApiError> {
        let salt = thread_rng().gen();

        // Since this is the only place in the code that touches "desired" keys, we
        // clean old keys here as well.
        // Note: clean-up previously desired values before inserting the new one. This
        // way if new value is already redundant (already submitted or equal to
        // consensus one), it will still get a chance to get submitted.
        let (_to_submit, to_delete) = self.desired_submitted_diff(&mut dbtx.to_ref_nc()).await;
        for key in to_delete {
            dbtx.remove_entry(&MetaDesiredKey(key)).await;
        }

        dbtx.insert_entry(
            &MetaDesiredKey(req.key),
            &MetaDesiredValue {
                value: req.value.clone(),
                salt,
            },
        )
        .await;

        Ok(())
    }

    async fn handle_get_consensus_request(
        &self,
        dbtx: &mut DatabaseTransaction<'_, NonCommittable>,
        req: &GetConsensusRequest,
    ) -> Result<Option<MetaConsensusValue>, ApiError> {
        Ok(dbtx.get_value(&MetaConsensusKey(req.0)).await)
    }

    async fn handle_get_consensus_revision_request(
        &self,
        dbtx: &mut DatabaseTransaction<'_, NonCommittable>,
        req: &GetConsensusRequest,
    ) -> Result<Option<u64>, ApiError> {
        Ok(dbtx
            .get_value(&MetaConsensusKey(req.0))
            .await
            .map(|cv| cv.revision))
    }

    async fn handle_get_submissions_request(
        &self,
        dbtx: &mut DatabaseTransaction<'_, NonCommittable>,
        _auth: &ApiAuth,
        req: &GetSubmissionsRequest,
    ) -> Result<BTreeMap<PeerId, MetaValue>, ApiError> {
        Ok(dbtx
            .find_by_prefix(&MetaSubmissionsByKeyPrefix(req.0))
            .await
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|(k, v)| (k.peer_id, v.value))
            .collect())
    }
}