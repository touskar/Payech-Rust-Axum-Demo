use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub paytech_api_key: String,
    pub paytech_api_secret: String,
    pub paytech_base_url: String,
    pub paytech_env: String,
    pub paytech_ipn_url: String,
    pub paytech_success_url: String,
    pub paytech_cancel_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Print loaded environment variables for debugging (without showing secrets)
        println!("Loading PayTech configuration...");
        
        let api_key = env::var("PAYTECH_API_KEY")
            .map_err(|_| "PAYTECH_API_KEY environment variable is required. Please set it in .env file.")?;
        
        let api_secret = env::var("PAYTECH_API_SECRET")
            .map_err(|_| "PAYTECH_API_SECRET environment variable is required. Please set it in .env file.")?;
        
        let base_url = env::var("PAYTECH_BASE_URL")
            .unwrap_or_else(|_| "https://paytech.sn/api".to_string());
        
        let paytech_env = env::var("PAYTECH_ENV")
            .unwrap_or_else(|_| "test".to_string());
        
        let ipn_url = env::var("PAYTECH_IPN_URL")
            .unwrap_or_else(|_| "https://secure-3ds.intech.sn/ping".to_string());
        
        let success_url = env::var("PAYTECH_SUCCESS_URL")
            .unwrap_or_else(|_| "https://example.com/payment/success".to_string());
        
        let cancel_url = env::var("PAYTECH_CANCEL_URL")
            .unwrap_or_else(|_| "https://example.com/payment/cancel".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| "PORT must be a valid number between 1 and 65535")?;

        println!("✓ PayTech API Key: {}***", &api_key[..std::cmp::min(8, api_key.len())]);
        println!("✓ PayTech Base URL: {}", base_url);
        println!("✓ PayTech Environment: {}", paytech_env);
        println!("✓ PayTech IPN URL: {}", ipn_url);
        println!("✓ Server Port: {}", port);
        
        Ok(Config {
            paytech_api_key: api_key,
            paytech_api_secret: api_secret,
            paytech_base_url: base_url,
            paytech_env,
            paytech_ipn_url: ipn_url,
            paytech_success_url: success_url,
            paytech_cancel_url: cancel_url,
            port,
        })
    }
}