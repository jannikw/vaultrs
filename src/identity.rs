use crate::{
    api::{
        self,
        identity::{requests::ReadEntityRequest, responses::ReadEntityResponse},
    },
    client::Client,
    error::ClientError,
};

/// Read entity by ID
///
/// See [ReadEntityRequest]
#[instrument(skip(client), err)]
pub async fn read_entity_by_id(
    client: &impl Client,
    entity_id: &str,
) -> Result<ReadEntityResponse, ClientError> {
    let endpoint = ReadEntityRequest::builder()
        .entity_id(entity_id)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}
