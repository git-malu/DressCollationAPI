/*
 * DressCollationService API
 *
 * A REST API for managing clothing items and collations for users.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `clothes_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClothesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `clothes_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClothesIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `clothes_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClothesIdGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `clothes_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClothesIdPutError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `clothes_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClothesPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `collations_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollationsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `collations_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollationsIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `collations_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollationsIdGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `collations_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollationsIdPutError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `collations_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CollationsPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `folders_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `folders_id_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `folders_id_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersIdGetError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `folders_id_put`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersIdPutError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `folders_post`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldersPostError {
    UnknownValue(serde_json::Value),
}


pub async fn clothes_get(configuration: &configuration::Configuration, ) -> Result<Vec<crate::models::Clothing>, Error<ClothesGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/clothes", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClothesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn clothes_id_delete(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<ClothesIdDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/clothes/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<ClothesIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn clothes_id_get(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Clothing, Error<ClothesIdGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/clothes/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClothesIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn clothes_id_put(configuration: &configuration::Configuration, id: &str, clothing: crate::models::Clothing) -> Result<crate::models::Clothing, Error<ClothesIdPutError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/clothes/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&clothing);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClothesIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn clothes_post(configuration: &configuration::Configuration, clothing: crate::models::Clothing) -> Result<crate::models::Clothing, Error<ClothesPostError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/clothes", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&clothing);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ClothesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn collations_get(configuration: &configuration::Configuration, ) -> Result<Vec<crate::models::Collation>, Error<CollationsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/collations", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CollationsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn collations_id_delete(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<CollationsIdDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/collations/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<CollationsIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn collations_id_get(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Collation, Error<CollationsIdGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/collations/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CollationsIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn collations_id_put(configuration: &configuration::Configuration, id: &str, collation: crate::models::Collation) -> Result<crate::models::Collation, Error<CollationsIdPutError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/collations/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&collation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CollationsIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn collations_post(configuration: &configuration::Configuration, collation: crate::models::Collation) -> Result<crate::models::Collation, Error<CollationsPostError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/collations", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&collation);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CollationsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn folders_get(configuration: &configuration::Configuration, ) -> Result<Vec<crate::models::Folder>, Error<FoldersGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/folders", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FoldersGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn folders_id_delete(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<FoldersIdDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/folders/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<FoldersIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn folders_id_get(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Folder, Error<FoldersIdGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/folders/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FoldersIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn folders_id_put(configuration: &configuration::Configuration, id: &str, folder: crate::models::Folder) -> Result<crate::models::Folder, Error<FoldersIdPutError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/folders/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&folder);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FoldersIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn folders_post(configuration: &configuration::Configuration, folder: crate::models::Folder) -> Result<crate::models::Folder, Error<FoldersPostError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/folders", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&folder);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FoldersPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

