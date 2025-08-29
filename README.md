# Aion Idle RPG Backend

## Overview
The Aion Server is a Rust-based backend for an idle RPG game built using the Actix-Web framework. It implements a sophisticated event-driven architecture for managing hero actions, combat, and resource management in an asynchronous idle game environment.

## Quick Start

### Prerequisites
- Docker
- Rust toolchain
- PostgreSQL client
- cargo-watch (optional, for development)

### Startup Instructions

1. Start Docker services:
```bash
docker compose up -d postgres
```

2. Set up the environment and database:
```bash
export APP_ENVIRONMENT=local
cargo prisma migrate dev

# Seed the database
cd seed/base
for sql_file in ./*.sql; do
    echo "Executing $sql_file"
    psql "postgresql://root:root@localhost:5432/defaultdb" -a -f "$sql_file"
done
cd ../..
```

3. Run the server:
```bash
# For development with auto-reload
cargo-watch -w src -x run

# Or just run normally
cargo run
```

4. To shut down:
```bash
docker ps
# get the container id
docker stop <container_id>
```

## Architecture

### Core Technology Stack
- **Language**: Rust
- **Web Framework**: Actix-Web 4
- **Database**: PostgreSQL with Prisma ORM
- **Session Storage**: Redis
- **Real-time Communication**: WebSockets (using tokio-tungstenite and actix-web-actors)
- **Authentication**: JWT tokens with wallet signature verification
- **Async Runtime**: Tokio

### Architecture Components

#### 1. **Application Entry Point**
- `main.rs` initializes the application with configuration loading
- `webserver.rs` sets up the HTTP server, middleware, and API routes
- Supports both local and production environments with different configurations

#### 2. **Configuration System**
- Hierarchical configuration using YAML files (`base.yaml`, `local.yaml`)
- Environment variable support for cloud deployments
- Manages database connections, Redis URIs, and application settings
- Dynamic duration configurations for different game actions

#### 3. **Database Layer (Prisma)**
The schema defines the core game entities:
- **Account**: Player accounts linked to Supabase authentication
- **Hero**: Player characters with stats, resources, and relationships
- **Actions**: Various game actions (explore, channel, quest, raid)
- **Combat**: Cards, decks, effects, and NPCs
- **Resources**: Game currencies (Aion, Flux, Gems)
- **Regions & Leylines**: Game world locations and special areas

#### 4. **Event-Driven Architecture**
The backend uses a sophisticated message-passing system:

**Message Manager (`messenger.rs`)**:
- Central hub for all game events
- Uses Tokio channels for async message passing
- Handles events like:
  - Explore actions
  - Channeling leylines
  - Quest progression
  - Combat actions

**Event Handlers**:
- `ExploreHandler`: Manages region exploration
- `ChannelingHandler`: Handles leyline channeling for resources
- `QuestHandler`: Quest acceptance and completion
- `LootBoxHandler`: Generates rewards for completed actions
- `CostHandler`: Deducts resources/stamina for actions

#### 5. **Task Management System**
Implements an async task system for time-based actions:
- Base `Task` trait for all game actions
- Tasks have durations and track start/completion times
- Status tracking (InProgress, Completed)
- Hero-specific task execution

#### 6. **Combat System**
A complex turn-based combat system with:

**Combat Controller**:
- Manages combat encounters between heroes and NPCs
- Handles combat state persistence
- WebSocket-based real-time combat updates

**Combat Components**:
- `CombatEncounter`: Main combat state container
- `CombatantType`: Heroes and Monsters
- `DecisionMaker`: AI and player decision interfaces
- Card-based combat with effects and targeting

**Combat Flow**:
1. Hero initiates combat (via quest action or raid)
2. Combat controller creates encounter
3. WebSocket connection established for real-time updates
4. Turn-based combat with card playing
5. Rewards distributed on completion

#### 7. **API Endpoints**
RESTful + WebSocket APIs organized by domain:

**Authentication** (`/api/auth`):
- Wallet-based login with signature verification

**Heroes** (`/api/heroes`):
- Create heroes
- Get hero state
- View completed actions

**Actions** (`/api/actions`):
- Explore regions
- Channel leylines
- Active actions tracking (WebSocket)

**Combat** (`/api/combat`):
- WebSocket endpoint for real-time combat
- Combat state management

**Cards & Decks** (`/api/cards`, `/api/decks`):
- Card collection management
- Deck building and management

**Quests** (`/api/quests`):
- Accept quests
- Perform quest actions
- Track quest completion

#### 8. **Infrastructure Layer**
- **Infra**: Singleton pattern for accessing repositories
- **Repositories**: Data access layer with typed Prisma queries
- **Redis Storage**: For combat state persistence and session management
- **Logger**: Centralized logging with Elasticsearch integration

#### 9. **Security & Middleware**
- CORS configuration for cross-origin requests
- JWT token validation
- Session management with Redis
- Anonymous user rejection middleware

#### 10. **Key Design Patterns**

**Event Sourcing**: Actions create immutable completion records

**Message Queue Pattern**: All game actions go through the central message manager

**Repository Pattern**: Clean separation between data access and business logic

**Actor Model**: Combat system uses actor-based concurrency

