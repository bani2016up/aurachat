
# Dashboard - DDD Application

## Architecture

This application follows Domain-Driven Design (DDD) principles with the following layers:

- **Domain Layer**: Core business logic, entities, and repository interfaces
- **Application Layer**: Use cases, DTOs, and application services
- **Infrastructure Layer**: Database implementation (SeaORM), messaging (RabbitMQ)
- **Presentation Layer**: REST API endpoints (Rocket)

## Prerequisites

- Rust 1.70+
- PostgreSQL
- RabbitMQ

## Setup

1. Install dependencies:
```bash
cargo build
```
