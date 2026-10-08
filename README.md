# RustyNet

Ein interaktives Kommandozeilen-Tool in Rust für Netzwerkwartung und -analyse.
RustyNet startet eine eigene Shell, in der Befehle wie `ping`, `trace` und `netcalc` ausgeführt werden können.

```
 ____               _             _   _        _
|  _ \  _   _  ___ | |_  _   _   | \ | |  ___ | |_
| |_) || | | |/ __|| __|| | | |  |  \| | / _ \| __|
|  _ < | |_| |\__ \| |_ | |_| |  | |\  ||  __/| |_
|_| \_\ \__,_||___/ \__| \__, |  |_| \_| \___| \__|
                         |___/
```

## Funktionen

| Befehl    | Beschreibung                                                        |
|-----------|---------------------------------------------------------------------|
| `ping`    | Pingt einen Host an (nutzt das System-`ping`)                       |
| `trace`   | Traceroute zu einer IP-Adresse, optional mit Hostnamen-Auflösung    |
| `netcalc` | Subnetzrechner: Netzinfos, Subnetting, Prüfen ob eine IP enthalten ist |
| `help`    | Zeigt die Hilfe aller Befehle an                                    |
| `clear`   | Leert den Bildschirm                                                |
| `exit`    | Beendet RustyNet                                                    |

## Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) mit Unterstützung für **Edition 2024** (Rust 1.85 oder neuer)
- **Windows**: `ping` und `trace` rufen das System-`ping` mit Windows-Parametern (`-n`, `-l`, `-w`, `-i`) auf.
  Für `trace --resolve` wird zusätzlich `nslookup` benötigt.
  `netcalc` funktioniert plattformunabhängig.

## Installation & Start

```bash
git clone https://github.com/lbrandstaetterhtl/RustyNet.git
cd RustyNet
cargo run --release
```

Nach dem Start erscheint der Prompt `>`, an dem Befehle eingegeben werden.
Mit `<befehl> --h` wird die Hilfe zu einem einzelnen Befehl angezeigt.

## Befehle im Detail

### `ping`

```
ping <ziel> [OPTIONEN]
```

| Option              | Beschreibung                    |
|---------------------|---------------------------------|
| `--count <anzahl>`  | Anzahl der Pings                |
| `--size <bytes>`    | Paketgröße in Bytes             |
| `--timeout <ms>`    | Timeout pro Antwort in ms       |

```
> ping google.com --count 4 --size 64 --timeout 1000
```

### `trace`

```
trace <ip> [OPTIONEN]
```

| Option           | Beschreibung                                   |
|------------------|------------------------------------------------|
| `--ttl <n>`      | Maximale Anzahl an Hops (Standard: 50)         |
| `--timeout <ms>` | Timeout pro Hop in ms (Standard: 2000)         |
| `--resolve`      | Hostnamen der einzelnen Hops auflösen          |

Das Ziel muss aktuell als IPv4-Adresse angegeben werden.

```
> trace 8.8.8.8
> trace 8.8.8.8 --ttl 20 --resolve
```

### `netcalc`

```
netcalc <netz/präfix> [OPTIONEN]
```

| Option              | Beschreibung                                         |
|---------------------|------------------------------------------------------|
| `--info`            | Zeigt alle Informationen zum Netz an                 |
| `--split <n>`       | Teilt das Netz in (mindestens) n gleich große Subnetze |
| `--prefix <länge>`  | Teilt das Netz anhand einer neuen Präfixlänge        |
| `--contains <ip>`   | Prüft, ob eine IP-Adresse im Netz liegt              |

```
> netcalc 10.10.15.0/24 --info
> netcalc 10.10.15.0/24 --split 4
> netcalc 10.10.15.0/24 --prefix 26
> netcalc 10.10.15.0/24 --contains 10.10.15.42
```

Beispielausgabe von `--info`:

```
  Network:         10.10.15.0/24
  Broadcast:       10.10.15.255
  Subnet mask:     255.255.255.0
  Wildcard:        0.0.0.255
  First host:      10.10.15.1
  Last host:       10.10.15.254
  Usable hosts:    254
```

## Projektstruktur

```
src/
├── main.rs           # Einstiegspunkt, Eingabeschleife
├── HandleCommand.rs  # Befehlsdefinitionen, Parsing der Eingabe und Argumente
├── Operate.rs        # Eigentliche Logik (ping, trace, netcalc)
├── Help.rs           # Hilfetexte der Befehle
└── Print.rs          # Banner und Ausgabeformatierung
```

## Abhängigkeiten

- [`clearscreen`](https://crates.io/crates/clearscreen) – plattformübergreifendes Leeren des Terminals