**Dependency Injection**: AppState provides shared dependencies to handlers

### Data Flow Example (Hero Explores Region)

1. **Client Request** → `/api/explore/{region}` endpoint
2. **Handler** validates hero can explore (stamina, location)
3. **Message** sent to `MESSENGER`: `Command::Explore`
4. **ExploreHandler** creates exploration task
5. **Task** executes with configured duration
6. **Completion** triggers `ExploreCompleted` event
7. **LootBoxHandler** generates rewards (resources, XP, discovery)
8. **CostHandler** deducts stamina cost
9. **Database** updates via Prisma transactions
10. **Client** receives success response

### Scalability Features

- **Async Everything**: Non-blocking I/O throughout
- **Message Queue**: Decoupled event processing
- **Redis**: Distributed session and combat state
- **Stateless API**: Horizontal scaling ready
- **Resource Pooling**: Database connection management

This architecture provides a robust foundation for an idle RPG with real-time combat, asynchronous task processing, and scalable event handling.

## Architecture Diagram

```mermaid
graph TD
    9["User<br>External Actor"]
    subgraph 1["Development & Deployment Infrastructure<br>Various"]
        40["Docker Configuration<br>Docker"]
        41["Build & Dependency Management<br>Rust Cargo"]
        42["Deployment Configuration<br>Kamal YAML"]
        43["Testing<br>Rust"]
    end
    subgraph 2["Prisma CLI System<br>Rust"]
        39["CLI Application<br>Rust"]
    end
    subgraph 3["Database System<br>PostgreSQL Prisma"]
        36["Prisma Schema<br>Prisma"]
        37["Prisma Migrations<br>Prisma"]
        38["Seed Data<br>SQL"]
    end
    subgraph 4["Backend Server System<br>Rust"]
        14["Authentication<br>Rust"]
        31["Game Models<br>Rust"]
        32["Configuration<br>Rust YAML"]
        33["Logging & Telemetry<br>Rust TOML"]
        34["Session State<br>Rust"]
        35["Utilities & Macros<br>Rust"]
        subgraph 5["Database Interaction & Repositories<br>Rust Prisma"]
            25["Card Repository<br>Rust Prisma"]
            26["Hero Repository<br>Rust Prisma"]
            27["Prisma Client Integration<br>Rust Prisma"]
            28["Database Client Setup<br>Rust"]
            29["Repository Trait<br>Rust"]
            30["Repository Helpers<br>Rust"]
        end
        subgraph 6["Services<br>Rust"]
            19["Combat Services<br>Rust"]
            20["Region Service<br>Rust"]
            21["Task Services<br>Rust"]
            22["Redis Storage<br>Rust"]
            23["Async Task Trait<br>Rust"]
            24["Combat Decision Maker Trait<br>Rust"]
        end
        subgraph 7["Game Logic & Event Handling<br>Rust"]
            15["Combat Event Handling<br>Rust"]
            16["Action Handling<br>Rust"]
            17["Game Events Module<br>Rust"]
            18["Persistent Wrapper<br>Rust"]
        end
        subgraph 8["Web Server & API Endpoints<br>Rust Actix-web"]
            10["Authentication Endpoints<br>Rust"]
            11["Game Data Endpoints<br>Rust"]
            12["Combat Socket<br>Rust WebSocket"]
            13["API Response Structures<br>Rust"]
        end
        %% Edges at this level (grouped by source)
        6["Services<br>Rust"] -->|Performs operations| 5["Database Interaction & Repositories<br>Rust Prisma"]
        7["Game Logic & Event Handling<br>Rust"] -->|Updates game state| 5["Database Interaction & Repositories<br>Rust Prisma"]
        7["Game Logic & Event Handling<br>Rust"] -->|Uses services| 6["Services<br>Rust"]
        14["Authentication<br>Rust"] -->|Reads user data| 5["Database Interaction & Repositories<br>Rust Prisma"]
        14["Authentication<br>Rust"] -->|Manages session| 34["Session State<br>Rust"]
        31["Game Models<br>Rust"] -->|Defines structures for| 5["Database Interaction & Repositories<br>Rust Prisma"]
        31["Game Models<br>Rust"] -->|Defines structures for| 6["Services<br>Rust"]
        31["Game Models<br>Rust"] -->|Defines structures for| 7["Game Logic & Event Handling<br>Rust"]
        8["Web Server & API Endpoints<br>Rust Actix-web"] -->|Triggers game logic| 7["Game Logic & Event Handling<br>Rust"]
        8["Web Server & API Endpoints<br>Rust Actix-web"] -->|Authenticates requests| 14["Authentication<br>Rust"]
        8["Web Server & API Endpoints<br>Rust Actix-web"] -->|Uses session state| 34["Session State<br>Rust"]
    end
    %% Edges at this level (grouped by source)
    2["Prisma CLI System<br>Rust"] -->|Manages| 3["Database System<br>PostgreSQL Prisma"]
    5["Database Interaction & Repositories<br>Rust Prisma"] -->|Connects to| 3["Database System<br>PostgreSQL Prisma"]
    9["User<br>External Actor"] -->|Interacts with| 8["Web Server & API Endpoints<br>Rust Actix-web"]
```