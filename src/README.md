# Rust Cellular Automaton 
## Author: Hamza Mohammed

![Alt Text](cellular_automaton.png)


Table of Contents
Introduction
Dependencies
Program Structure
Functions
random_initial_state
apply_rules
update_grid
Main Function
Usage
Example Output
Conclusion

1. Introduction<a name="introduction"></a>
This Rust program simulates a one-dimensional cellular automaton. Cellular automata are discrete mathematical models that consist of a grid of cells that evolve over discrete time steps based on simple rules. In this program, the automaton is represented as a grid where each cell can have one of two states (0 or 1).

2. Dependencies<a name="dependencies"></a>
This program relies on two external crates for functionality:

rand: Used for random number generation.
image: Used for image processing and saving the automaton as an image.
To include these dependencies, add them to your Cargo.toml file as shown in the code.

3. Program Structure<a name="program-structure"></a>
The program consists of the following components:

Import statements for external crates (rand and image).
Function definitions for simulating the cellular automaton.
The main function that orchestrates the simulation.
User input for setting the number of cells, generations, and a random rule.
4. Functions<a name="functions"></a>
random_initial_state
This function generates a random initial state for the cellular automaton grid.

apply_rules
This function applies the automaton's rules to determine the state of a cell in the next generation based on its current state and the states of its neighbors.

update_grid
This function updates the entire grid based on the specified rule.

5. Main Function<a name="main-function"></a>
The main function serves as the entry point for the program. It:

Reads user input for the number of cells, generations, and a random rule.
Generates the initial state.
Calls update_grid to simulate the automaton's evolution.
Creates an image representing the automaton's final state and saves it as "cellular_automaton.png."
6. Usage<a name="usage"></a>
To use this program, follow these steps:

Compile the program using cargo build.
Run the program using cargo run.
Provide input for the number of cells, generations, and observe the generated image.
7. Example Output<a name="example-output"></a>
The program saves an image called "cellular_automaton.png" in the current directory, which visually represents the final state of the cellular automaton.

8. Conclusion<a name="conclusion"></a>
This Rust program demonstrates the simulation of a one-dimensional cellular automaton, providing a visual representation of the automaton's evolution over time. You can customize the automaton's behavior by modifying the initial conditions and rules.

Feel free to experiment with different settings to observe the emergent patterns and behaviors that cellular automata can exhibit.

