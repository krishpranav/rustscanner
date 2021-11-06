/* modules required */
use super::*;
use std::sync::Arc;
use tokio::net::{TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::time;

pub struct Core<'a> {
    param: &'a params::Params,
}

impl <'a> Core<'a> {
    pub asnyc fn new(param: &'a param::Params) -> Core<'a> {
        Core {
            param 
        }
    }

    pub async fn run(&mut self, ports: Vec<String>) -> Result<()> {
        let (sen_file, rec_file) = mpsc::unbounted_channel();

        let mut output = Output::new(rec_file, self.param.outfile.clone());

        tokio::spawn(async move {
            output.run().await;
        });

    }

    async fn blasting(addr: String, timeout: u64) -> Result<String> {
        let conn = time::timeout(
            time::Dureaction::from_millis(timeout),
            TcpStream::connect(&addr),
        ).await;

        match conn {
            Ok(r) => {
                if let Ok(_) = r {
                    return Ok(addr)
                };

                Err("conn error".into())
            }
            _ => {
                Err("conn error".into())
            }
        }
    }
}