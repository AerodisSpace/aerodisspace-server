# AerodisSpace Server Workspace

## Overview

This workspace contains the core API and internal utility crates essential for the development and operation of our AerodisSpace streaming platform dedicated to model aviation. Here, you'll find everything needed to set up, run, and contribute to the server that powers our real-time streaming experience.

## Includes

- **API**: The application programming interface that manages communication between the platform's clients and the database, as well as integrating real-time sensor data.
- **Utility Crates (Utils)**: A set of internal libraries developed to support API operations, including data manipulation, hardware communication, and performance optimizations.

## Dependencies

To work on this workspace, you'll need to install some essential tools and libraries:

### Charybdis-Migrate CLI-Tool

We use `charybdis-migrate` for managing database migrations. Install it with the following command:

```bash
cargo install charybdis-migrate
```

### Caution
- **Automatic Migrations**: The build command is configured to perform migrations automatically with each compilation. This helps keep the database schema in sync with the current version of the code.

### Documentation
For complete details on the API and LIVE, and how to use the utility crates, please consult our documentation. The documentation is available at the following link: 
<a href="https://docs.aerodis.space">AerodisSpace Server Documentation</a>.

### How to Contribute
We are always looking for improvements and value the community's contribution. If you are interested in contributing to the AerodisSpace Server Workspace, see below for how to get started:

1. Clone the Repository: Fork and clone the repository to start working locally on your improvements.

2. Install Dependencies: Make sure you have all necessary dependencies installed.

3. Make Your Changes: Implement your improvements or fixes.

4. Test Locally: Use the available tests to ensure your contribution does not break any existing functionality.

5. Open a Pull Request: Submit your changes for review.
For more information on how to contribute, please see our contribution guide in the repository.