# Volaris GUI

Volaris GUI is a sleek and modern Svelte-based application designed to streamline the process of file encryption and decryption using key files. This tool offers a user-friendly interface with drag-and-drop support, tabbed navigation, and seamless key file management.

## Features

- **File Encryption & Decryption**: Easily encrypt or decrypt files with just a few clicks.
- **Key File Management**: Create new key files or select existing ones directly from the app.
- **Drag-and-Drop Support**: Quickly select files by dragging them into the designated area.
- **Modern UI**: A clean and responsive design using Svelte and Tailwind CSS.

## Screenshots

N/A

## Getting Started

### Prerequisites

To run the project, you'll need to have the following installed:

- [Node.js](https://nodejs.org/) (v16.x or later)
- [Pnpm](https://pnpm.io)
- [Tauri](https://tauri.app/)

### Installation

1. **Clone the repository**:

   `git clone https://github.com/volar-is/volaris-gui.git`
   `cd volaris-gui`

2. **Install dependencies**:

   `pnpm install`

3. **Run the application**:

   - **Development Mode**:

     `pnpm tauri dev`

   - **Build for Production**:

     `pnpm tauri build`

### Usage

1. **File Encryption**:

   - Navigate to the "File" tab.
   - Drag and drop your file into the drop zone, or click to select a file.
   - Choose to "Encrypt File" to secure your data.

2. **File Decryption**:

   - Navigate to the "File" tab.
   - Drag and drop your encrypted file into the drop zone, or click to select a file.
   - Choose to "Decrypt File" to unlock your data.

3. **Key File Management**:

   - Navigate to the "Key File" tab.
   - Create a new key file by clicking "Create New Key File" or select an existing key file.

### Contributing

We welcome contributions from the community. To contribute:

1. Fork the repository.
2. Create a new branch:
   `git checkout -b feature/YourFeature`
3. Make your changes and commit them:
   `git commit -m "feat: Add your feature"`
4. Push to the branch:
   `git push origin feature/YourFeature`
5. Open a pull request.

Please ensure your code adheres to our coding standards and includes appropriate tests.

**Note:** This repository uses conventional commits. Please follow the [conventional commits guidelines](https://www.conventionalcommits.org/en/v1.0.0/) for your commit messages.

### License

This project is licensed under the BSD 2-Clause License License. See the [LICENSE](/LICENSE) file for details.

### Acknowledgments

- [Svelte](https://svelte.dev/) - The JavaScript framework used for building the UI.
- [Tauri](https://tauri.app/) - Framework for building tiny, fast binaries for all major desktop platforms.
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework for styling.
