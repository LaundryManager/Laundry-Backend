# LaundryAPI

This API are being created to handle laundry services. I will implement this as part of my Rust Backend learning journey.

Roadmap:
- [x] Reorganize the project structure
  - [x] Implement config module
    - [x] Added config files
    - [x] Added development config extraction
    - [x] Implement Data Factory to load config files at compile time, before app runs
      - [x] Add Secret in Data Factory Items
        - [x] JWT Login
        - [x] JWT Handler
  - [x] Implement handlers module
  - [x] Implement models module
  - [x] Implement routes module
  - [x] Implement utils module
  - [ ] Tests
    - [x] Integration tests * It will not be implemented for now
    - [x] Unit tests
      - [x] *? In which module should I implement unit tests?
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
