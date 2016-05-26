mold2d
======

#### A simple 2d platformer game library in Rust

![demo](/assets/demo.gif "Mario demo")

It includes the main game library implemented on top of SDL and a demo game that is used to
test the performance and correctness of the library.

The library allows for message passing to handle complex events. Every game object can send messages to either the parent view or to other game objects. It can also process messages sent to it and return a response message. This allows for greater flexibility because the messages sent can be different for different types of games and the core library will still function, and greater simplicity because instead of every object containing mutable references to the other objects, they can just communicate using immutable messages.

The library also has convenient methods for loading levels from text files, animating sprites from a spritesheet, viewport handling, spatial partitioning (only quadtrees right now), collision detection, displaying fonts, raycasting, and managing game objects and the score.

TODO
----
* Improve collision detection
* Add more spatial partitioning types
* Improve performance
