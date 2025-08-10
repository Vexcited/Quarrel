use serde::{Deserialize, Serialize};

/// https://github.com/discord-userdoccers/discord-userdoccers/blob/master/pages/reference.mdx#operating-system-type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OperatingSystemType {
  #[serde(rename = "Android")]
  Android,
  #[serde(rename = "Mac OS X")]
  Mac,
  #[serde(rename = "iOS")]
  Ios,
  #[serde(rename = "Linux")]
  Linux,
  #[serde(rename = "Windows")]
  Windows,
  #[serde(rename = "PlayStation")]
  PlayStation,
  #[serde(rename = "Xbox")]
  Xbox,
  #[serde(rename = "Unknown")]
  #[serde(other)]
  Unknown,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialize_os() {
    assert_eq!(
      serde_json::to_string(&OperatingSystemType::Ios).unwrap(),
      r#""iOS""#
    );

    assert_eq!(
      serde_json::to_string(&OperatingSystemType::Mac).unwrap(),
      r#""Mac OS X""#
    );
  }

  #[test]
  fn test_deserialize_known_os() {
    let json = r#""Windows""#;
    let os: OperatingSystemType = serde_json::from_str(json).unwrap();
    assert_eq!(os, OperatingSystemType::Windows);
  }

  #[test]
  fn test_deserialize_unknown_os() {
    let json = r#""Win95""#;
    let os: OperatingSystemType = serde_json::from_str(json).unwrap();
    assert_eq!(os, OperatingSystemType::Unknown);
  }
}
