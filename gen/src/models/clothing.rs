/*
 * DressCollationService API
 *
 * A REST API for managing clothing items and collations for users.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clothing {
    /// Unique identifier for the clothing item.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the clothing item.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "body_part", skip_serializing_if = "Option::is_none")]
    pub body_part: Option<crate::models::BodyPart>,
    /// A list of seasons in which the clothing item is appropriate to wear.
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<i32>>,
    /// A list of image URLs associated with the clothing item.
    #[serde(rename = "pictures", skip_serializing_if = "Option::is_none")]
    pub pictures: Option<Vec<String>>,
    /// Description of the clothing item.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Rating of the clothing item (0-5).
    #[serde(rename = "rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
    /// A list of tags associated with the clothing item.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl Clothing {
    pub fn new(id: String, name: String) -> Clothing {
        Clothing {
            id,
            name,
            body_part: None,
            seasons: None,
            pictures: None,
            description: None,
            rating: None,
            tags: None,
        }
    }
}


