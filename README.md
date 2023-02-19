# LaundryAPI

This API are being created to handle laundry services. I will implement this as part of my Rust Backend learning journey.

Roadmap:
- [ ] Reorganize the project structure
  - [ ] Implement config module
    - [x] Added config files
    - [x] Added development config extraction
    - [ ] Implement Data Factory to load config files at compile time, before app runs
      - [ ] Add Secret in Data Factory Items
        - [ ] JWT Login
        - [ ] JWT Handler
  - [x] Implement handlers module
  - [x] Implement models module
  - [x] Implement routes module
  - [x] Implement utils module
  - [ ] Tests
    - [ ] Integration tests
    - [ ] Unit tests
- [x] Handlers
  - [x] Implement token validation
- [x] Routes
  - [x] Implement user routes


## Next Project Structure

```├── Cargo.toml
├── src
│   ├── main.rs
│   ├── config
│   │   ├── development.toml
│   │   ├── production.toml
│   │   ├── staging.toml
│   │   └── config.rs
│   ├── handlers
│   │   ├── user.rs
│   │   └── ...
│   ├── models
│   │   ├── user.rs
│   │   └── ...
│   ├── routes
│   │   ├── user.rs
│   │   └── ...
│   ├── schema *?
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── ...
│   └── utils
│       ├── auth.rs
│       ├── database.rs
│       └── ...
└── tests
    ├── integration
    │   ├── user_test.rs
    │   └── ...
    └── unit
        ├── auth_test.rs
        └── ...
```
