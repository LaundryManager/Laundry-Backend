# LaundryAPI

This API are being created to handle laundry services. I will implement this as part of my Rust Backend learning journey.

Roadmap:
- [ ] Reorganize the project structure
  - [ ] Implement config module
  - [ ] Implement handlers module
  - [ ] Implement models module
  - [ ] Implement routes module
  - [ ] Implement utils module
  - [ ] Implement schema module ? (I'm not sure if I will use this because seems redundant with models)
  - [ ] Tests
    - [ ] Integration tests
    - [ ] Unit tests
- [ ] Handlers
  - [ ] Implement token validation
- [ ] Routes
  - [ ] Implement user routes


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
