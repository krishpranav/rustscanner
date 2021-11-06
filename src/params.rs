use std::path::PathBuf;
use structopt::StructOpt;
use super::*;

#[derive(StructOpt, Debug)]

impl Params {
    pub async fn get_ports(&self) -> Result<Vec<String>> {
        let idx1 = match self.port.find("-") {
            Some(idx) => idx,
            None => 0,
        };

        let mut lists = Vec::new();
        Ok(lists)
    }
}