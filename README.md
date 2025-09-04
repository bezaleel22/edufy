# LLA Web CMS

A modular REST API CMS built with Rust and Axum.

## Features

- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Blog Management**: Full CRUD operations for blog posts
- **File Upload**: Support for image and file uploads
- **Audit Logging**: Track user actions and system events
- **CORS Support**: Cross-origin resource sharing enabled
- **Modular Architecture**: Clean separation of concerns

## API Endpoints

### Authentication
- `POST /auth/login` - User login
- `POST /auth/logout` - User logout

### Blog Posts
- `GET /blog` - List all blog posts
- `POST /blog` - Create a new blog post
- `GET /blog/:id` - Get a specific blog post
- `PUT /blog/:id` - Update a blog post
- `DELETE /blog/:id` - Delete a blog post

### File Upload
- `POST /upload/image` - Upload an image
- `POST /upload/file` - Upload a file

## Default Credentials

- **Email**: admin@llaweb.com
- **Password**: admin123

## Running the Server

```bash
cd cms
cargo run
```

The server will start on `http://0.0.0.0:3001`

## Project Structure

```
cms/
├── src/
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Data models and DTOs
│   ├── handlers.rs      # HTTP request handlers
│   ├── auth.rs          # Authentication logic
│   ├── blog.rs          # Blog management logic
│   └── storage.rs       # File upload handling
├── Cargo.toml
└── README.md
```

## Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **jsonwebtoken**: JWT token handling
- **bcrypt**: Password hashing
- **uuid**: Unique identifier generation
- **chrono**: Date/time handling
- **tower-http**: HTTP middleware

## Architecture

The CMS is built with a modular architecture:

1. **Models**: Data structures and business logic
2. **Handlers**: HTTP request/response handling
3. **Auth**: Authentication and authorization
4. **Blog**: Blog post management
5. **Storage**: File upload and management

All modules communicate through a shared `AppState` that contains thread-safe data structures for users, sessions, blog posts, and audit logs.

## Security Features

- Password hashing with bcrypt
- JWT token-based authentication
- Session management
- Audit logging for security events
- CORS protection

## Development

To contribute to the project:

1. Clone the repository
2. Make changes to the relevant modules
3. Run `cargo build` to check for compilation errors
4. Run `cargo test` to run tests (when implemented)
5. Submit a pull request

## License

This project is licensed under the MIT License.
