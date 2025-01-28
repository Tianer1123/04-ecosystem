use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    dob: DateTime<Utc>,
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User {
        name: "Alice".to_string(),
        age: 28,
        dob: Utc::now(),
        skills: vec!["rust".to_string(), "Python".to_string()],
    };
    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user1 = serde_json::from_str(&json)?;
    println!("{:?}", user1);

    assert_eq!(user, user1);
    Ok(())
}
