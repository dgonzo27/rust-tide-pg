# Rust + Tide + PostgreSQL

[![Rust](https://img.shields.io/badge/Rust-v1.67.1-blue?logo=rust&logoColor=000000)](https://www.rust-lang.org) [![Tide](https://img.shields.io/badge/Tide-v0.16.0-blue?logo=tide&logoColor=4050FB)](https://github.com/http-rs/tide) [![Docker](https://img.shields.io/badge/Docker-latest-blue?logo=docker&logoColor=blue)](https://www.docker.com/products/docker-desktop/) [![Pre-Commit](https://img.shields.io/badge/Hooks-Pre--Commit-blue?logo=pre-commit&logoColor=FAB040)](https://pre-commit.com) [![Bash](https://img.shields.io/badge/GNU-Bash-blue?logo=gnu-bash&logoColor=4EAA25)](https://www.gnu.org/software/bash/) [![GitHub_Actions](https://img.shields.io/badge/CI/CD-GitHub_Actions-blue?logo=githubactions&logoColor=2088FF)](https://docs.github.com/en/actions)

This repository contains a Rust (Tide) API with a containerized PostgreSQL database. Additionally, the repository contains a Rust test suite for integration testing API routes as it relates to CRUD operations and transactions against the containerized database.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Local Development Guide](#local-development-guide)
  - [Other Commands](#other-commands)
- [Need Support?](#need-support)
- [Reporting Security Vulnerabilities and Security Bugs](#reporting-security-vulnerabilities-and-security-bugs)
- [Contributing](#contributing)

## Getting Started

This section provides supporting steps and documentation for developing locally.

### Prerequisites

Before jumping into the code, there are a few prerequisites.

1. Local development should be done from a UNIX-based machine - use Linux, MacOS, or WSL2 if you're on a Windows machine.

2. GitHub access should be managed through an SSH key in your UNIX environment. If you're unfamiliar with this process [start here](https://docs.github.com/en/authentication/connecting-to-github-with-ssh).

3. [Docker Desktop](https://www.docker.com/products/docker-desktop/) should be installed on your machine for local development. If you're on Windows, configure your settings to enable Docker Desktop in WSL2.

4. [pre-commit](https://pre-commit.com/) should be installed globally on your machine for linting and validating your code prior to pushing up to GitHub.

5. [Rust](https://www.rust-lang.org/tools/install) should be installed globally on your machine for compiling and running code.

6. Optionally, the [Rust-Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension can be installed in VS Code for a better development experience.

### Local Development Guide

This project uses a variation of [scripts to rule them all](https://github.com/github/scripts-to-rule-them-all).

1. Clone the repository (if this is your first time).

   ```sh
   git clone git@github.com:dgonzo27/rust-tide-pg.git
   ```

2. Navigate into the repository directory.

   ```sh
   cd rust-tide-pg
   ```

3. Ensure pre-commit is enabled for this repository.

   ```sh
   pre-commit install
   ```

4. You might need to give yourself access to the scripts for running this project.

   ```sh
   chmod +rwx scripts/clean
   chmod +rwx scripts/compile
   chmod +rwx scripts/database
   chmod +rwx scripts/format
   chmod +rwx scripts/server
   chmod +rwx scripts/setup
   chmod +rwx scripts/stop
   chmod +rwx scripts/test
   ```

5. Run the project.

   ```sh
   scripts/setup
   ```

6. From a new terminal tab or an API client of your choice, test some API routes.

   ```sh
   curl -v -d '{"title":"test", "description": "test"}' http://localhost:8080/movies
   curl -g http://localhost:8080/movies
   curl -g http://localhost:8080/movies/1
   curl -X PUT -d '{"id": 1, "title": "new title", "description": "new desc"}' http://localhost:8080/movies/1
   curl -X DELETE http://localhost:8080/movies/1
   ```

7. Stop the Rust server from your original terminal tab.

   ```sh
   ctrl + c
   ```

8. Run the test suite.

   ```sh
   scripts/test
   ```

9. Stop the development environment.

   ```sh
   scripts/stop
   ```

### Other Commands

1. Format and validate all files.

   ```sh
   scripts/format
   ```

2. Clean your database, removing any existing volumes and images (data).

   ```sh
   scripts/clean
   ```

## Need Support?

File an issue via [GitHub Issues](https://github.com/dgonzo27/rust-tide-pg/issues).

## Reporting Security Vulnerabilities and Security Bugs

Security vulnerabilities and bugs should be reported privately, via email, to the maintainers of this repository. Please contact [Dylan Gonzales](mailto:dylangonzales247@gmail.com). For more information, visit the [security guidelines](./SECURITY.md).

## Contributing

Before contributing to this repository, please review the [code of conduct](./CODE_OF_CONDUCT.md).

Contributions and suggestions are welcomed. However, there is a level of responsibility placed on the contributor to follow best-practices, provide thorough testing, follow the branching strategy, use the pull request template, and maintain a positive and coachable attitude when receiving feedback or questions on your code. For more details on these responsibilities, please visit the [contributing guide](./CONTRIBUTING.md).

When contributing, you are granting the maintainers of this repository the rights to use your contribution(s).
