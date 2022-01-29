use curl::easy::{Easy2, List, Handler, WriteError};
use curl::multi::{Easy2Handle, Multi};
use anyhow::Result;
use base64::encode;
use std::collections::HashMap;

const URL: &str = "https://challenges.qluv.io/items/";

struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

/// Initialize a request that requests information for a specific id.
fn init_request(multi: &mut Multi, token: usize, id: &str) -> Result<Easy2Handle<Collector>> {
    let mut request = Easy2::new(Collector(Vec::new()));
    let url = format!("{}{}", URL, id);
    request.url(&url)?;

    // set header
    let mut list = List::new();
    let header = format!("Authorization: {}", encode(id));
    list.append(&header).unwrap();
    request.http_headers(list).unwrap();

    let mut handle = multi.add2(request)?;
    handle.set_token(token)?;
    Ok(handle)
}

/// Perform simultaneous requests for all IDs and update requested information in infos.
/// Print status of requests in real time if verbose is set to true.
pub fn multi_requests<'a>(ids: &'a Vec<String>, infos: &mut HashMap<&'a String, String>, verbose: bool) -> Result<()> {
    let mut multi = Multi::new();

    // set to 5 so that new requests are pended until less than 5 requests are being processed by the API
    multi.set_max_total_connections(5)
         .expect("failed to set the maximum number of simultaneously open connections");

    let mut handles = HashMap::new();

    // initialize requests
    for (token, id) in ids.iter().enumerate() {
        // requests are created only for ids not seen before
        if !infos.contains_key(&id) {
            infos.insert(id, "".to_string());
            handles.insert(token, init_request(&mut multi, token, id)?);
        }
    }

    let mut exist_unfinished_requests = true;
    while exist_unfinished_requests {
        if multi.perform()? == 0 {
            // set to true when results of all requests have been received
            exist_unfinished_requests = false;
        }
        multi.messages(|msg| {
            let token = msg.token().expect("failed to get token");
            let handle = handles.get_mut(&token).expect("failed to retrieve handle by token");
            match msg.result_for2(&handle).unwrap() {
                Ok(()) => {
                    let http_status = handle.response_code().expect("no status code");
                    // update (ID, information) pair
                    infos.insert(&ids[token], String::from_utf8(handle.get_ref().0.clone()).unwrap());
                    // print http status and requested information if verbose is true
                    if verbose {
                        println!("(Status: {}) (ID: {} -> {:?})", http_status, ids[token],
                                 String::from_utf8(handle.get_ref().0.clone()).unwrap());
                    }
                }
                Err(e) => {
                    println!("ID: {} -> Error: {}", ids[token], e);
                }
            }
        });
    }

    Ok(())
}