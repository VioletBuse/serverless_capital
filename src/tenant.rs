use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tenant {
    pub module: String,
    pub id: String,
}

// pub struct Deployment {
//     pub
// }
