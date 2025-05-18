# nodexapigen

**`nodexapigen`** is a fast CLI tool written in Rust that scaffolds modular **Node.js REST API servers**. For building a API starter CRUD setup.  
`nodexapigen` will saves time and get you nice project folder setup as starting position.

---

## What It Does

`nodexapigen` automates the creation of a structured Node.js API project. It generates folders, boilerplate files, routing logic and good code seperation with help of flags or interactive input.

---

## Features (Planned)

- [ ] Create a project folder with recommended **folder structure**
- [ ] Generate **modular routers**, **controllers**, and **handlers** with CRUD capabilities
  - [ ] Create
  - [ ] Read
  - [ ] Update
  - [ ] Delete
  - [ ] ReadAll
- [ ] Optional setup for:
  - [ ] Authentication (`--with-auth`)
    - [ ] Approved by mail verification
    - [ ] Approved by admin
  - [ ] CORS & logging middleware
- [ ] CLI flags + interactive mode
- [ ] Customizable naming for routes/entities
- [ ] Auto-create `.env`, `package.json`, `README.md`

---

## Target Folder Structure
```
api-server/
├── src/
│ ├── router/
│ │ └── user.routes.js
│ ├── controllers/
│ │ └── user.controller.js
│ ├── services/
│ │ └── user.service.js
│ ├── handlers/
│ │ └── errorHandler.js
│ └── app.js
├── .env
├── package.json
└── README.md
```

---

## Implementation Tasks

### Core CLI Functionality
- [ ] Parse command-line arguments
- [ ] Prompt for project name if none is given
- [ ] Create root project directory
- [ ] Scaffold subfolders: `src`, `router`, `controllers`, `handlers`
- [ ] Generate starter files (`server.js`, `.env`)

### Router
- [ ] Generate `router/` files based on CLI args
- [ ] Wire routes to Express `Router` in `router.js`

### Controller
- [ ] Create controller files with CRUD stubs
- [ ] Create service files with placeholder logic

### Middleware & Utility Support
- [ ] Add middleware folder (e.g., `auth.js`, `logger.js`)
- [ ] Add centralized error handler

### Scripts
- [ ] Add scripts folder (e.g., `seed-data.js`, `prune-data.js`)

### Optional Features
- [ ] Add `--with-auth` flag (JWT + login route stub)
- [ ] Add `--with-mailer` 
- [ ] Add `--with-mailer` 

---

## Example Usage

```bash
nodexapigen create my-api --with-auth --with-mailer
```
---

## Built With
- Rust (CLI implementation)
- clap (CLI argument parsing)

---

## License
MIT © 2025
