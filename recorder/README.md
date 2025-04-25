# Network Recorder / Player

Task structures:

- Main - receive commands, output state and errors
- Recorder - receives packets from Infra outputs, stores in the DB per x period or y packets.
- Player - Fetch a series (amount of time) from DB and output via Infra
- Infra - per stream (socket) jobs

```mermaid
---
title: State of the Recorder
---
stateDiagram-v2
    [*] --> Uninitialised
    Uninitialised --> Ready: Create Recording
    Uninitialised --> Ready: Load Recording
    Ready --> Uninitialised: Close Recording
    Ready --> Recording: Record
    Ready --> Playing: Play
    Ready --> Ready: Seek
    Recording --> Ready: Stop
    Playing --> Ready: Stop
    Playing --> Ready: Rewind
    Playing --> Finished: Reached end or recording
    Finished --> Ready: Rewind
    Uninitialised --> [*]: Quit
```

```mermaid
---
title: Concurrent tasks
---
graph TD
    Main --> Recorder;
    Main --> Infra;
    Main --> Player;
```

```mermaid
sequenceDiagram
    Main ->> Recorder: stream/infra config
    Recorder ->> Infra: infra config
    Infra -->> Recorder: Store in DB
    Main ->> Player: file and infra
    Player ->> Main: State
```
