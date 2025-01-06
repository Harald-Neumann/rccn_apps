use serde::{Deserialize};
use bincode;

use rccn_usr::service::{CommandParseError, CommandParseResult, ServiceCommand};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};

#[derive(Debug, Deserialize)]
pub struct BsdiffCreateCommand {
    #[serde(deserialize_with = "fixed_length_string")]
    pub file_1: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub file_2: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub output_file: String,
}

#[derive(Debug, Deserialize)]
pub struct BsdiffPatchCommand {
    #[serde(deserialize_with = "fixed_length_string")]
    pub source_file: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub patch_file: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub output_file: String,
}

fn fixed_length_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,	
{
    let bytes: [u8; 8] = serde::de::Deserialize::deserialize(deserializer)?;
    let s = String::from_utf8(bytes.to_vec()).map(|s| s.trim_matches('\0').to_string()).map_err(serde::de::Error::custom)?;
    Ok(s)
}

#[derive(Debug, Deserialize)]
pub enum BsdiffCommand {
    Create(BsdiffCreateCommand),
    Patch(BsdiffPatchCommand),
}

impl ServiceCommand for BsdiffCommand {
    fn from_pus_tc(tc: &PusTcReader) -> CommandParseResult<Self> {
        println!("Data: {:?}", tc.app_data());
        match tc.subservice() {
            1 => {
                let cmd = bincode::deserialize(tc.app_data())
                    .map_err(|_| CommandParseError::Other)?;
                Ok(Self::Create(cmd))
            },
            2 => {
                let cmd = bincode::deserialize(tc.app_data())
                    .map_err(|_| CommandParseError::Other)?;
                Ok(Self::Patch(cmd))
            },
            _ => {
                Err(CommandParseError::UnknownSubservice(tc.subservice()))
            },
        }
    }
}



// Test the deserialization of BsdiffCommand from PusTcReader
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bsdiff_command_deserialization() {
    }
}