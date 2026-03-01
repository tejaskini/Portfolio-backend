# Portfolio Backend API

A robust Rust-based REST API backend for managing portfolio content (projects, experience, education, skills) with JWT authentication and MongoDB integration.

## 🚀 Tech Stack

- **Language:** Rust (2021 edition)
- **Web Framework:** Actix-web 4.x
- **Database:** MongoDB 2.x
- **Authentication:** JWT (JSON Web Tokens)
- **Password Encoding:** Base64 (reversible for development)
- **Additional Libraries:**
  - `serde` & `serde_json` - Data serialization
  - `bcrypt` - Password hashing (removed, using base64 for dev)
  - `jsonwebtoken` - JWT creation and validation
  - `chrono` - Timestamp handling
  - `uuid` - Unique identifiers
  - `dotenv` - Environment variable management

## 📋 Prerequisites

- Rust 1.70+ ([Install Rust](https://www.rust-lang.org/tools/install))
- MongoDB 4.0+ (Local or Cloud)
- Cargo (comes with Rust)

## 🔧 Installation & Setup

### 1. Clone the repository
```bash
git clone <repository-url>
cd portfolio-backend
```

### 2. Create `.env` file
```env
MONGO_URI=mongodb://localhost:27017
PORT=8080
JWT_SECRET=your_super_secret_key_here
```

### 3. Build the project
```bash
cargo build
```

### 4. Run the server
```bash
cargo run
```

Server will start on `http://localhost:8080`

## 📚 API Endpoints

### Authentication Routes

#### Register
```
POST /api/v1/register
Content-Type: application/json

{
  "username": "admin",
  "email": "admin@example.com",
  "phone": "+1234567890",
  "password": "your_password"
}

Response: 201 Created
{
  "message": "Admin registered successfully",
  "username": "admin"
}
```

#### Login
```
POST /api/v1/login
Content-Type: application/json

{
  "username": "admin",
  "password": "your_password"
}

Response: 200 OK
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### Verify Reset
```
POST /api/v1/verify-reset
Content-Type: application/json

{
  "email_or_phone": "admin@example.com"
}

Response: 200 OK
{
  "message": "User found",
  "username": "admin"
}
```

#### Reset Password
```
POST /api/v1/reset-password
Content-Type: application/json

{
  "email_or_phone": "admin@example.com",
  "new_password": "new_password123"
}

Response: 200 OK
{
  "message": "Password reset successful"
}
```

### Projects Routes

#### Create Project (🔒 Requires Token)
```
POST /api/v1/projects
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "My Project",
  "description": "Project description",
  "tech_stack": ["Rust", "Actix-web", "MongoDB"],
  "image_url": "https://example.com/image.jpg",
  "live_link": "https://example.com",
  "repo_link": "https://github.com/user/repo"
}

Response: 201 Created
```

#### Get All Projects
```
GET /api/v1/projects

Response: 200 OK
{
  "message": "Projects retrieved",
  "data": [...]
}
```

#### Update Project (🔒 Requires Token)
```
PUT /api/v1/projects/{id}
Authorization: Bearer <token>
Content-Type: application/json

{...project data...}

Response: 200 OK
```

#### Delete Project (🔒 Requires Token)
```
DELETE /api/v1/projects/{id}
Authorization: Bearer <token>

Response: 200 OK
```

### Experience Routes

#### Create Experience (🔒 Requires Token)
```
POST /api/v1/experience
Authorization: Bearer <token>
Content-Type: application/json

{
  "company": "Tech Company",
  "role": "Senior Developer",
  "description": "Worked on amazing projects",
  "start_date": "2020-01-15",
  "end_date": "2023-12-31",
  "is_current": false
}
```

#### Get All Experience
```
GET /api/v1/experience
```

#### Update Experience (🔒 Requires Token)
```
PUT /api/v1/experience/{id}
Authorization: Bearer <token>
```

#### Delete Experience (🔒 Requires Token)
```
DELETE /api/v1/experience/{id}
Authorization: Bearer <token>
```

### Education Routes

#### Create Education (🔒 Requires Token)
```
POST /api/v1/education
Authorization: Bearer <token>
Content-Type: application/json

{
  "institution": "MIT",
  "degree": "Bachelor",
  "field_of_study": "Computer Science",
  "start_year": 2018,
  "end_year": 2022
}
```

#### Get All Education
```
GET /api/v1/education
```

#### Update Education (🔒 Requires Token)
```
PUT /api/v1/education/{id}
Authorization: Bearer <token>
```

#### Delete Education (🔒 Requires Token)
```
DELETE /api/v1/education/{id}
Authorization: Bearer <token>
```

### Skills Routes

#### Create Skill (🔒 Requires Token)
```
POST /api/v1/skills
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Rust",
  "proficiency": "Advanced",
  "category": "Programming Languages"
}
```

#### Get All Skills
```
GET /api/v1/skills
```

#### Update Skill (🔒 Requires Token)
```
PUT /api/v1/skills/{id}
Authorization: Bearer <token>
```

#### Delete Skill (🔒 Requires Token)
```
DELETE /api/v1/skills/{id}
Authorization: Bearer <token>
```

### Password Utility Endpoints (Development Only)

#### Encode Password
```
POST /dev/encode-password
Content-Type: application/json

