# Readme for Rust Web Scraper and Packet Sniffer

## Description
This Rust project includes web scraping functionalities and packet sniffing capabilities. It utilizes the `iced` framework for a graphical user interface, enabling users to input URLs, view scraped content, and monitor network traffic to capture domain names from DNS requests.

## Features
- **Web Scraping**: Fetch and parse HTML content from URLs.
- **GUI with Iced**: User-friendly interface for URL input and content viewing.
- **Packet Sniffing**: Captures domain names from network packets, focusing on DNS requests.
- **Domain List Viewing**: Toggle view for a list of captured domain names.
- **Asynchronous Programming**: Implements Rust's async/await for efficient network requests.
- **Environment Configuration**: Utilizes `.env` file for flexible network interface configuration.

## Installation
Ensure you have Rust and Cargo installed. Follow the installation instructions [here](https://www.rust-lang.org/tools/install) if necessary.

1. Clone the repository:
2. Navigate to the project directory:
3. Build the project:
   ```
   cargo build
   ```

## Usage
Before running the application, set the network interface in the `.env` file:

1. Create a `.env` file in the root directory.
2. Add the following line to renseign your network interface:
   ```
   NETWORK_INTERFACE=en0
   ```
3. Run the application:
   ```
   cargo run
   ```

The GUI allows for URL input, viewing scraped content, toggling the captured domain list, and refreshing the view.

## Dependencies
- `iced`: For the GUI.
- `reqwest`: For HTTP requests.
- `scraper`: For HTML parsing.
- `pnet`: For packet capturing.
- `tokio`: For the async runtime.
- `std::sync`: For thread-safe data sharing.
- `dotenv`: For loading environment variables from a `.env` file.

## Contributing
Contributions are welcome. Please follow the code of conduct and submit pull requests for new features or bug fixes.

## License
Licensed under [MIT License](LICENSE.md).

**Note**: This readme provides an overview. For detailed documentation, refer to the in-project code comments and docstrings.