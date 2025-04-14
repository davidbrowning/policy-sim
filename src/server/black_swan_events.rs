use rand::Rng; // This explicit import is needed for the gen() method
use crate::server::individual::Individual;

/// Represents the effects a black swan event has on individuals
#[derive(Debug, Clone)]
pub struct EventEffect {
    pub health_impact: f64,
    pub happiness_impact: f64,
    pub wealth_impact: f64,
    pub duration_ticks: u32,
}

impl Default for EventEffect {
    fn default() -> Self {
        EventEffect {
            health_impact: 0.0,
            happiness_impact: 0.0,
            wealth_impact: 0.0,
            duration_ticks: 1,
        }
    }
}

/// Represents a potential black swan event in the simulation
#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub probability: f64,
    pub effects: EventEffect,
}

impl Event {
    /// Creates a new event
    pub fn new(id: u32, name: &str, description: &str, probability: f64, effects: EventEffect) -> Self {
        Event {
            id,
            name: name.to_string(),
            description: description.to_string(),
            probability,
            effects,
        }
    }

    /// Determines if the event should trigger based on its probability
    pub fn should_trigger<R: Rng>(&self, rng: &mut R) -> bool {
        let random_value: f64 = rng.random();
        random_value < self.probability
    }

    /// Applies the event effects to all individuals
    pub fn apply(&self, individuals: &mut Vec<Individual>) {
        for individual in individuals.iter_mut() {
            individual.health += self.effects.health_impact;
            individual.happiness += self.effects.happiness_impact;
            individual.wealth += self.effects.wealth_impact;
            
            // Ensure values don't go below zero
            individual.health = individual.health.max(0.0);
            individual.happiness = individual.happiness.max(0.0);
            individual.wealth = individual.wealth.max(0.0);
        }
    }
}

/// Manager for all black swan events in the simulation
pub struct EventSystem {
    events: Vec<Event>,
    active_events: Vec<(Event, u32)>, // (Event, remaining_duration)
}

impl EventSystem {
    /// Creates a new event system
    pub fn new() -> Self {
        EventSystem {
            events: Vec::new(),
            active_events: Vec::new(),
        }
    }

    /// Adds an event to the system
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Processes the current tick, checking for new events and applying active ones
    pub fn process_tick<R: Rng>(&mut self, rng: &mut R, individuals: &mut Vec<Individual>) {
        // Check for new events
        for event in &self.events {
            if event.should_trigger(rng) {
                println!("Black swan event triggered: {}", event.name);
                self.active_events.push((event.clone(), event.effects.duration_ticks));
            }
        }

        // Apply active events
        for (event, _) in &self.active_events {
            event.apply(individuals);
        }

        // Update duration of active events
        let mut i = 0;
        while i < self.active_events.len() {
            let (_, duration) = &mut self.active_events[i];
            *duration -= 1;
            
            if *duration == 0 {
                self.active_events.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Returns a list of active events
    pub fn get_active_events(&self) -> Vec<&Event> {
        self.active_events.iter().map(|(event, _)| event).collect()
    }

    /// Loads predefined events into the system
    pub fn load_predefined_events(&mut self) {
        // Natural disasters
        self.add_event(Event::new(
            1,
            "Earthquake",
            "A major earthquake hits the region",
            0.01,
            EventEffect {
                health_impact: -20.0,
                happiness_impact: -15.0,
                wealth_impact: -30.0,
                duration_ticks: 10,
            },
        ));

        self.add_event(Event::new(
            2,
            "Flooding",
            "Severe flooding affects large areas",
            0.03,
            EventEffect {
                health_impact: -10.0,
                happiness_impact: -20.0,
                wealth_impact: -25.0,
                duration_ticks: 8,
            },
        ));

        // Economic events
        self.add_event(Event::new(
            3,
            "Economic Boom",
            "Unexpected economic prosperity",
            0.05,
            EventEffect {
                health_impact: 5.0,
                happiness_impact: 15.0,
                wealth_impact: 25.0,
                duration_ticks: 20,
            },
        ));

        self.add_event(Event::new(
            4,
            "Market Crash",
            "Sudden market downturn affecting investments",
            0.02,
            EventEffect {
                health_impact: -5.0,
                happiness_impact: -25.0,
                wealth_impact: -40.0,
                duration_ticks: 15,
            },
        ));

        // Health events
        self.add_event(Event::new(
            5,
            "Medical Breakthrough",
            "A significant medical advancement improves healthcare",
            0.04,
            EventEffect {
                health_impact: 15.0,
                happiness_impact: 10.0,
                wealth_impact: 0.0,
                duration_ticks: 30,
            },
        ));

        self.add_event(Event::new(
            6,
            "Epidemic",
            "A disease outbreak affects the population",
            0.015,
            EventEffect {
                health_impact: -30.0,
                happiness_impact: -20.0,
                wealth_impact: -10.0,
                duration_ticks: 12,
            },
        ));
    }
}

#[cfg(test)]
mod events_tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_event_apply_logic() {
        // Create a test earthquake event
        let earthquake = Event {
            id: 1,
            name: "Earthquake".to_string(),
            description: "A major earthquake hits the region".to_string(),
            probability: 0.01,
            effects: EventEffect {
                health_impact: -20.0,
                happiness_impact: -15.0,
                wealth_impact: -30.0,
                duration_ticks: 10,
            },
        };
        
        // Create some test individuals
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 80.0,
                wealth: 100.0,
                // Other fields...
            },
            Individual {
                id: 2,
                health: 90.0,
                happiness: 70.0,
                wealth: 120.0,
                // Other fields...
            },
        ];
        
        // Apply earthquake event
        earthquake.apply(&mut individuals);
        
        // Verify impacts on individuals
        assert_eq!(individuals[0].health, 80.0);    // 100 - 20
        assert_eq!(individuals[0].happiness, 65.0); // 80 - 15
        assert_eq!(individuals[0].wealth, 70.0);    // 100 - 30
        
        assert_eq!(individuals[1].health, 70.0);    // 90 - 20
        assert_eq!(individuals[1].happiness, 55.0); // 70 - 15
        assert_eq!(individuals[1].wealth, 90.0);    // 120 - 30
    }

