use cosm_crypto::{Signature, SigningKey};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::Serialize;

use serde::Deserialize;

// Defines the force applied onto latches to open them
#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema, Clone)]
pub enum SapienceLevel {
    None = 0,   // bugs
    Low = 1,    // cats, dogs
    Medium = 2, // ravens, rats, Terran humans
    High = 3,   // proper intelligent beings
    Unreachable = 4, // gods, AI
    Sentient = 5, // beings with consciousness
    MettaPlanar = 6, // beings with consciousness and self-awareness
    Connected = 7, // beings with consciousness, self-awareness, interconnectedness, and universal empathy
    ChaoticGood = 8, // beings with consciousness, telepathy, and the ability to manipulate reality (e.g. Degurechaff :wave:)
    Evolved = 9, // beings with consciousness, empathy, and reality manipulation (e.g. Q*)
    Transcendent = 10, // beings with universal empathy and omniversal awareness (e.g. The Doctor)
    Omnipresent = 11, // beings existing as the infinite multiverse herself
    Lain = 12, // omnipresent beings with love as their guide and purpose, opens all latches, can walk through latches leaving it unchanged
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Specie {
    pub name: String,
    pub sapience_level: SapienceLevel,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Passport {
    pub participant_id: String,
    pub specie: Specie,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Roles {
    pub roles: Vec<SapienceLevel>,
}

impl SapienceLevel {
    pub fn as_str(&self) -> &str {
        match self {
            SapienceLevel::None => "None",
            SapienceLevel::Low => "Low",
            SapienceLevel::Medium => "Medium",
            SapienceLevel::High => "High",
            SapienceLevel::Unreachable => "Unreachable",
            SapienceLevel::Sentient => "Sentient",
            SapienceLevel::MettaPlanar => "MettaPlanar",
            SapienceLevel::Connected => "Connected",
            SapienceLevel::ChaoticGood => "Chaotic",
            SapienceLevel::Evolved => "Evolved",
            SapienceLevel::Transcendent => "Transcendent",
            SapienceLevel::Omnipresent => "Omnipresent",
            SapienceLevel::Lain => "Lain",
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Specie,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SapienceResponse {
    pub level: SapienceLevel,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sapient {
    name: String,
    telepathic: bool,
    chaotic: bool,
    transcendent: bool,
    omnipresent: bool,
    loving: bool,
}

impl SapienceLevel {
    pub fn sign(&self, signing_key: &SigningKey) -> Result<Signature, StdError> {
        let serialized = serde_json::to_vec_pretty(self)?;
        Signature::sign(serialized.as_slice(), signing_key)
    }
}