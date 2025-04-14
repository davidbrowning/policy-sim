use rand::Rng;
use crate::server::individual::Individual;
use crate::server::types::Trait;

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

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub probability: f64,
    pub effects: EventEffect,
}

impl Event {
    pub fn new(id: u32, name: &str, description: &str, probability: f64, effects: EventEffect) -> Self {
        Event {
            id,
            name: name.to_string(),
            description: description.to_string(),
            probability,
            effects,
        }
    }

    pub fn should_trigger<R: Rng>(&self, rng: &mut R) -> bool {
        rng.gen::<f64>() < self.probability
    }

    pub fn apply(&self, individuals: &mut Vec<Individual>) {
        for individual in individuals.iter_mut() {
            individual.health += self.effects.health_impact;
            individual.happiness += self.effects.happiness_impact;
            individual.wealth += self.effects.wealth_impact;
            individual.health = individual.health.max(0.0);
            individual.happiness = individual.happiness.max(0.0);
            individual.wealth = individual.wealth.max(0.0);
        }
    }
}

pub struct EventSystem {
    events: Vec<Event>,
    active_events: Vec<(Event, u32)>,
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            events: Vec::new(),
            active_events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn process_tick<R: Rng>(&mut self, rng: &mut R, individuals: &mut Vec<Individual>) {
        for event in &self.events {
            if event.should_trigger(rng) {
                println!("Black swan event triggered: {}", event.name);
                self.active_events.push((event.clone(), event.effects.duration_ticks));
            }
        }

        for (event, _) in &self.active_events {
            event.apply(individuals);
        }

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

    pub fn get_active_events(&self) -> Vec<&Event> {
        self.active_events.iter().map(|(event, _)| event).collect()
    }

    pub fn load_predefined_events(&mut self) {
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
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_event_apply_logic() {
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
        
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 80.0,
                wealth: 100.0,
                community_id: 1,
                traits: vec![Trait::Industrious],
                age: 20,
                children: 0,
            },
            Individual {
                id: 2,
                health: 90.0,
                happiness: 70.0,
                wealth: 120.0,
                community_id: 2,
                traits: vec![Trait::Industrious],
                age: 20,
                children: 0,
            },
        ];
        
        earthquake.apply(&mut individuals);
        
        assert_eq!(individuals[0].health, 80.0);
        assert_eq!(individuals[0].happiness, 65.0);
        assert_eq!(individuals[0].wealth, 70.0);
        
        assert_eq!(individuals[1].health, 70.0);
        assert_eq!(individuals[1].happiness, 55.0);
        assert_eq!(individuals[1].wealth, 90.0);
    }

    #[test]
    fn test_event_trigger_probability() {
        let common_event = Event {
            id: 1,
            name: "Common Event".to_string(),
            description: "Happens frequently".to_string(),
            probability: 0.8,
            effects: EventEffect::default(),
        };
        
        let rare_event = Event {
            id: 2,
            name: "Rare Event".to_string(),
            description: "Happens rarely".to_string(),
            probability: 0.05,
            effects: EventEffect::default(),
        };
        
        let mut rng = StdRng::seed_from_u64(42);
        
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
        
        let common_rate = common_triggers as f64 / trials as f64;
        assert!((common_rate - 0.8).abs() < 0.05);
        
        let rare_rate = rare_triggers as f64 / trials as f64;
        assert!((rare_rate - 0.05).abs() < 0.03);
    }

    #[test]
    fn test_event_system_process_tick() {
        let mut event_system = EventSystem::new();
        
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
        
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 100.0,
                wealth: 100.0,
                community_id: 3,
                traits: vec![Trait::Industrious],
                age: 20,
                children: 0,
            },
        ];
        
        let mut rng = StdRng::seed_from_u64(42);
        
        event_system.process_tick(&mut rng, &mut individuals);
        
        assert_eq!(individuals[0].health, 90.0);
        assert_eq!(individuals[0].happiness, 95.0);
        assert_eq!(individuals[0].wealth, 85.0);
        
        assert_eq!(event_system.active_events.len(), 1);
        
        event_system.process_tick(&mut rng, &mut individuals);
        event_system.process_tick(&mut rng, &mut individuals);
        
        assert_eq!(individuals[0].health, 70.0);
        assert_eq!(individuals[0].happiness, 85.0);
        assert_eq!(individuals[0].wealth, 55.0);
        
        assert_eq!(event_system.active_events.len(), 0);
    }

    #[test]
    fn test_multiple_active_events() {
        let mut event_system = EventSystem::new();
        
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
        
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 100.0,
                wealth: 100.0,
                community_id: 3,
                traits: vec![Trait::Industrious],
                age: 20,
                children: 0,
            },
        ];
        
        let mut rng = StdRng::seed_from_u64(42);
        
        event_system.process_tick(&mut rng, &mut individuals);
        
        assert_eq!(individuals[0].health, 95.0);
        assert_eq!(individuals[0].happiness, 95.0);
        
        assert_eq!(event_system.active_events.len(), 1);
        
        event_system.process_tick(&mut rng, &mut individuals);
        
        assert_eq!(individuals[0].health, 95.0);
        assert_eq!(individuals[0].happiness, 90.0);
    }
}
