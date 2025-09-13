use crate::*;

// Used to deploy aea contracts with the account-factory
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct DeployArgs {
    pub account_prefix: String,
    pub wasm_base64: String,
    pub init_method_name: String,
    pub init_args: Option<String>,
}

// Used to deploy third party dao contracts
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DeployContractInput {
    /// Contract args
    pub dao_id: DaoId,
    pub account_prefix: String,
    pub wasm_hash: CryptoHash,
    pub init_method_name: String,
    pub init_args: Option<String>,
    pub initial_balance: YoctoAmount,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    /// Video args
    pub name: String,
    pub description: Option<String>,
    pub video: VideoHash,
    pub image: ImageHash,
    pub location: Option<String>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct DeployContractInputVideoOption {
    /// Contract args
    pub dao_id: DaoId,
    pub account_prefix: String,
    pub wasm_hash: CryptoHash,
    pub init_method_name: String,
    pub init_args: Option<String>,
    pub initial_balance: YoctoAmount,
    pub repository_url: Option<String>,
    pub compiler_version: Option<String>,
    /// Video args
    pub name: String,
    pub description: Option<String>,
    pub video_bundle: Option<VideoBundle>,
    pub location: Option<String>,
}

impl DeployContractInput {
    pub fn from_video_option(
        input: DeployContractInputVideoOption,
        proposal_video: VideoHash,
        proposal_image: ImageHash,
    ) -> Self {
        let (video, image) = if let Some(bundle) = input.video_bundle {
            (bundle.video, bundle.image)
        } else {
            (proposal_video, proposal_image)
        };

        Self {
            // Contract fields
            dao_id: input.dao_id,
            account_prefix: input.account_prefix,
            wasm_hash: input.wasm_hash,
            init_method_name: input.init_method_name,
            init_args: input.init_args,
            initial_balance: input.initial_balance,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            // Video fields
            name: input.name,
            description: input.description,
            video,
            image,
            location: input.location,
        }
    }

    pub fn unwrap_video_option(input: DeployContractInputVideoOption) -> Self {

        let bundle = input.video_bundle.unwrap();

        Self {
            // Contract fields
            dao_id: input.dao_id,
            account_prefix: input.account_prefix,
            wasm_hash: input.wasm_hash,
            init_method_name: input.init_method_name,
            init_args: input.init_args,
            initial_balance: input.initial_balance,
            repository_url: input.repository_url,
            compiler_version: input.compiler_version,
            // Video fields
            name: input.name,
            description: input.description,
            video: bundle.video,
            image: bundle.image,
            location: input.location,
        }
    }
}