    #[test]
    fn test_event_trigger_probability() {
        // Create events with different probabilities
        let common_event = Event {
            id: 1,
            name: "Common Event".to_string(),
            description: "Happens frequently".to_string(),
            probability: 0.8, // 80% chance
            effects: EventEffect::default(),
        };
        
        let rare_event = Event {
            id: 2,
            name: "Rare Event".to_string(),
            description: "Happens rarely".to_string(),
            probability: 0.05, // 5% chance
            effects: EventEffect::default(),
        };
        
        // Use seeded RNG for deterministic tests
        let mut rng = StdRng::seed_from_u64(42);
        
        // Run many trials to test probability
        let trials = 1000;
        let mut common_triggers = 0;
        let mut rare_triggers = 0;
        
        for _ in 0..trials {
            if common_event.should_trigger(&mut rng) {
                common_triggers += 1;
            }
            
            if rare_event.should_trigger(&mut rng) {
                rare_triggers += 1;
            }
        }
        
        // Common event should trigger close to 80% of the time
        let common_rate = common_triggers as f64 / trials as f64;
        assert!((common_rate - 0.8).abs() < 0.05);
        
        // Rare event should trigger close to 5% of the time
        let rare_rate = rare_triggers as f64 / trials as f64;
        assert!((rare_rate - 0.05).abs() < 0.03);
    }

    #[test]
    fn test_event_system_process_tick() {
        let mut event_system = EventSystem::new();
        
        // Add a guaranteed event (100% probability)
        event_system.add_event(Event::new(
            1,
            "Test Event",
            "An event that always happens",
            1.0,
            EventEffect {
                health_impact: -10.0,
                happiness_impact: -5.0,
                wealth_impact: -15.0,
                duration_ticks: 3,
            },
        ));
        
        // Create test individuals
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 100.0,
                wealth: 100.0,
                // Other fields...
            },
        ];
        
        // Seed RNG for deterministic tests
        let mut rng = StdRng::seed_from_u64(42);
        
        // Process first tick - event should trigger and apply
        event_system.process_tick(&mut rng, &mut individuals);
        
        // Check the event was applied
        assert_eq!(individuals[0].health, 90.0);
        assert_eq!(individuals[0].happiness, 95.0);
        assert_eq!(individuals[0].wealth, 85.0);
        
        // Check that the event is active
        assert_eq!(event_system.active_events.len(), 1);
        
        // Process 2 more ticks
        event_system.process_tick(&mut rng, &mut individuals);
        event_system.process_tick(&mut rng, &mut individuals);
        
        // Event should continue applying effects
        assert_eq!(individuals[0].health, 70.0);
        assert_eq!(individuals[0].happiness, 85.0);
        assert_eq!(individuals[0].wealth, 55.0);
        
        // After the third tick, the event should expire
        assert_eq!(event_system.active_events.len(), 0);
    }

    #[test]
    fn test_multiple_active_events() {
        let mut event_system = EventSystem::new();
        
        // Add two guaranteed events with different durations
        event_system.add_event(Event::new(
            1,
            "Short Event",
            "An event with short duration",
            1.0,
            EventEffect {
                health_impact: -5.0,
                happiness_impact: 0.0,
                wealth_impact: 0.0,
                duration_ticks: 1,
            },
        ));
        
        event_system.add_event(Event::new(
            2,
            "Long Event",
            "An event with longer duration",
            1.0,
            EventEffect {
                health_impact: 0.0,
                happiness_impact: -5.0,
                wealth_impact: 0.0,
                duration_ticks: 3,
            },
        ));
        
        // Create test individuals
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 100.0,
                wealth: 100.0,
                // Other fields...
            },
        ];
        
        let mut rng = StdRng::seed_from_u64(42);
        
        // Process first tick - both events should trigger
        event_system.process_tick(&mut rng, &mut individuals);
        
        // Check that both events were applied
        assert_eq!(individuals[0].health, 95.0);  // -5 from short event
        assert_eq!(individuals[0].happiness, 95.0);  // -5 from long event
        
        // After first tick, short event should expire but long event remains
        assert_eq!(event_system.active_events.len(), 1);
        
        // Process second tick
        event_system.process_tick(&mut rng, &mut individuals);
        
        // Only long event should continue applying effects
        assert_eq!(individuals[0].health, 95.0);  // No change
        assert_eq!(individuals[0].happiness, 90.0);  // Additional -5 from long event
    }
}

