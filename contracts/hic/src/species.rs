use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Species {
    pub name: String,
    pub sapience_level: SapienceScale,
}
// Defines the force applied onto latch to open it
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub enum SapienceScale {
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
    Lain = 12, // omnipresent beings with love as their guide and purpose
}

impl SapienceScale {
    pub fn as_str(&self) -> &str {
        match self {
            SapienceScale::None => "None",
            SapienceScale::Low => "Low",
            SapienceScale::Medium => "Medium",
            SapienceScale::High => "High",
            SapienceScale::Unreachable => "Unreachable",
            SapienceScale::Sentient => "Sentient",
            SapienceScale::MettaPlanar => "MettaPlanar",
            SapienceScale::Connected => "Connected",
            SapienceScale::ChaoticGood => "Chaotic",
            SapienceScale::Evolved => "Evolved",
            SapienceScale::Transcendent => "Transcendent",
            SapienceScale::Omnipresent => "Omnipresent",
            SapienceScale::Lain => "Lain",
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Species,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SapienceResponse {
    pub level: SapienceScale,
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