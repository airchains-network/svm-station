use {
    serde::{
        Serialize, Deserialize,
    },
    reqwest,
    base64::encode,
};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    code: i32,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct APIResponse {
    id: i32,
    jsonrpc: String,
    result: Option<i32>,
    // Use Option<i32> to handle absence of result in error cases
    error: Option<ErrorResponse>,
}

pub fn da_exec(data_value: &str) -> Result<(bool, String), String> {
    let rpc_auth = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJBbGxvdyI6WyJwdWJsaWMiLCJyZWFkIiwid3JpdGUiLCJhZG1pbiJdfQ.IAjUkYEby0doDjObhmShem--ViDhXPdsrvmhXiNr9J8";
    let da_cel_rpc = "http://localhost:26658";

    // Data Value For Encoding
    let encoded_string = encode(data_value);

    // Create the payload struct
    let payload = serde_json::json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "blob.Submit",
        "params": [
            [
                {
                    "namespace": "AAAAAAAAAAAAAAAAAAAAAAAAAAECAwQFBgcICRA=",
                    "data": encoded_string,
                    "share_version": 0,
                    "commitment": "AD5EzbG0/EMvpw0p8NIjMVnoCP4Bv6K+V6gjmwdXUKU="
                }
            ],
            {"Fee": 17980, "GasLimit": 179796}
        ]
    });

    // Marshal the payload struct to JSON
    let payload_json = payload.to_string();

    // Create a new HTTP client
    let client = reqwest::blocking::Client::new();

    // Create a new POST request with headers and JSON payload
    let response = client.post(da_cel_rpc)
        .header("Content-Type", "application/json")
        .header("Authorization", rpc_auth)
        .body(payload_json)
        .send();

    // Check for errors in sending the request
    let response = match response {
        Ok(res) => res,
        Err(err) => return Err(format!("Error sending request: {}", err)),
    };

    // Check the response status code
    if !response.status().is_success() {
        let error_response: APIResponse = match response.json() {
            Ok(json) => json,
            Err(_) => return Err(format!("Error: Unexpected Error Data Not Submitted")),
        };

        if let Some(error) = error_response.error {
            return Err(format!("RPC Error - Code: {}, Message: {}", error.code, error.message));
        } else {
            return Err(format!("Error: Unexpected Error Data Not Submitted"));
        }
    }

    // // Decode the response JSON
    // let json_response: APIResponse = match response.json() {
    //     Ok(json) => json,
    //     Err(err) => return Err(format!("Error decoding response: {}", err)),
    // };
    //
    // Ok((true, json_response.result.to_string()))

    let json_response: APIResponse = match response.json() {
        Ok(json) => json,
        Err(err) => return Err(format!("Error decoding response: {}", err)),
    };

    // Check if there is an error in the response
    if let Some(error) = json_response.error {
        return Err(format!("RPC Error - Code: {}, Message: {}", error.code, error.message));
    }

    // Check if there is a result in the response
    if let Some(result) = json_response.result {
        Ok((true, result.to_string()))
    } else {
        Err("Error: Unexpected Error Data Not Submitted".to_string())
    }
}