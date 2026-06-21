# Overview
This file is a lightweight map of the current source tree and the major responsibilities of each module.

# Manifest
- `src`
    - `entry` - Process startup. Waits for the target game modules, installs subscribers, and starts the WebUI server.
    - `battle` - Battle-state aggregation and event dispatch. `BattleContext` owns the live battle summary and turns subscriber events into packets for clients.
    - `export` - Export and CSV generation for battle results and related summary data.
    - `kreide` - Game integration layer and IL2CPP helpers.
        - `helpers` - Helper functions used by subscribers and battle logic.
        - `types` - IL2CPP-facing type definitions and bindings used by the game integration layer.
    - `logging` - Logging setup and routing for console and file output.
    - `models` - Shared data types used by subscribers, the battle context, and the server payloads.
        - `events` - Internal event types. Includes battle flow events and damage events.
        - `types` - Core game data types such as `Avatar`, `Enemy`, `Entity`, `Property`, `BattleStats`, `Skill`, and `TurnInfo`.
        - `packets` - Serialized server packets broadcast to clients, including connection, error, and battle event payloads.
    - `server` - Socket server and broadcast layer. Serves the client connection and emits packets to connected clients.
    - `subscribers` - Game hooks that observe runtime activity and forward it into the battle layer.
        - `battle` - Hooks for battle flow, damage, stat changes, wave/cycle updates, and lineup initialization.
