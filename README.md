# Basher Backend

Welcome to the backend of **Basher**, an anonymous forum-based web application. Moving forward, this backend is responsible for server-side logic as well as user authentication and real-time services for the said platform.

## Technologies Used

- **[Axum](https://github.com/tokio-rs/axum):** A lightweight, ergonomic web framework for Rust used to build the core backend API.
- **[GraphQL](https://graphql.org):** It is an API query language that performs validation and data retrieval efficiently.
- **[SSE (Server-Sent Events)](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events):** This allows for constant updates, which establishes an engaging interaction between the server to the clients.
- **[JWT (JSON Web Tokens)](https://jwt.io):** This ensures that the user authentication process is secure and the user session is managed in a stateless manner.
- **[SurrealDB](https://surrealdb.com/):** This is a multi-model database, all-in-one system, which provides the functions of database management, queries, and application programming interface.
- **[Shuttle](https://www.shuttle.dev/):** This is a cloud-oriented development service for Rust applications with a modern approach to application hosting and maintenance.

## Core Elements

- **Anonymous Posting:** Users are identified by numbers, starting at `0` (the Original Poster) for each topic. This label persists across replies within the same topic.
- **Real-time Updates:** Through SSE, the server automatically gives updates concerning new replies as well as topics.
- **GraphQL Integration:** The overall structure has elements that ensure it is easier to query and retrieve information.
- **Secure Authentication:** With the implementation of JWT, it consults user sessions to be secure and stateless.

## Introduction

### Requirements

Make sure you have the following items installed:

- **[Rust](https://www.rust-lang.org/tools/install):** A programming language that is system based and is regarded highly due to its security, speed, and ability to do many tasks at the same time.
- **[Cargo](https://doc.rust-lang.org/cargo/):** The command line interface for Rust which is able to act as both a build system and a package manager.
- **[SurrealDB](https://surrealdb.com/):** A multi model database system that encompasses database, queries, application programming interface and authentication functionalities into one system.
- **[Shuttle](https://www.shuttle.dev/):** A cloud service for deploying applications developed in Rust.

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/DestinEcarma/basher-backend
   ```

2. Set up the database:

   1. Navigate to the `database` directory:

      ```sh
      cd basher-backend/database
      ```

   2. Start the database server:

      ```sh
      ./start_dev.sh
      ```

   3. Initialize the schema (only needs to be done once):
      ```sh
      ./init_schema.sh
      ```

3. In a new terminal, navigate to the cloned repository and start the server:
   ```sh
   shuttle run
   ```

## License

This project is licensed under the [MIT License](LICENSE). See the LICENSE file for more details.
