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
  - [x] Tests
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

Uso da lavanderia:
Cada usuário pode usar uma das lavadoras por 1:30 horas por dia, os horários são fixos, começando a partir das 7 da manhã:

- 7:00 - 8:30
- 8:30 - 10:00
- 10:00 - 11:30
- 11:30 - 13:00
- 13:00 - 14:30
- 14:30 - 16:00
- 16:00 - 17:30
- 17:30 - 19:00
- 19:00 - 20:30
- 20:30 - 22:00

Totalizando 10 horários disponíveis por dia. ( Se tiver mais de uma máquina o valor deve ser multiplicado pela quantidade de máquinas disponíveis. )
