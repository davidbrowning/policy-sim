# Policy Simulator: A Philosophical Simulation Sandbox

## Project Description

PolicySim is a Rust-based simulation designed to explore the fundamental question: "What drives humanity?" It's a philosophical sandbox where you can experiment with different societal policies and observe their effects on a simulated population. The simulation incorporates elements of resource management, individual behavior, and unexpected events to provide a rich and dynamic environment for experimentation.

## Core Features

* **Action Modeling**: The simulation includes a system for modeling actions that individuals can take, such as farming, hunting, and trading. These actions have associated costs and benefits, including impacts on health, happiness, and wealth.
* **Policy Effects**: Players can define policies that modify the simulation's dynamics. Policies can affect action difficulty, time requirements, and relative importance, allowing players to guide the behavior of the simulated individuals.
* **Black Swan Events**: The simulation features a system for "black swan" events—rare, high-impact occurrences like natural disasters or economic crises. These events introduce an element of unpredictability and challenge players to adapt to unforeseen circumstances.
* **Individual Agency**: Simulated individuals make choices based on their needs, desires, and the prevailing policies. The simulation models how individuals respond to different conditions and how their behavior shapes the overall community.
* **Community Dynamics**: Individuals are grouped into communities, and their interactions and collective behavior influence the simulation's progression.
* **Geography**: The simulation incorporates geographical factors that affect resource availability and individual well-being.
* **Traits**: Individuals possess traits that influence their behavior and how they respond to different policies and events.

## Technical Architecture

* **Rust**: The simulation is implemented in Rust, leveraging its performance, memory safety, and concurrency features.
* **ECS Architecture**: The simulation uses an Entity-Component-System (ECS) architecture, specifically the `hecs` crate, to manage the simulated individuals and their attributes.
* **Randomness**: The `rand` crate is used for generating random events and individual choices, adding an element of unpredictability to the simulation.
