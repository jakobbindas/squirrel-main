# Untitled Squirrel Game

by Untitled Squirrel Group

## Setup
1. Install Rust
2. Install LLD (see [bevy fast compile setup](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)).
3. Install rust-analyzer for your editor of choice (for [VSCode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)).
4. Set your editor to format on save.

## Team Members
* Advanced Topic Subteam 1: Physics
	* Jakob Bindas
	* Imran Haidery
	* Maddy Powers

* Advanced Topic Subteam 2: Procedural Generation
	* Miles Hamilton
	* Justin Holt
	* Kyra Schultz
	* Sam Wang

## Game Description

*Untitled Squirrel Game* is a puzzle-platformer adventure game where you play as a squirrel trying to store acorns for the winter. To do so, you'll need to traverse a procedurally generated cave system, solve various puzzles, and squirrel your way around danger.

## Advanced Topic Description

### Physics

The pride and joy of *Untitled Squirrel Game*'s physics is simulated fluid dynamics--i.e., water and lava simluations with player and environmental interactions! Also, flight dynamics will come into play as one reaches the depths of the Earth.

### Procedural Generation

Each level is a procedurally-generated tile-based cave. Uses binary space partitioning to create a map of caves to explore, while a random walk generates the actual cave systems. With various environmental features (e.g., platforms, holes, reservoirs) randomly added in, exploration will be fun and unique everytime!

## Midterm Goals

* Static level, acting as framework for procedural generation
* Basic physics engine (e.g., velocity, collision)
* Basic movement (e.g., left, right, jump)

## Final Goals

* Gameplay and Systems (20%)
	* Movement, jumping, and throwing (the acorn)
	* Win condition (reaching the bottom)
	* Player upgrades/power-ups
		* Required for progression deeper into the Earth
* Physics (30%)
	* Fluid dynamics (i.e., water, lava)
	* Flight dynamics (i.e., air)
* Procedural Generation (30%)
	* Binary space partitioning for overall map creation
	* Random walk to generate procedural cave systems
	* Environment features (e.g., platforms, reservoirs)

## Stretch Goals

* Temperature
	* Varying temperature impacts the environment (e.g., slipping, cold air, etc.)
* Procedurally generated puzzles
	* Like other features, the puzzles should be integrated with the other procedurally generated features
	* Aiming for a single puzzle that can be generated in multiple levels
