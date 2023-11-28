# Rustarama National Football League Scraper

A Rust-based CLI tool for scraping football league data from a given URL.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Acknowledgements](#acknowledgements)

## Overview

Rustarama National Football League Scraper is a command-line tool developed in Rust that allows users to scrape 
and display football league data from a specified URL. It was built as a toy project to help learn Rust
and is not intended for production usage.


## Features

- **Data Scraping:** Extracts league data from the provided URL.
- **Structured Output:** Displays scraped data in a structured table format.
- **CLI Interface:** User-friendly command-line interface for easy interaction.

## Getting Started

To use Rustarama National League Scraper locally, follow the steps below.

### Prerequisites

Make sure you have Rust and Cargo installed.

```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

Clone the repository and build the project.

```bash
# Clone the repository
git clone https://github.com/CodeGareth-PublicCode/rustarama.git

# Navigate to the project directory
cd rustarama

# Build the project
cargo build
```

## Usage

Run the scraper with the league table URL as a command-line argument.

```bash
# Example usage
./rustarama.exe --league_table_url <very_specific_league_url>
```

## Acknowledgements

- [Reqwest](https://docs.rs/reqwest/) - HTTP client for Rust.
- [Scraper](https://docs.rs/scraper/) - HTML parsing library for Rust.
- [Structopt](https://docs.rs/structopt/) - Command-line argument parsing for Rust.
