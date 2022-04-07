/**
 * Function that gets the stock price of a given stock, using the yahoo finance API.
 * The function takes a string as an argument, which is the stock ticker.
 */
pub async fn get_stock_price(ticker: String) -> Result<f64, String> {
    // Perform the request using reqwest
    let response = reqwest::get(&format!("https://query1.finance.yahoo.com/v7/finance/quote?symbols={}", ticker)).await;
    // Check if the request was successful
    if response.is_err() {
        return Err("Could not get the stock price.".to_string());
    }
    // Get the response
    let response = response.unwrap();
    // Check if the response was successful
    if !response.status().is_success() {
        return Err("Could not get the stock price.".to_string());
    }
    // Get the response body
    let response_body = response.text().await;
    // Return the price of the stock
    return Ok(response_body.unwrap().parse::<f64>().unwrap());
}