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

🔐 Role-Based Access Control (RBAC) – Access Management

This project implements a Role-Based Access Control (RBAC) system using:

🦀 Rust

⚡ Actix Web

🍃 MongoDB

🔑 JWT Authentication

The system supports Admin Users and Web Users with dynamic permission-based access control.

📌 Table of Contents

Architecture Overview

Role Structure

JWT Structure

Permission Flow

Access Control Logic

Web User Access Management

Example Protected Route

Database Schema

Security Notes

🏗️ Architecture Overview

The system follows Role-Based Access Control (RBAC) principles:

User logs in

Role is fetched from database

Permissions are extracted

JWT is generated with permissions

Each API checks required permission before execution

👥 User Types
Type	Description
super_admin	Full access to all resources
admin	Access based on assigned permissions
web_user	Public-facing user with restricted access
🗄️ Roles Collection (MongoDB)

Collection: roles

Example Role Document
{
  "_id": ObjectId("65fa380a8d974209406ce5d3c"),
  "type": "admin",
  "permissions": [
    "user:create",
    "user:update",
    "user:delete",
    "project:create",
    "project:update",
    "project:delete"
  ]
}
Fields

type → Defines the role type

permissions → List of allowed actions

🔑 JWT Claims Structure

After successful login, JWT contains:

Claims {
    sub: "admin",
    exp: 1772711162,
    admin_type: "super_admin",
    user_id: "69a380a8d974209406ce5d3c",
    permissions: [
        "user:create",
        "project:create"
    ]
}
Why Store Permissions in JWT?

No database call on every request

Faster authorization

Stateless authentication

🔄 Permission Flow
Step 1: Login

Validate credentials

Fetch role from roles collection

Extract permissions

Generate JWT including permissions

Step 2: Middleware Authentication

Custom extractor:

AuthenticatedUser

This:

Validates JWT

Extracts claims

Attaches user info to request

Step 3: Permission Check
pub async fn check_permission(
    user: &AuthenticatedUser,
    required_permission: &str,
) -> bool {
    user.permissions.contains(&required_permission.to_string())
}
🔐 Protecting Routes
Example: Create Project (Admin Only)
#[post("/projects")]
pub async fn create_project(
    admin: AuthenticatedUser,
    db: web::Data<mongodb::Database>,
    payload: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, MyError> {

    if !check_permission(&admin, "project:create").await {
        return Err(MyError::AuthError("Permission denied".to_string()));
    }

    // Continue logic...
}
🌐 Managing Access for Web Users

For public website users:

Option 1: Allow Public Read Access

Do not require AuthenticatedUser extractor.

#[get("/projects")]
pub async fn get_projects(db: web::Data<mongodb::Database>) {
    // Public access
}
Option 2: Restricted Web User Permissions

Create a role:

{
  "type": "web_user",
  "permissions": [
    "project:view"
  ]
}

Then check permission normally using check_permission().

🗂️ Database Schema Overview
Admin Collection
{
  "_id": ObjectId,
  "username": "admin",
  "password": "hashed_password",
  "role_id": ObjectId
}
Roles Collection
{
  "_id": ObjectId,
  "type": "admin",
  "permissions": []
}
Projects Collection
{
  "_id": ObjectId,
  "title": "Portfolio Website",
  "description": "Built with Rust and React",
  "tech_stack": ["Rust", "MongoDB", "React"],
  "image_url": "https://...",
  "live_link": "https://...",
  "repo_link": "https://...",
  "created_at": 1700000000000,
  "updated_at": 1700000000000
}
🛡️ Security Best Practices

Always hash passwords (bcrypt or argon2)

Never trust frontend permissions

Always verify permissions in backend

Use short JWT expiry

Store JWT secret securely (env variable)

Validate ObjectId properly before DB queries

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
