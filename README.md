# Rust Git Wrapper

This Rust Git Wrapper is a custom command-line tool designed to intelligently select the appropriate SSH key for different Git repositories. It's particularly useful for managing multiple Git accounts (like personal and work accounts) on the same machine.

## Features

- Dynamically selects the correct SSH key based on the current working directory.
- Easy configuration via a YAML file.
- Transparent replacement of the standard Git command.

## Installation

To use this Git wrapper, follow these steps:

### Prerequisites

- Rust and Cargo (the Rust package manager) must be installed on your machine. If you don't have them installed, you can get them from [the Rust website](https://www.rust-lang.org/tools/install).
- Ensure you have SSH keys set up for your different Git accounts.

### Setup

1. **Clone the Repository:**
   - Clone this repository to your local machine using:
     ```
     git clone <repository-url>
     ```

2. **Build the Project:**
   - Navigate to the project directory and build the project with Cargo:
     ```
     cd git_wrapper
     cargo build --release
     ```

3. **Configuration File:**
   - Create a `.git_ssh_ids` file in your home directory. This file should map your directories to their respective SSH keys. For example:
     ```yaml
     default: ~/.ssh/default_id_rsa.pub
     directories:
       - path: ~/code/work
         key: ~/.ssh/work_id_rsa.pub
       - path: ~/code/personal
         key: ~/.ssh/personal_id_rsa.pub
     ```

4. **Replace the Default Git Command:**
   - Create an alias in your shell configuration (like `.bashrc` or `.zshrc`):
     ```
     alias git="path/to/git_wrapper/target/release/git_wrapper"
     ```

### Usage

Once installed, you can use the `git` command as you normally would. The wrapper will automatically select the correct SSH key based on your current directory.

## Contributing

Contributions to this project are welcome! Feel free to fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
