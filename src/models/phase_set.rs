use serde::Deserialize;
use serde::Serialize;

use crate::models::phase::Phase;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PhaseSet {
    pub phases: Vec<Phase>,
}

impl PhaseSet {
    pub fn new() -> Self {
        Self {
            phases: vec![
                Phase::new(1, "Phase 1: ...".to_string()),
                Phase::new(2, "Phase 2: ...".to_string()),
                Phase::new(3, "Phase 3: ...".to_string()),
                Phase::new(4, "Phase 4: ...".to_string()),
            ],
        }
    }
}

impl Default for PhaseSet {
    fn default() -> Self {
        Self::new()
    }
}
