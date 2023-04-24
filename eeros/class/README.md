# EEROS Project during Class

## Safety-Properties

| # | Name           | R | G | 0 | 1 |
|---|----------------|---|---|---|---|
| 0 | System Off     |   |   |   |   |
| 1 | Shutting down  | x |   |   |   |
| 2 | Breaking       | x |   | x |   |
| 3 | Starting up    |   | x |   |   |
| 4 | Emergency Mode | x |   |   |   |
| 5 | Emergency Stop | x |   | x |   |
| 6 | System On      |   | x |   |   |
| 7 | Motors On      |   | x |   | x |
| 8 | System running |   | x |   | x |

* **Pause-Button**: Reset Emergency
* **Mode-Button**: Emergency

```mermaid
stateDiagram
    OFF : 0 - System Off
    STOP : 1 - Shutting Down
    BREAK : 2 - Breaking
    START : 3 - Starting up
    E : 4 - Emergency
    EB : 5 - Emergency Stop
    SYSON : 6 - System On
    PON : 7 - Motors On
    MOVE : 8 - System Moving

    [*] --> OFF
    OFF --> START : Starting
    STOP --> OFF : Shutdown

    BREAK --> STOP : Halted
    BREAK --> START : Resetted (Mode)

    E --> SYSON : Reset (Pause)
    E --> BREAK : Abort (Mode)
    EB --> E : Halted
    START --> SYSON : Started

    SYSON --> PON : MotorsOn
    SYSON --> E : Emergency (Pause)
    SYSON --> BREAK : Abort (Mode)
    PON --> SYSON : PowerOff
    PON --> E : Emergency (Pause)
    PON --> BREAK : Abort (Mode)

    PON --> MOVE : StartMoving
    MOVE --> PON : StopMoving
    MOVE --> EB : Emergency (Pause)
    MOVE --> BREAK : Abort (Mode)
```
