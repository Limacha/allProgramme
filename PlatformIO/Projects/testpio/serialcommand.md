# Protocol Documentation

## Prefix

| Action                  | Abréviation | Hexa équivalent        |
|--------------------------|-------------|-------------------------|
| command                  | cmd         | `0x63 0x6D 0x64`        |
| manage tank motor        | mtm         | `0x6D 0x74 0x6D`        |

---

## Command

| Size (octets) | Prefix            | Params        |
|---------------|-------------------|---------------|
| `2 + 0 = 2`   | `0x64 0x6D` (demomotor) | *aucun* |

---

## Motor Manager

| Size (octets) | Command | Hexa | Params                                                                 |
|---------------|---------|------|------------------------------------------------------------------------|
| `1 + 0 = 1`   | reset         | `0x01` | *aucun*                                                                |
| `1 + (4-5) = 5-6` | move          | `0x02` | direction moteur R, vitesse moteur R, direction moteur L, vitesse moteur L, durée |
| `1 + (2-4) = 3-5` | moveDirection | `0x03` | direction, vitesse, relation, temps                                      |
| `1 + 2 = 3`   | changeSpeed   | `0x04` | speed, moteur(0x01 R, 0x02 L, 0x03 B)                                    |
| `1 + 2 = 3`   | changeDirection   | `0x05` | direction, moteur(0x01 R, 0x02 L, 0x03 B)                                    |
| `1 + 1 = 2`   | changeTankDirection   | `0x06` | direction                                                       |
