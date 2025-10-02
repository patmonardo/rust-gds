# Rust Property API with NAPI

This project implements a basic Property API using Rust and NAPI for integration with Node.js. The API allows for managing properties, including creating, retrieving, and updating property information.

## Project Structure

```
rust-property-api
├── src
│   ├── lib.rs          # Main entry point for the Rust library
│   ├── handlers.rs     # API handlers for property operations
│   ├── models.rs       # Data structures for properties
│   ├── errors.rs       # Custom error types for the API
│   └── types
│       └── mod.rs      # Type definitions and aliases
├── package.json        # npm configuration file
├── Cargo.toml          # Rust project configuration file
├── build.rs            # Build script for the project
├── .gitignore          # Git ignore file
└── README.md           # Project documentation
```

## Setup Instructions

1. **Clone the repository:**
   ```
   git clone <repository-url>
   cd rust-property
   ```

2. **Install Rust:**
   Make sure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

3. **Install Node.js:**
   Ensure you have Node.js installed. You can download it from [nodejs.org](https://nodejs.org/).

4. **Install dependencies:**
   Run the following command to install the necessary npm packages:
   ```
   npm install
   ```

5. **Build the project:**
   Use Cargo to build the Rust library:
   ```
   cargo build
   ```

## Usage

After building the project, you can use the API in your Node.js application. Import the compiled library and call the exposed functions to manage properties.

## API Documentation

- **get_property(property_id: PropertyId)**: Retrieves a property by its ID.
- **create_property(property_data: PropertyData)**: Creates a new property with the provided data.
- **update_property(property_id: PropertyId, property_data: PropertyData)**: Updates an existing property.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.