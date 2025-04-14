# Policy Simulation Game Design Document

## Overview
This document outlines the architecture for a policy simulation game developed in Rust, utilizing SQLite for data management and Grafana for real-time dashboards. The design prioritizes simplicity, maintainability, and scalability while delivering an engaging experience that allows players to explore policy-making and its societal impacts. The simulation centers on individuals whose behaviors are influenced by policies, with metrics and black swan events adding depth and challenge.

## System Architecture

### High-Level Component Diagram
```
[Simulation Server]
   | (in-memory metric cache, batch writes every 10 ticks)
   v
[SQLite Database] <-> [Grafana (SQL queries via plugin)]
   | (policies, individuals, events)
   v
[Disk (single .sqlite file)]
```

- **Policy Editor**: A user-friendly interface for creating and editing policies in YAML.
- **Simulation Server**: A Rust-based engine that processes policies, simulates individual behaviors, and updates metrics.
- **SQLite Database**: Stores policies, individual states, metrics, and event data.
- **Grafana Dashboard**: Displays real-time simulation metrics with customizable views.

### Data Flow
1. **Policy Creation**: Players define policies in the editor, saved as YAML files and persisted in SQLite.
2. **Simulation Loop**:
   - The server reads policies and individual states from SQLite.
   - Policies are applied to individuals, updating their action costs (e.g., time to eat).
   - Individual decisions are simulated based on stats, policies, and environment.
   - Metrics (e.g., birth rates, debt) are calculated and written to SQLite.
3. **Dashboard Rendering**:
   - Grafana queries SQLite via a Rust-based data source plugin.
   - Metrics are aggregated for low-latency visualization.
   - Players customize dashboards to track preferred metrics.

## Database Schema

```sql
-- Policies: Stores player-defined policies
CREATE TABLE policies (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    content TEXT NOT NULL, -- YAML content
    community_id INTEGER,  -- Family, city, nation
    adoption_rate REAL,    -- 0.0 to 1.0
    created_at TIMESTAMP
);

-- Individuals: Tracks individual states
CREATE TABLE individuals (
    id INTEGER PRIMARY KEY,
    community_id INTEGER,
    health REAL,          -- 0.0 to 1.0
    happiness REAL,       -- 0.0 to 1.0
    wealth REAL,          -- Resource units
    action_priorities TEXT -- JSON: { "sleeping": 8, "eating": 2, ... }
);

-- Metrics: Stores simulation metrics
CREATE TABLE metrics (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,   -- e.g., "birth_rate", "water_quality"
    value REAL,
    timestamp TIMESTAMP,
    is_custom BOOLEAN     -- Player-defined metrics
);

-- Events: Stores black swan events
CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    type TEXT NOT NULL,   -- e.g., "earthquake", "market_crash"
    impact REAL,          -- Severity (0.0 to 1.0)
    community_id INTEGER,
    triggered_at TIMESTAMP
);
```

- **Indexes**: Create indexes on `policies.community_id`, `metrics.timestamp`, and `individuals.community_id` for query performance.
- **Extensibility**: The `metrics` table supports custom metrics via `is_custom`. The `action_priorities` JSON field allows flexible action modeling.

## Core Components

### Policy Editor
- **Design**: A CLI-based editor with optional TUI (using `ratatouille`) for YAML policy creation. Policies define action cost modifiers (e.g., `eating: { time: 2, difficulty: 1.5 }`) and community scope.
- **Simplicity**: YAML minimizes syntax complexity. A template system provides defaults (e.g., `{ sleeping: { time: 8 } }`) to reduce cognitive load.
- **Extensibility**: Supports plugins for syntax highlighting (via `tree-sitter`) and validation (e.g., schema checks). Policies are parsed into Rust structs for simulation.
- **Example Policy**:
  ```yaml
  name: Universal Healthcare
  community: nation
  effects:
    health: { multiplier: 1.2 }
    wealth: { multiplier: 0.9 }
    eating: { time: 1.8 }
  adoption_rate: 0.7
  ```

### Simulation Server
- **Design**: A single-threaded Rust server using `tokio` for async I/O. The server runs a tick-based loop (e.g., 1 tick = 1 simulated day).
- **Optimization**:
  - Policies are cached in memory to reduce SQLite queries.
  - Individual updates are batched using `sqlx` transactions.
  - A component-based system (using `hecs` ECS) separates individual stats, actions, and policy effects for modularity.
