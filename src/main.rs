mod server;
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::server::individual::Individual;
use crate::server::metrics_system::MetricsSystem;
use crate::server::black_swan_events::EventSystem;
use crate::server::black_swan_events::EventEffect;
use crate::server::black_swan_events::Event;
use std::{thread, time::Duration};

fn main() {
    println!("Starting simulation!");
    //let mut rng = StdRng::seed_from_u64(42);
    println!("Initialize Event System");
    let mut metrics_system = MetricsSystem::new();
    let mut individuals = vec![
        Individual {
            id: 1,
            health: 80.0,
            happiness: 70.0,
            wealth: 100.0,
            community_id: 1,
            age: 30,
            children: 2,
            traits: vec![],
        },
        Individual {
            id: 2,
            health: 60.0,
            happiness: 90.0,
            wealth: 50.0,
            community_id: 1,
            age: 25,
            children: 0,
            traits: vec![],
        },
        Individual {
            id: 3,
            health: 100.0,
            happiness: 60.0,
            wealth: 150.0,
            community_id: 2,
            age: 40,
            children: 3,
            traits: vec![],
        },
    ];
    println!("Initialize 3 Individuals");
    println!("Calculate predefined metrics");
    let metrics = metrics_system.calculate_predefined_metrics(&individuals);
    println!("{:?}", metrics);
    println!("Initialize Event System");
    let mut tick = 0;
    let mut event_system = EventSystem::new();
    println!("Process 10 event ticks (no events)");
    let mut rng = StdRng::seed_from_u64(42);
    for n in 0..10 {
        tick += 1;
        event_system.process_tick(&mut rng, &mut individuals);
        metrics_system.update_metrics(&individuals, tick);
        thread::sleep(Duration::from_millis(1000));
        println!("Current metrics {:?}", metrics_system.get_all_metrics());
        println!("Iteration number {:?}", n);
    }

    println!("EARTHQUAKE!");
    let earthquake = Event {
        id: 1,
        name: "Earthquake".to_string(),
        description: "A major earthquake hits the region".to_string(),
        probability: 1.0,
        effects: EventEffect {
            health_impact: -20.0,
            happiness_impact: -15.0,
            wealth_impact: -30.0,
            duration_ticks: 5,
        },
    };

    earthquake.apply(&mut individuals);
    event_system.add_event(earthquake);
    println!("Individuals: {:?}", individuals);
    println!("Process 10 event ticks (observe effect of earthquake)");
    for n in 0..10 {
        tick += 1;
        event_system.process_tick(&mut rng, &mut individuals);
        metrics_system.update_metrics(&individuals, tick);
        thread::sleep(Duration::from_millis(1000));
        println!("Current metrics {:?}", metrics_system.get_all_metrics());
        println!("Iteration number {:?}", n);
    }

        
}
