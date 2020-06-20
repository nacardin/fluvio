//!
//! # Spu Status
//!
//! Spu Status metadata information cached locally.
//!
use std::fmt;

use kf_protocol::derive::{Decode, Encode};



// -----------------------------------
// Data Structures
// -----------------------------------

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct SpuStatus {
    pub resolution: SpuResolution,
}

impl fmt::Display for SpuStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.resolution)
    }
}


impl Default for SpuStatus {
    fn default() -> Self {
        SpuStatus {
            resolution: SpuResolution::default(),
        }
    }
}

impl SpuStatus {
    /// Resolution to string label
    pub fn resolution_label(&self) -> &'static str {
        match self.resolution {
            SpuResolution::Online => "online",
            SpuResolution::Offline => "offline",
            SpuResolution::Init => "Init",
        }
    }

    /// Checks if resoultion is marked online. true for online, false otherwise
    pub fn is_online(&self) -> bool {
        self.resolution == SpuResolution::Online
    }

    pub fn is_offline(&self) -> bool {
        self.resolution == SpuResolution::Offline
    }

    /// Set resolution to status to online
    pub fn set_online(&mut self) {
        self.resolution = SpuResolution::Online;
    }

    /// Set resolution to status to offline
    pub fn set_offline(&mut self) {
        self.resolution = SpuResolution::Offline;
    }
}

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub enum SpuResolution {
    Online,
    Offline,
    Init,
}

// -----------------------------------
// Implementation - SpuResolution
// -----------------------------------

impl Default for SpuResolution {
    fn default() -> Self {
        SpuResolution::Init
    }
}
