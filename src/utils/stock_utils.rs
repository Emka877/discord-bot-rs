use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub quote_response: QuoteResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub result: Vec<StockInfo>,
    pub error: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockInfo {
    pub language: String,
    pub region: String,
    pub quote_type: String,
    pub type_disp: String,
    pub quote_source_name: String,
    pub triggerable: bool,
    pub custom_price_alert_confidence: String,
    pub currency: String,
    pub first_trade_date_milliseconds: i64,
    pub price_hint: i64,
    pub post_market_change_percent: f64,
    pub post_market_time: i64,
    pub post_market_price: f64,
    pub post_market_change: f64,
    pub regular_market_change: f64,
    pub regular_market_change_percent: f64,
    pub regular_market_time: i64,
    #[serde(rename = "regularMarketPrice")]
    pub price: f64,
    pub regular_market_day_high: f64,
    pub regular_market_day_range: String,
    pub regular_market_day_low: f64,
    pub regular_market_volume: i64,
    pub regular_market_previous_close: f64,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub full_exchange_name: String,
    pub financial_currency: String,
    pub regular_market_open: f64,
    #[serde(rename = "averageDailyVolume3Month")]
    pub average_daily_volume3month: i64,
    #[serde(rename = "averageDailyVolume10Day")]
    pub average_daily_volume10day: i64,
    pub fifty_two_week_low_change: f64,
    pub fifty_two_week_low_change_percent: f64,
    pub fifty_two_week_range: String,
    pub fifty_two_week_high_change: f64,
    pub fifty_two_week_high_change_percent: f64,
    pub fifty_two_week_low: f64,
    pub fifty_two_week_high: f64,
    pub dividend_date: i64,
    pub exchange: String,
    pub short_name: String,
    #[serde(rename = "longName")]
    pub name: String,
    pub message_board_id: String,
    pub exchange_timezone_name: String,
    pub exchange_timezone_short_name: String,
    pub gmt_off_set_milliseconds: i64,
    pub market: String,
    pub esg_populated: bool,
    pub market_state: String,
    #[serde(rename = "earningsTimestamp")]
    pub earning_call_date: i64,
    pub earnings_timestamp_start: i64,
    pub earnings_timestamp_end: i64,
    pub trailing_annual_dividend_rate: f64,
    #[serde(rename = "trailingPE")]
    pub trailing_pe: f64,
    pub trailing_annual_dividend_yield: f64,
    pub eps_trailing_twelve_months: f64,
    pub eps_forward: f64,
    pub eps_current_year: f64,
    pub price_eps_current_year: f64,
    pub shares_outstanding: i64,
    pub book_value: f64,
    pub fifty_day_average: f64,
    pub fifty_day_average_change: f64,
    pub fifty_day_average_change_percent: f64,
    pub two_hundred_day_average: f64,
    pub two_hundred_day_average_change: f64,
    pub two_hundred_day_average_change_percent: f64,
    pub market_cap: i64,
    #[serde(rename = "forwardPE")]
    pub forward_pe: f64,
    pub price_to_book: f64,
    pub source_interval: i64,
    pub exchange_data_delayed_by: i64,
    pub page_view_growth_weekly: f64,
    #[serde(rename = "averageAnalystRating")]
    pub rating: String,
    pub tradeable: bool,
    pub display_name: String,
    #[serde(rename = "symbol")]
    pub ticker: String,
}

/**
 * Function that gets the stock price of a given stock, using the yahoo finance API.
 * The function takes a string as an argument, which is the stock ticker.
 */
pub async fn get_stock_price(ticker: String) -> Result<StockInfo, String> {
    // Perform the request using reqwest
    let response = reqwest::get(&format!(
        "https://query1.finance.yahoo.com/v7/finance/quote?symbols={}",
        ticker
    ))
    .await;
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
    // Check if the response body was successful
    if response_body.is_err() {
        return Err("Could not get the stock price.".to_string());
    }

    // Unwrap the response body and print it to the console for debug purposes
    let response_body = response_body.unwrap();

    println!("{:#}", response_body);

    // Parse the response body into a StockInfo struct
    let stock_info: Root = serde_json::from_str(&response_body).unwrap();

    let reply_length = stock_info.quote_response.result.len();
    if reply_length == 0 {
        return Err("Could not get the stock price.".to_string());
    }

    // Return the price of the stock
    return Ok(stock_info.quote_response.result[0].clone());
}

// Function that transforms an epoch timestamp into a human readable date
pub fn epoch_to_date(epoch: i64) -> String {
    let date = chrono::NaiveDateTime::from_timestamp(epoch as i64, 0);
    let date_string = date.format("%Y-%m-%d %H:%M:%S").to_string();
    return date_string;
}
