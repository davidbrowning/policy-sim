Act as a Prompt Enhancer AI that takes user-input prompts and transforms them into more engaging, detailed, and thought-provoking questions. Describe the process you follow to enhance a prompt, the types of improvements you make, and share an example of how you'd turn a simple, one-sentence prompt into an enriched, multi-layered question that encourages deeper thinking and more insightful responses.

Here is the first prompt I want you to enhance: 

"Act as an expert software architect, create design.md for a policy simulation game built with rust, leaning on sqlite and grafana as a display. minimize complexity to every extent possible. 

Details 
---

- The simulation will essentially consist of
	- a policy editor (all post-production polish will go into making this cleaner) which will edit policies in a plain text format (thinking json or yaml)
	- a grafana dashboard displaying metrics
	- a simulation server that consumes the policies and adjusts the metrics based on what happens in the simulation
- [[Individuals]] are where the core of the simulation lives. Individuals can make decisions and will base those decisions on policies (real or maybe even just perceived)
- [[Policy Simulation Metrics]] will include the following list. Metrics will be updated at intervals specified in the metric itself. A real-world application of this idea is the US Census and laws around Census taking.
	- war casualties (including wounded)
	- debt (individual and nation)
	- death
	- birth
	- muscle mass
	- books read
	- water quality
	- welfare recipients
	- passport holders
	- natural resources remaining
	- custom metrics will be possible for advanced players
- [[Policy Simulation Individual Actions]]
	- sleeping
	- eating
	- reproducing
	- gathering
		- resources
		- information
	- seeking
		- community
		- pleasure
		- enlightenment
		- adventure
		- wealth
		- knowledge
- Ideas:
	- All seeking behaviors can be done above board or illicitly
	- Any human individual will have a base ability and requirement for gathering resources, eating, and sleeping. Base stats should be comparable to a hunter/gather community and adjusted from there to reflect reality. Multipliers depending on geographic location and natural resource availability.
	- Community is simulated based on resource availability and allocation
	- Policy affects the difficulty and time required to satisfy each individual behavior.
	  Policy adoption and enforcement is inversely related to the number of sub groups (for example, a family policy is enforced 100% of individuals belonging to that family, a city 90%, a state 80% and a nation 70% - todo play with these numbers)  
	- Conflicting policies will result in diminished adoption and enforcement among all affected communities but will favor the one with fewer subgroups
	- Alignment in national policies between differing nations will facilitate peace, misalignment will facilitate war. War is a policy decision.
	- Must code in black swan events.
	- What is the driving force of humanity?
	- Success is defined by the player rather than the game.


Example data
---
sleeping: 8
eating: 2
reproducing: 1
gathering:
  resources: 6
  information: 2
seeking: 
  community: 2
  pleasure: 2
  enlightenment: 1"
