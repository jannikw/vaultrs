use super::responses::ReadEntityResponse;
use rustify_derive::Endpoint;

/// ## Read entity by ID
/// This endpoint queries the entity by its identifier.
///
/// * Path: {self.mount}/{self.path}
/// * Method: GET
/// * Response: ReadEntityResponse
/// * Reference: https://developer.hashicorp.com/vault/api-docs/secret/identity/entity#read-entity-by-id
#[derive(Builder, Debug, Default, Endpoint)]
#[endpoint(
    path = "/identity/entity/id/{self.entity_id}",
    method = "GET",
    response = "ReadEntityResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadEntityRequest {
    #[endpoint(skip)]
    pub entity_id: String,
}
