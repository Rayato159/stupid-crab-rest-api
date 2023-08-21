<h1>Stupid 🦀 CRUD using Axum</h1>

<p>I just practice a Rust for the reason of Golang boring. I need more challenging in my backend developer career path.</p>

<p>Therefore, I've made this project. Just a stupid project but have made me proud.</p>

```rust
fn main() {
    println!("THIS IS MY 1st FOR 🦀")
}
```

<h2>🐳 Start MongoDb on Docker</h2>

<p>Pull image and run the container</p>

```bash
docker pull mongo:latest
```

```bash
docker run --name crab-db -e MONGO_INITDB_ROOT_USERNAME=root -e MONGO_INITDB_ROOT_PASSWORD=123456 -p 27017:27017 -d mongo
```

<p>Using mongoshell</p>

```bash
docker exec -it crab-db bash
```

<p>Creating a database</p>

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