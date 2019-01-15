use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use failure::Error;
use std::collections::HashMap;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct TravisError {
    error_type : Option<String>,
    error_message : Option<String>
}

fn main() -> Result<(), Error> {
    let client = Client::new();

    let mut request_params = HashMap::new();
    request_params.insert("message", "Autobuild triggered by new WLCS build");
    request_params.insert("branch", "master");

    let mut body_params = HashMap::new();
    body_params.insert("request", request_params);

    let request = client.post("https://api.travis-ci.org/repo/RAOF%2findurs/requests")
        .json(&body_params);

    let request = request
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, "token TOKEN")
        .header("Travis-API-Version", "3");
    println!("Sending {:#?}", &request);

    let request = request.build()?;

    println!("Request is {:#?}", &request);
    println!("Body is {:#?}", request.body());

    let mut response = client.execute(request)?;

    println!("{:#?}", response);

    println!("{:#?}", response.text()?);

    let error : HashMap<String, String> = response.json()?;
    println!("{:#?}", error);
    Ok(())
}
