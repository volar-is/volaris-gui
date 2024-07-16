# Volaris GUI

![GitHub License](https://img.shields.io/github/license/volar-is/volaris-gui) ![GitHub Issues](https://img.shields.io/github/issues/volar-is/volaris-gui) ![GitHub Stars](https://img.shields.io/github/stars/volar-is/volaris-gui)

## Introduction

Volaris GUI is the graphical user interface for the Volaris encryption tool. Built using Svelte and Tauri, it provides an intuitive and user-friendly experience for securing your data across multiple platforms, including desktop and *hopefully* mobile devices.

## Features

- **User-Friendly Interface**: An intuitive and responsive design powered by Svelte.
- **Cross-Platform Support**: Available on Windows, macOS, Linux.
- **Rust-Based Backend**: Leveraging the Volaris Rust backend for robust security and performance.
- **Modern Encryption Standards**: Utilizes secure encryption algorithms to ensure data security.

## Security

Volaris GUI uses the same modern cryptographic methods as the core Volaris tool (XChaCha20-Poly1305 + AES-256-GCM), with audited backends to keep your data secure. It offers an effortless way to encrypt your files before sharing them, ensuring they remain safe in transit.

## Current Status

Volaris GUI is currently in development. We are committed to delivering a secure and user-friendly encryption tool. Stay tuned for updates and releases.

## Installation

### Prerequisites

- Ensure you have [Node.js](https://nodejs.org/) installed.
- Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.

### Building from Source

1. Clone the repository:
   `git clone https://github.com/volar-is/volaris-gui.git`
2. Change to the project directory:
   `cd volaris-gui`
3. Install dependencies:
   `pnpm install`
4. Build the project:
   `pnpm run build`

## Testing

Once built, you can run the application:
`pnpm run dev`

## Contributing

We welcome contributions from the community. To contribute:

1. Fork the repository.
2. Create a new branch:
   `git checkout -b mame/YourFeature`
3. Make your changes and commit them:
   `git commit -m "feat: Add your feature"`
4. Push to the branch:
   `git push origin name/YourFeature`
5. Open a pull request.

Please ensure your code adheres to our coding standards and includes appropriate tests.

**Note:** This repository uses conventional commits. Please follow the [conventional commits guidelines](https://www.conventionalcommits.org/en/v1.0.0/) for your commit messages.

## License

This project is licensed under the BSD 2-Clause License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or support, please open an issue on GitHub.
