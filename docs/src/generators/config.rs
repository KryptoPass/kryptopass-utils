use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub properties: Properties,
    pub requirements: Requirements,
    pub allowed: Option<Allowed>,
    pub not_allowed: Option<NotAllowed>,
    pub rules: Option<Rules>,
    #[serde(flatten)]
    pub languages: HashMap<String, LanguageSets>,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub version: String,
    pub lang: Vec<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub generation_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Requirements {
    pub length: LengthRequirement,
    #[serde(flatten)]
    pub sets: HashMap<String, SetRequirement>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LengthRequirement {
    Range { min: usize, max: usize },
    Exact(usize),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SetRequirement {
    Range { min: usize, max: Option<usize> },
    Exact(usize),
}

#[derive(Debug, Deserialize)]
pub struct Allowed {
    pub include: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct NotAllowed {
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rules {
    #[serde(rename = "max-consecutive")]
    pub max_consecutive: Option<usize>,
    #[serde(rename = "min-entropy-bits")]
    pub min_entropy_bits: Option<f64>,
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LanguageSets {
    #[serde(flatten)]
    pub sets: HashMap<String, Vec<String>>,
}
