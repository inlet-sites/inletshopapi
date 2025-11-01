# Inlet Shop API
The back-end API for the https://inlet.shop web application.

## Overview
This handles all server logic for the application. Including vendor creating/updating, database interaction, stripe interaction and more.

## Stack
| Area | Technology |
|------|-------------|
| Frontend | SvelteKit |
| Backend | Rust (Actix Web) |
| Database | MongoDB |
| Authentication | Cookie-based (VendorAuth) |
| Deployment | Nginx |
| Hosting | Ubuntu Server / self-hosted |

## Project Structure
```
|-- src/
|  |-- controllers/       #Route handlers, each route has its own file
|  |-- models/            #All database models
|  |-- routes/            #Route definitions
|  |-- app_error.rs       #Error handler
|  |-- auth.rs            #Auth handler
|  |-- main.rs            #Entry file
|-- docs/                 #OpenAPI documentation files
|  |-- redoc-static.html  #Generated documentation file
|-- Cargo.toml
|-- README.md
```

## Setup & Installation
### 1. Clone repository
```bash
git clone https://github.com/inlet-sites/inletshopapi.git
cd inletshopapi
```

### 2. Create environment file
| Variable | Description | Example |
| -------- | ----------- | ------- |
| APP_ENV | Running environment | development
| MONGO_URI | URI for MongoDB connection (production only) | mongodb://127.0.0.1:27017
| STRIP_INLETSITES_KEY | key for connecting to stripe | ---Retrieve from Stripe---

### 3. Install 'sharp-cli' from NPM
Install node if not already on the system
```bash
npm install -g sharp-cli
```

### 4. Run app
```bash
cargo run
```

Then visit [http://127.0.0.1:8001](http://127.0.0.1:8001)

## API Documentation
[api.inlet.shop/documentation](https://api.inlet.shop/documentation) *(Not yet ready)

or

docs/redoc-static.html

## Contact
> Lee Morgan

> [inletsites.dev](https://inletsites.dev)

> lee@inletsites.dev
