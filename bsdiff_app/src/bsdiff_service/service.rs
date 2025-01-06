use std::io;
use std::io::Cursor;
use qbsdiff::{Bsdiff, Bspatch};
use std::fs;

use crate::bsdiff_service::command::BsdiffCommand;
use rccn_usr::{pus::parameter_management_service::service, service::{AcceptanceResult, AcceptedTc, CommandExecutionStatus, PusService, SubserviceTmData}};

pub struct BsdiffService {}

impl BsdiffService {
    pub fn new() -> Self {
        BsdiffService {}
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
            BsdiffCommand::Create(cmd) => tc.handle_with_tm(|| {
                let source = fs::read(cmd.file_1)?;
                let target = fs::read(cmd.file_2)?;
            
                let mut output = Vec::new();
            
                Bsdiff::new(&source, &target)
                    .compare(Cursor::new( &mut output))?;

                fs::write(cmd.output_file, &output)?;
                Ok::<SubserviceTmData, io::Error>(SubserviceTmData{subservice: Self::service(), data: output})
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