# Introduction
This document describes a multithreaded architecture for a terminal-based game. It is designed to manage game logic, user input and rendering across multiple threads, each dedicated to specific tasks. The design emphasizes modularity, scalability, and responsiveness, leveraging inter-thread communication to coordinate between components.

The interface consists of dynamic terminal "windows," each responsible for rendering specific content. For example, one window displays the game's main output, while another handles debug information. A third and fourth could be added if there was a need to run 2 instances of a game or two entirely separate games side by side. The renderer dynamically adjusts these windows based on user actions, such as toggling visibility or resizing the terminal. An example can be seen below where one of the debug windows is hidden.

A basic wireframe of how the ui could look can be seen below:
![image](https://github.com/user-attachments/assets/005ceebb-439e-41ec-91f0-fa4f1ba2b24b)

In this example the communication and between threads would look something like this:
![image](https://github.com/user-attachments/assets/3b3a2002-3e15-4a9d-9c8e-2ab4dba2b2aa)


# Terminology
- Spawn: Initializing an instance of a struct and executing its core loop on a new thread.
- Send/Notify: Using `std::sync::mpsc::{Sender, Receiver}` to facilitate communication between threads.

# Functional Components
## 1. Coordinator Process

Responsibilities:

- Manages the lifecycle of all other processes.
- Ensures proper thread termination when the program exits.
- Spawns all other processes during initialization.
![image](https://github.com/user-attachments/assets/2be91573-5c61-4931-b8f6-598d3667e64e)

## 2. Input Listener

Responsibilities:

- Listens for user inputs such as:
    - Keyboard events
    - Mouse events
    - Terminal resize events
- Sends all captured events to every other process.
- Allows each process to handle only the events it is interested in, ignoring others.
![image](https://github.com/user-attachments/assets/b80511be-ac9d-4d45-8161-4b9e79fa6e72)

## 3. Renderer

Responsibilities:

- Manages terminal layout by tracking terminal size.
- Allocates display space to processes known as "windows," where:
    - A pixel corresponds to one terminal character.
    - A window refers to a process responsible for rendering content in a section of the terminal.
- Supports hiding windows, recalculating space for other windows, and notifying them of layout changes.
- Collects rendered data from all windows and outputs the final display to the terminal.
![image](https://github.com/user-attachments/assets/a1541dcf-20e4-4acc-b70a-18bd1f49d374)

## 4. Windows

Responsibilities:

- Receives data from the game process.
- Adjusts game data to fit the available window space before rendering it.
- Can be activated or deactivated by the game process.
![image](https://github.com/user-attachments/assets/5ccba192-c9aa-4910-b4b9-cd7fa8d9fb4a)

## 5. Game Process

Responsibilities:

- Implements the core game logic.
- In the current design, this process runs Conway's Game of Life with features such as:
    - Updating the game state every second.
    - Handling keyboard inputs to control game functions like pause, zoom, and restart.
- Sends the updated game state to attached windows for rendering.
- Sends debug information to the Debug Window and manages its visibility (activation/deactivation).
- Designed to be interchangeable to support other games in the future.
![image](https://github.com/user-attachments/assets/5e86c9f5-4163-4bfa-9409-44570419ebad)

# Analysis
## Inter-Process Communication
- Communication between processes is achieved via `std::sync::mpsc::{Sender, Receiver}`.
- Processes notify one another of events, data updates, or layout changes as required.
- The event-driven design ensures efficient, decoupled interaction among processes.

## Key Features
- Dynamic Layout Management:
    The renderer dynamically adjusts the terminal layout based on active windows and notifies windows of their allocated space.
- Extensibility:
    The modular design allows for easy replacement of the game logic or the addition of new windows and processes.
- Centralized Control:
    The Coordinator Process simplifies thread lifecycle management and ensures clean program shutdown.

## Considerations for Implementation
Ensure proper synchronization to avoid race conditions during layout updates or event propagation.
Implement error handling for scenarios like thread crashes or message queue overflows.
Optimize rendering and event propagation to prevent bottlenecks in performance-critical scenarios.

# Improvements and considerations

1. Potential Bottlenecks:
    - Renderer as a Bottleneck:
        The renderer collects data from all windows, recalculates layout, and renders the final output. If windows or the game produce data frequently, the renderer may become overwhelmed.
    - Suggestion: Introduce batching or rate-limiting for rendering updates.
2. Event Propagation Overhead:
    - The input listener broadcasts all events to all processes. If the number of processes increases, this could lead to inefficiencies.
    - Suggestion:
        - Implement event filtering in the input listener to reduce unnecessary messages.
        - Use a publish/subscribe model (e.g., topics for specific event types) instead of broadcasting.
3. Thread Explosion:
    - Each window and game instance spawns its own thread. For small applications, this is fine, but for complex games with multiple windows or subsystems, the thread count could grow unmanageable.
    - Suggestion: Use a thread pool for lightweight processes like windows, or leverage async runtimes (e.g., tokio) to avoid one thread per window.
4. Coordinator Single Point of Failure:
    - If the coordinator thread crashes, it might leave other threads running, leading to resource leaks.
    - Suggestion: 
        - Use `std::thread::JoinHandle` for all spawned threads and ensure graceful shutdown on errors.
        - Implement a heartbeat or health-check mechanism to monitor threads.
5. Window-Renderer Synchronization:
    - The renderer recalculates layout dynamically when a window is hidden. This recalculation could introduce inconsistencies if window state changes frequently.
    - Suggestion: Introduce a double-buffering mechanism to manage layout updates without interrupting the renderer's current operation.
6. Game State Notification:
    - The game sends updates to windows. If the game state changes frequently, windows might struggle to keep up with processing.
    - Suggestion: Implement throttling for game state updates or make updates conditional on changes.

## Specific Concerns About Interactions
1. Input Listener:
Does it buffer events? If not, high-frequency inputs could lead to dropped events or overwhelmed threads.

2. Thread Shutdown:

How does the coordinator ensure a clean shutdown? Does it signal all threads and wait for them to finish, or is there a risk of abrupt termination?

## Additional Considerations
1. Error Handling:
Consider how to handle errors in individual threads. For example, if the game thread crashes, does the coordinator attempt to restart it or shut down the application gracefully?

2. Testing:
Mock each process (e.g., a mock game or renderer) to simulate real-world conditions and identify bottlenecks or race conditions.

3. Logging:
Incorporate structured logging (including per-thread identifiers) to debug inter-thread communication issues.
