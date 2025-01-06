use serde::Deserialize;
use std::io;
use bincode;

use rccn_usr::service::{CommandParseError, CommandParseResult, ServiceCommand};
use satrs::spacepackets::ecss::{tc::PusTcReader, PusPacket};

const FILE_NAME_LENGTH: usize = 8;

#[derive(Debug, Deserialize)]
pub struct BsdiffCreateCommand {
    #[serde(deserialize_with = "fixed_length_string")]
    pub source_file: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub target_file: String,
    #[serde(deserialize_with = "fixed_length_string")]
    pub output_file: String,
    pub response: BsdiffCreateCommandResponse,
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
    let bytes: [u8; FILE_NAME_LENGTH] = serde::de::Deserialize::deserialize(deserializer)?;
    let s = String::from_utf8(bytes.to_vec()).map(|s| s.trim_matches('\0').to_string()).map_err(serde::de::Error::custom)?;
    Ok(s)
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(try_from = "u8")]
#[repr(u8)]
pub enum BsdiffCreateCommandResponse {
    Yes = 1,
    No = 2,
}

// To be used by serde
impl TryFrom<u8> for BsdiffCreateCommandResponse {
    type Error = io::Error; // Todo: Check if this is the correct error type

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Yes),
            2 => Ok(Self::No),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid value for BsdiffCreateCommandResponse")),
        }
    }
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



#[cfg(test)]
mod tests {
    use crate::bsdiff_service::command::{BsdiffCreateCommand, BsdiffCreateCommandResponse};

    #[test]
    fn test_bsdiff_command_deserialization() {
        let data = [97, 0, 0, 0, 0, 0, 0, 0, 101, 102, 103, 104, 105, 106, 99, 107, 98, 99, 100, 0, 0, 0, 0, 0, 2];
        let cmd: BsdiffCreateCommand = bincode::deserialize(&data).unwrap();
        assert_eq!(cmd.source_file, "a");
        assert_eq!(cmd.target_file, "efghijck");
        assert_eq!(cmd.output_file, "bcd");
        assert_eq!(cmd.response, BsdiffCreateCommandResponse::No);
    }
}