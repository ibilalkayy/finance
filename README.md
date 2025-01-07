# Finance CLI Application

The Finance CLI Application is a command-line tool to help users manage their personal finances effectively. It supports tasks such as adding transactions, filtering data by category, viewing all transactions, and more, while storing data in a PostgreSQL database.

---

## Features

- **Add Transactions**: Insert new financial transactions into the database.
- **View Transactions**: Display all transactions stored in the database.
- **Filter Transactions**: Filter transactions by category.
- **Database Integration**: Persistent storage using PostgreSQL.
- **CLI Interaction**: User-friendly command-line interface with support for flags and options.

---

## Requirements

Ensure the following dependencies and packages are installed:

### Dependencies

- **Rust**: Programming language used to develop this application.
- **PostgreSQL**: Database system for storing financial data.

### Rust Crates

- `postgres`: PostgreSQL client for Rust.
- `dotenv`: Manage environment variables.
- `clap`: Parse command-line arguments and flags.

---

## Installation Guide

1. Clone the repository:
   ```bash
   git clone https://github.com/ibilalkayy/finance.git
   cd finance
   ```

2. Install Rust:
   Follow the instructions at [Rust's official website](https://www.rust-lang.org/tools/install).

3. Install dependencies:
   Add the required crates to your project:
   ```bash
   cargo add postgres dotenv clap
   ```

4. Set up the PostgreSQL database:
   - Ensure PostgreSQL is installed and running.
   - Create a database named `finance`:
     ```sql
     CREATE DATABASE finance;
     ```
   - Add the following environment variables to a `.env` file in the root directory:
     ```env
     DATABASE_URL=postgres://user:password@localhost:5432/my_database
     ```

5. Run the application:
   ```bash
   cargo run -- [commands and flags]
   ```

---

## Usage

### Example Commands

- **Add a Transaction**:
  ```bash
  cargo run -- add --description "Grocery shopping" --amount 150 --category "Food" --date "2024-12-25"
  ```

- **View All Transactions**:
  ```bash
  cargo run -- view
  ```

- **Filter Transactions by Category**:
  ```bash
  cargo run -- filter --category "Food"
  ```

---

## Contribution Guide

We welcome contributions! Follow these steps to contribute:

1. Fork the repository.
2. Clone your forked repository:
   ```bash
   git clone https://github.com/your-username/finance.git
   ```
3. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature-name
   ```
4. Make changes and commit them:
   ```bash
   git commit -m "Description of your changes"
   ```
5. Push to your fork:
   ```bash
   git push origin feature-name
   ```
6. Open a pull request in the main repository.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact

For questions or support, please open an issue in the repository.

