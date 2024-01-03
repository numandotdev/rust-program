use reqwest;

pub async fn make_api_request(api_url: &str) -> Result<(), reqwest::Error> {
    // Make a GET request to the API
    let response = reqwest::get(api_url).await?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;
        println!("API Response: {}", body);
    } else {
        println!("API Request failed with status: {}", response.status());
    }

    Ok(())
}