- **Simulation Loop**:
  1. Load policies and individuals.
  2. Apply policy effects (e.g., adjust action costs).
  3. Simulate individual decisions (see below).
  4. Update metrics and persist to SQLite.
  5. Check for black swan events.

### Grafana Dashboard
- **Design**: A custom Grafana data source plugin (Rust-based) queries SQLite for metrics. Dashboards display time-series data (e.g., birth rates over time) and aggregate stats (e.g., average health per community).
- **Data Pipeline**:
  - Metrics are aggregated in SQLite views (e.g., `SELECT AVG(value) FROM metrics WHERE name='health' GROUP BY timestamp`).
  - The plugin uses `sqlx` for low-latency queries.
  - Players customize dashboards via Grafana’s UI, selecting metrics and time ranges.
- **Visualization**:
  - Line charts for trends (e.g., water quality).
  - Bar charts for comparisons (e.g., debt across communities).
  - Heatmaps for spatial metrics (if geography is added).
- **Performance**: Limit queries to 100-200 data points per chart. Use SQLite’s `VACUUM` to optimize storage.

## Simulation Dynamics

### Individual Decision-Making
- **Model**: Individuals have stats (`health`, `happiness`, `wealth`) and action priorities (e.g., `{ sleeping: 8, eating: 2 }`). Decisions are made via a weighted random choice:
  ```rust
  fn choose_action(individual: &Individual, policies: &[Policy]) -> Action {
      let mut priorities = individual.action_priorities.clone();
      for policy in policies {
          priorities.apply(policy.effects); // Adjust time/difficulty
      }
      let total_weight = priorities.values().sum();
      let choice = rand::random::<f32>() * total_weight;
      priorities.weighted_select(choice)
  }
  ```
- **Realism vs. Efficiency**: Limit actions to 5-10 per individual (e.g., sleeping, eating, seeking community). Use a multiplier system for environmental factors (e.g., `wealth *= 0.8` in arid regions).
- **Diversity**: Model decision biases via traits (e.g., `rational: 0.7`, `emotional: 0.3`). Traits adjust weights dynamically (e.g., emotional individuals prioritize community).

### Policy Impact
- **Design**: Policies modify action costs and stats via multipliers. For example:
  ```rust
  struct PolicyEffect {
      action: String,
      time: f32,     // Hours required
      difficulty: f32 // Stat cost multiplier
  }
  ```
- **Conflicting Policies**: Policies from different communities (e.g., family vs. nation) are blended using adoption rates:
  ```rust
  fn blend_effects(effects: &[PolicyEffect], adoption_rates: &[f32]) -> PolicyEffect {
      let total_adoption = adoption_rates.iter().sum::<f32>();
      effects.iter().fold(PolicyEffect::default(), |acc, e| {
          let weight = adoption_rates[e.id] / total_adoption;
          acc + e * weight
      })
  }
  ```
- **Adoption Rates**: Smaller communities (e.g., family) have higher adoption (e.g., 0.9) than nations (e.g., 0.5), reflecting trust dynamics.

### Metrics System
- **Design**: Metrics are updated per tick and stored in SQLite. Predefined metrics (e.g., `war_casualties`, `birth_rate`) are hardcoded, while custom metrics are defined via policies:
  ```yaml
  metric:
    name: education_level
    formula: avg(individuals.happiness * 0.5 + wealth * 0.3)
  ```
- **Dynamic Intervals**: Metrics like `birth_rate` update yearly (365 ticks), while `water_quality` updates daily. Use a cron-like scheduler in Rust:
  ```rust
  fn schedule_metric_update(metric: &Metric, tick: u64) -> bool {
      match metric.name {
          "birth_rate" => tick % 365 == 0,
          "water_quality" => true,
          _ => tick % 30 == 0 // Monthly default
      }
  }
  ```

### Black Swan Events
- **Design**: Events are triggered randomly with a probability (e.g., 0.01 per tick). A modular system stores events as structs:
  ```rust
  struct Event {
      id: u64,
      type_: String,
      impact: f32, // 0.0 to 1.0
      apply: fn(&mut SimulationState),
  }
  ```
- **Integration**: Events modify stats or action costs (e.g., `earthquake: health *= 0.7`). A registry allows new events without code changes:
  ```rust
  let events = vec![
      Event { type_: "earthquake", impact: 0.3, apply: |state| state.health *= 0.7 },
      Event { type_: "market_crash", impact: 0.5, apply: |state| state.wealth *= 0.6 },
  ];
  ```
