// background
// Type:string | null
// min length:  
// 1
// max length:  
// 128

// content
// Type:string | null
// max length:  
// 2000

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct background(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content(String);