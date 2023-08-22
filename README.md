<h1>Stupid 🦀 CRUD using Axum</h1>

<p>I just practice a Rust for the reason of Golang boring. I need more challenging in my backend developer career path.</p>

<p>Therefore, I've made this project. Just a stupid project but have made me proud.</p>

```rust
fn main() {
    println!("THIS IS MY 1st FOR 🦀")
}
```

<h2>🐳 Start Project on Docker Compose</h2>

<p>Start app</p>

```bash
docker compose up -d
```

<p>Insert mock data into the database</p>

```bash
docker exec -it stupid-crab-api-db bash
```

```bash
mongosh mongodb://root:123456@0.0.0.0:27017
```

```bash
use crab_db
```

```bash
db.items.insertMany([
  {
    "name": "Sword of Valor",
    "description": "One-Handed Sword",
    "damage": 120,
    "level_required": 30,
    "price": 1500
  },
  {
    "name": "Elven Bow",
    "description": "Bow",
    "damage": 90,
    "level_required": 25,
    "price": 1200
  },
  {
    "name": "Staff of Fire",
    "description": "Staff",
    "damage": 105,
    "level_required": 28,
    "price": 1400
  },
  {
    "name": "Dwarven Hammer",
    "description": "Two-Handed Hammer",
    "damage": 160,
    "level_required": 35,
    "price": 1800
  },
  {
    "name": "Assassin's Dagger",
    "description": "Dagger",
    "damage": 80,
    "level_required": 22,
    "price": 1000
  }
])
```