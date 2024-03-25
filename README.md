# File Management CLI App

A file management CLI app built using Rust, with the following functionality:

* Copying files
* Deleting files
* Moving files
* Renaming files
* Listing the properties of a file

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

* Rust (>= 1.45)
* Cargo (Rust package manager)

### Installing

1. Clone the repository:
```
git clone https://github.com/[username]/file-management-cli.git
```
2. Change to the project directory:
```
cd file-management-cli
```
3. Build the project:
```
cargo build --release
```
4. Run the project:
```
./target/release/file_management_cli
```

## Using the App

### Commands

#### Copying Files

```
file_management_cli copy <SOURCE_PATH> <DESTINATION_PATH> [FILE_NAME]
```

#### Deleting Files

```
file_management_cli delete <FILE_PATH>
```

#### Moving Files

```
file_management_cli move <SOURCE_PATH> <DESTINATION_PATH>
```

#### Renaming Files

```
file_management_cli rename <OLD_NAME> <NEW_NAME>
```

#### Listing File Properties

```
file_management_cli properties <FILE>
```

### Options

* `-h`, `--help`: Print help information
* `-V`, `--version`: Print version information

## Roadmap

* Creating directories
* Deleting directories
* Listing files in a directory
* Listing sub-directories of a directory

### Creating Directories

* Implement a command for creating directories
* Allow creating multiple directories at once
* Validate directory creation

### Deleting Directories

* Implement a command for deleting directories
* Allow deleting multiple directories at once
* Validate directory deletion

### Listing Files in a Directory

* Implement a command for listing files in a directory
* Allow listing files recursively
* Allow filtering by file type

### Listing Sub-Directories of a Directory

* Implement a command for listing sub-directories of a directory
* Allow listing sub-directories recursively
* Allow filtering by directory depth

## Built With

* [clap](https://github.com/clap-rs/clap) - Command Line Argument Parser.
* [walkdir](https://github.com/BurntSushi/walkdir) - File tree traversal.
* [file-format](https://github.com/servo/file-format) - File format parsing.

## Acknowledgments

* Hat tip to the creators of Clap, Walkdir, and File-format.

## Disclaimer

This README is for educational purposes only. The author is not responsible for any misuse of the information contained in this README.