# PayTech Rust Axum Demo

A Rust implementation of the PayTech payment gateway API using the Axum web framework. This project is a port of the Go Gin implementation to demonstrate PayTech integration in Rust.

## Features

- 🚀 Built with Axum web framework
- 💳 PayTech payment gateway integration
- 🛡️ HMAC signature verification for callbacks
- 📦 Product catalog management
- 🔧 Environment-based configuration with `.env` support
- 📝 Detailed logging and configuration feedback
- 🔍 Comprehensive error handling and API response debugging
- 🛠️ Flexible PayTech response parsing with fallback mechanisms

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Welcome message |
| `/api/list_product` | POST | List products with optional filtering |
| `/api/request_payment` | POST | Create payment request |
| `/api/callback_paytech` | POST | Handle PayTech IPN callbacks |

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd paytech-rust-axum-demo
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your PayTech credentials
```

## Configuration

Create a `.env` file with the following variables:

```env
# Server Configuration
PORT=8080

# PayTech API Configuration
PAYTECH_API_KEY=your_api_key_here
PAYTECH_API_SECRET=your_api_secret_here
PAYTECH_BASE_URL=https://paytech.sn/api
PAYTECH_ENV=test

# URLs for payment callbacks
PAYTECH_IPN_URL=https://your-domain.com/api/callback_paytech
PAYTECH_SUCCESS_URL=https://your-domain.com/payment/success
PAYTECH_CANCEL_URL=https://your-domain.com/payment/cancel
```

## Usage

1. Start the server:
```bash
cargo run
```

The application will:
- Automatically load environment variables from `.env` file
- Display configuration status on startup
- Show which environment variables are loaded
- Start the server on the configured PORT (default: 8080)

Example startup output:
```
Successfully loaded .env file
Loading PayTech configuration...
✓ PayTech API Key: test_api***
✓ PayTech Base URL: https://paytech.sn/api
✓ PayTech Environment: test
✓ PayTech IPN URL: http://localhost:8080/api/callback_paytech
✓ Server Port: 8080
Server running on http://0.0.0.0:8080
```

You can change the port by setting the `PORT` environment variable:
```bash
PORT=3000 cargo run
# or set it in your .env file
```

## API Usage Examples

### List Products
```bash
curl -X POST http://localhost:8080/api/list_product \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "allowed_type=electronics"
```

### Request Payment
```bash
curl -X POST http://localhost:8080/api/request_payment \
  -H "Content-Type: application/json" \
  -d '{
    "products": [
      {"uuid": "550e8400-e29b-41d4-a716-446655440001", "qty": 1},
      {"uuid": "550e8400-e29b-41d4-a716-446655440003", "qty": 2}
    ]
  }'
```


## License

This project is for demonstration purposes.