- **Stability**: Cap event frequency (e.g., 1 per 100 ticks) and limit impact (e.g., `health >= 0.1`).

## Individual Actions and Behaviors

### Action Modeling
- **Design**: Actions are time-based (e.g., `sleeping: 8 hours`). Policies adjust time or difficulty:
  ```rust
  struct Action {
      name: String,
      base_time: f32, // Hours
      difficulty: f32 // Stat cost
  }
  ```
- **Balance**: Essential actions (e.g., eating) have fixed priorities, while aspirational actions (e.g., seeking enlightenment) are optional. Illicit actions (e.g., theft) reduce `happiness` but boost `wealth`.
- **Mechanics**: Players tweak action priorities via policies. For example, a policy promoting education increases `seeking_knowledge` weight.

### Behavioral Modeling
- **Baseline**: Individuals start with hunter-gatherer stats (e.g., `health: 0.8`, `wealth: 0.5`). Environmental multipliers adjust stats:
  ```rust
  fn apply_environment(individual: &mut Individual, geography: &Geography) {
      individual.wealth *= geography.resource_factor; // e.g., 0.7 for desert
      individual.health *= geography.hazard_factor;  // e.g., 0.9 for forest
  }
  ```
- **Community Formation**: Individuals join communities based on policy alignment:
  ```rust
  fn join_community(individual: &Individual, policies: &[Policy]) -> Option<u64> {
      policies.iter()
          .max_by_key(|p| p.adoption_rate * individual.policy_alignment(p))
          .map(|p| p.community_id)
  }
  ```

## Game Philosophy and Player Agency

### Player-Defined Success
- **Design**: Players set goals via custom metrics (e.g., `maximize: happiness`). The dashboard highlights progress toward these goals.
- **Feedback**: Simulation outcomes reflect goal alignment (e.g., high `happiness` reduces war risk). A summary report per tick shows key changes.

### Driving Force of Humanity
- **Integration**: Players experiment with “motivations” via policies (e.g., `survival: eating.time *= 0.8`). The simulation reveals trade-offs (e.g., survival-focused policies reduce `happiness`).
- **Reflection**: The dashboard includes a “philosophy panel” summarizing dominant motivations based on player policies, prompting reflection on humanity’s drivers.

### War and Peace
- **Design**: War risk increases with policy misalignment:
  ```rust
  fn war_risk(community_a: &Community, community_b: &Community) -> f32 {
      let alignment = policy_similarity(community_a.policies, community_b.policies);
      1.0 - alignment // Higher misalignment = higher risk
  }
  ```
- **Mechanics**: War reduces `health` and `wealth`. Players mitigate risk via diplomacy policies (e.g., `trade_agreement: alignment += 0.2`).

## Rust-Specific Considerations
- **Modularity**: Use crates for separation (e.g., `policy_parser`, `simulation_core`, `grafana_plugin`).
- **Memory Safety**: Leverage Rust’s ownership model to manage individual states. Use `Arc` for shared policy data.
- **Async Processing**: Use `tokio` for non-blocking SQLite queries and Grafana updates.
- **Libraries**:
  - `sqlx`: Type-safe SQLite integration.
  - `serde`: YAML parsing for policies.
  - `hecs`: ECS for individual simulation.
  - `rand`: Random events and decisions.
- **Testing**: Unit tests for policy parsing and simulation logic. Integration tests for SQLite and Grafana workflows.

## Maintaining Simplicity
- **Constraints**:
  - Limit policies to 10 per community.
  - Cap individuals at 1,000 per simulation.
  - Restrict metrics to 20 (10 predefined, 10 custom).
- **Trade-offs**:
  - Avoid complex AI for individuals; use weighted choices.
  - Simplify geography to 3-5 regions.
  - Batch SQLite writes to reduce I/O.
- **Extensibility**: Use trait-based systems (e.g., `trait ActionEffect`) for new actions or events without refactoring.

## Philosophical Reflection
The design encourages players to explore “What drives humanity?” by letting them define policies that prioritize different motivations (e.g., survival vs. enlightenment). The simulation’s feedback—via metrics, events, and community dynamics—reveals the consequences of these choices. For example, a player focusing on wealth may trigger war, prompting reflection on greed versus harmony. The minimalist architecture ensures players focus on these questions rather than wrestling with complexity, fostering a sandbox for philosophical experimentation.