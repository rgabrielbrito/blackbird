# Blackbird
### Plan For READMEs
The project was built by using cargo workspaces. Because of this, the functionality for the application
has been separated into four crates, each of which support a different aspect of the simulation: the neural
network, the genetic algorithm, the simulation code, and the simulation bindings for wasm. There is also the 
javascript, where the functions are invoked. 

Given this separation, I'm thinking of having five READMEs. The README for the root directory of the project is going to 
be rather simple. It is going to contain explanations for what the project is, where the tutorial can be found, and the 
build instructions.

The other READMEs are going to provide descriptions for each of the crates. In this way, we're able to explain each of them
in a standalone manner rather than adding all of the explanations at the root level. I might want to consider adding
a README for the javascript as well, though it is more important for you to describe the Rust code and move on from there.

After you modify all of those, you probably want to add comments to the code. Don't overdo it, just explain snippets of code
that are unclear.