{
  "password": "myPassword123"
}

Response: 200 OK
{
  "original_password": "myPassword123",
  "encoded_password": "bXlQYXNzd29yZDEyMw=="
}
```

#### Decode Password
```
POST /dev/decode-password
Content-Type: application/json

{
  "encoded_password": "bXlQYXNzd29yZDEyMw=="
}

Response: 200 OK
{
  "encoded_password": "bXlQYXNzd29yZDEyMw==",
  "decoded_password": "myPassword123"
}
```

## 🔐 Authentication

### Token Expiration
- **Duration:** 24 hours
- **Type:** JWT (JSON Web Token)
- **Algorithm:** HS256

### How to Use Token

1. **Login to get token:**
   ```
   POST /api/v1/login
   ```

2. **Use token in Authorization header:**
   ```
   Authorization: Bearer <your_token>
   ```

3. **Token expires after 24 hours** → Need to login again

### What Requires Token
- ✅ All `POST` requests (Create operations)
- ✅ All `PUT` requests (Update operations)
- ✅ All `DELETE` requests (Delete operations)
- ❌ `GET` requests (Public read access)

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point & route configuration
├── config/
│   ├── db.rs           # MongoDB connection setup
│   └── mod.rs
├── models/
│   ├── admin.rs        # Admin user & authentication models
│   ├── project.rs      # Project model
│   ├── experience.rs   # Experience model
│   ├── education.rs    # Education model
│   ├── skill.rs        # Skill model
│   └── mod.rs
├── routes/
│   ├── auth.rs         # Authentication endpoints
│   ├── projects.rs     # Project CRUD endpoints
│   ├── experience.rs   # Experience CRUD endpoints
│   ├── education.rs    # Education CRUD endpoints
│   ├── skills.rs       # Skills CRUD endpoints
│   ├── password_utils.rs # Password encoding/decoding
│   └── mod.rs
├── middleware/
│   ├── auth_middleware.rs # JWT token validation
│   └── mod.rs
├── utils/
│   ├── jwt.rs          # JWT token creation
│   ├── response.rs     # API response formatting
│   └── mod.rs
├── error.rs            # Error handling & types
└── Cargo.toml          # Project dependencies
```

## 🔄 Workflow Example

### 1. Register New Admin
```bash
curl -X POST http://localhost:8080/api/v1/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_dev",
    "email": "john@example.com",
    "phone": "+1234567890",
    "password": "secure_password"
  }'
```

### 2. Login to Get Token
```bash
curl -X POST http://localhost:8080/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_dev",
    "password": "secure_password"
  }'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJqb2huX2RldiIsImV4cCI6MTcwOTI5MDAwMH0.xxx"
}
```

### 3. Use Token to Create Project
```bash
curl -X POST http://localhost:8080/api/v1/projects \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "title": "My Rust API",
    "description": "A REST API built with Actix-web",
    "tech_stack": ["Rust", "Actix-web", "MongoDB"],
    "image_url": "https://example.com/image.jpg",
    "live_link": "https://api.example.com",
    "repo_link": "https://github.com/user/repo"
  }'
```

## 🛠️ Development Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

### Run with Release Optimization
```bash
cargo build --release
./target/release/portfolio-backend
```

### Run Tests
```bash
cargo test
```

### Check for Issues
```bash
cargo check
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

## 🚨 Error Responses

### 400 Bad Request
```json
{
  "message": "Invalid request body"
}
```

### 401 Unauthorized (Missing Token)
```json
{
  "message": "No token provided"
}
```

### 401 Unauthorized (Invalid Token)
```json
{
  "message": "Invalid token"
}
```

### 404 Not Found
```json
{
  "message": "Resource not found"
}
```

### 409 Conflict
```json
{
  "message": "Username already exists"
}
```

### 500 Internal Server Error
```json
{
  "message": "Internal server error"
}
```

## 🔐 Security Notes

⚠️ **Important for Production:**
- Change `JWT_SECRET` to a strong, random string
- Use HTTPS instead of HTTP
- Implement rate limiting
- Add CORS configuration for your frontend domain
- Use environment variables for sensitive data
- Add input validation and sanitization
- Implement logging and monitoring
- Regular security audits

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📧 Contact

For questions or support, please reach out to the project maintainer.

---

**Last Updated:** March 1, 2026
**Status:** Production Ready
