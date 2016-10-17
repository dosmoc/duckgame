My first Rust program, and first (evetually) game. Using [piston](http://www.piston.rs/). I want it to be a game where a momma duck picks up her baby ducks and brings them back to the nest. Right now I've got a rotating moving triangle! 

## General Goals
	- learn Rust
	- eventually learn idiomatic Rust 
	- learn the mechanics of making a game
	- eventually learn good abstractions for a game
	- eventually make a good game

## Game goals
	- [x] make a thing that can move in multiple directions
	- [ ] make the player duck a sprite instead of a triangle
	- [ ] add three, randomly placed baby ducks
	- [ ] let the momma get ducks
	- [ ] add a nest for the ducks to go back to and win the game
	- [ ] let the player keep playing
	- [ ] score?
	- [ ] acceleration / deceleration?

## Learning observations
	- I was expecting to worry more about memory management but so far haven't
	- I'm pretty sure I understand why concurrent mutation of self is should be avoided, but not sure if that's what rustc stopped me from trying to do 
	- Still hazy on reference semantics
	- Still hazy on borrowing
	- Thinking about how an player character can move is fun
	- Haven't had to think about cosine and sine, and radians in a long time. Cool!
	- Not sure if using enums correctly
	- Haven't needed thus far to think about iteration very much 
	- Not sure if using physics / math terms correctly
	- GIMP pallet parser?
	- Sound??
	- How to handle movement of baby ducks. Custom events? Let's think of it this way: Each duck can receive movement events which update its state. It doesn't matter what the source of the movement event is: some sort of emitter, or something that translates key presses.