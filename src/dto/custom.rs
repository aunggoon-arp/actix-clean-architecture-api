use utoipa::IntoParams;

#[derive(IntoParams, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ParamRequest {
    pub id: i32,
}