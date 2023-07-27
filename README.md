# txtastic

[![License](https://img.shields.io/github/license/cabralski/txtastic)](https://github.com/cabralski/txtastic/blob/master/LICENSE)

`txtastic` is a dead simple pastebin built using Rust, Rocket, and TailwindCSS!

### Features
- **Simple and Intuitive**: A straightforward and easy-to-use interface for sharing code snippets or text.
- **Easy Sharing**: Share your snippets quickly and effortlessly with others by simply sending them the generated link.

### Getting Started
Follow the instructions below to set up and run `txtastic` on your local machine.

### Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
   
   ```shell
   git clone https://github.com/cabralski/txtastic.git
   ```

2. Build the project:

   ```shell
   cd txtastic/
   cargo build --release
   ```

### Running the Application

1. Start the Rocket server:
   ```shell
   cargo run --release
   ```

2. Access the application in your browser at:
   [localhost:8000](http://localhost:8000)

### API

`txtastic` has only two endpoints:

```
GET `/api/txt/<id>` - Retrieves the content of the `txt`.
```

```
POST `/api/txt/`    - Creates a `txt` and redirects to the created URL.
```

### License

This project is licensed under the [Apache 2.0 License](https://github.com/cabralski/txtastic/blob/master/LICENSE).
