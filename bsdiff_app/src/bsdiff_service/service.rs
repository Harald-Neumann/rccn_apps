use std::io;
use std::io::Cursor;
use qbsdiff::{Bsdiff, Bspatch};
use std::fs;

use crate::bsdiff_service::command::{BsdiffCommand, BsdiffCreateCommandResponse};
use rccn_usr::service::{AcceptanceResult, AcceptedTc, PusService, SubserviceTmData};

use super::command::BsdiffCreateCommand;
use crate::bsdiff_service::telemetry::CreateResponse;

pub struct BsdiffService {}

impl BsdiffService {
    pub fn new() -> Self {
        BsdiffService {}
    }

    fn handle_create(cmd: BsdiffCreateCommand) -> Result<Vec<u8>, io::Error> {
        let source = fs::read(cmd.source_file)?;
        let target = fs::read(cmd.target_file)?;
            
        let mut output = Vec::new();
            
        Bsdiff::new(&source, &target)
            .compare(Cursor::new( &mut output))?;

        fs::write(cmd.output_file, &output)?;
        Ok(output)
    }
}


impl PusService for BsdiffService {
    type CommandT = BsdiffCommand;

    fn service() -> u8 {
        131
    }

    fn handle_tc(&mut self, mut tc: AcceptedTc, cmd: Self::CommandT) -> AcceptanceResult {
        println!("Handling");
        match cmd {
            BsdiffCommand::Create(cmd) if cmd.response == BsdiffCreateCommandResponse::Yes => tc.handle_with_tm(|| {
                let patch_data = Self::handle_create(cmd)?;
                Ok::<SubserviceTmData, io::Error>(SubserviceTmData{subservice: 1, data: CreateResponse{length: patch_data.len() as u32, data: patch_data}.into()})
            }),
            BsdiffCommand::Create(cmd) => tc.handle( || {
                Self::handle_create(cmd).is_ok()
            }),
            
            BsdiffCommand::Patch(cmd) => tc.handle( || {
                let Ok(source) = fs::read(cmd.source_file) else {
                    return false;
                }; 
                let Ok(patch) = fs::read(cmd.patch_file) else {
                    return false;
                };
                let Ok(target_file) = fs::File::create(cmd.output_file) else {
                    return false;
                };
                
                Bspatch::new(&patch).and_then(|patcher| patcher.apply(&source, &target_file)).is_ok()
            }),
        }
    }
}