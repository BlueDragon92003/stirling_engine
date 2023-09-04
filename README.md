# Stirling Engine

The Stirling Engine is a relatively simple video game engine that has the
following goals:

- **Moddability**: Mods are an integral part of video games. They allow players
    to customize their game to fit whatever experience they desire, and can
    breathe life into an old game. This engine was specifically designed to make
    creating easily moddable games very simple.
- **Durability**: Games are no fun when they crash for no reason. The Stirling
    Engine seeks to be uncrashable -- any crashes a game may experience will be
    entirely due to the logic of the game itself. In addition in-built hang
    detection automatically crashes a game instead of letting it sit frozen on
    the player's screen.
- **Speed**: Just like Rust, the Stirling Engine aims to be fast. Every in-built
    aspect of it is designed to complete as efficiently and as quickly as
    possible.
- **Usability**: It is not fun diving into a heavily complex library with no
    understanding of what is going on. the Stirling Engine thus seeks a level
    of usability that allows development without diving into the minutiae of
    video game development.
- **Customizability**: At the same time, those who are experienced developers
    that need maximum control over every aspect of their game should be allowed
    it. While a set of default shaders are used in 3D graphics, they can be
    overwritten for any game to create the exact visual landscape envisioned. If
    the default options are too slow or don't handle the use case properly, they
    can be ripped out and replaced with something else.

# Plan of Action (Release 0.0)
1. [ ] Game Loop (Accomplished through WGPU)
   - [X] Ticking System
   - [ ] 'Local' Game State
   - [X] Update Method
2. [ ] Basic Input
3. [ ] ECS Infrastructure
4. [ ] 2D Graphics
5. [ ] Asset Loading
   - [ ] Resources
   - [ ] Entities
6. [ ] Mod Manager
   - [ ] Mod Detection
   - [ ] Mod Loading
     - [ ] Manual Load Order
     - [ ] Dependancy Checking
   - [ ] Hooks (Custom Systems)
7. [ ] Game Data
   - [ ] Saving
   - [ ] Loading
8. [ ] Mouse Input
   - [ ] Clicks
   - [ ] Location (e.g. pont & click)

# Plan of Action (Release 1.0)
1. [ ] Networking Code
2. [ ] Extend Graphics
   - [ ] UI Graphics
   - [ ] 3D Graphics
3. [ ] Advanced Modding
   - [ ] Automatic Load Order
   - [ ] Custom Asset Loading
   - [ ] Custom Component Loading
   - [ ] Asset Overwriting
4. [ ] Sound System
5. [ ] Controller input

# Plan of Action (Future Releases)

Once release 1.0 is complete, any and all future releases will continue to work
towards the goals outlined at the top of this document.