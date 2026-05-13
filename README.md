# rs-temp

`rs-temp` is a simple yet powerful command-line utility for monitoring CPU and NVIDIA GPU temperatures on Linux systems. It provides multiple output formats, including detailed human-readable, compact short-form, and machine-readable JSON.

## Features

- **CPU Temperature Monitoring**: Displays overall and per-core CPU temperatures.
- **NVIDIA GPU Temperature Monitoring**: Displays temperature for each detected NVIDIA GPU.
- **Flexible Output Formats**:
    - **Default**: A detailed, easy-to-read format for terminals.
    - **Short**: A compact, single-line output suitable for status bars (e.g., `polybar`, `waybar`).
    - **JSON**: Machine-readable output for scripting and integration with other tools.
- **Device Filtering**: Option to display temperatures for only CPU, only GPU, or both.

## Dependencies

This tool relies on the following system components and libraries:

- **`sysfs`**: For reading CPU temperature data. Ensure your system exposes this information.
- **NVIDIA Management Library (NVML)**: For reading GPU temperature data. This requires the official NVIDIA drivers to be installed.

## Installation

1.  Ensure you have a Rust development environment installed. If not, follow the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  Clone the repository:
    ```sh
    git clone https://github.com/tugapse/rs-temp.git
    cd rs-temp
    ```
3.  Build the project in release mode:
    ```sh
    cargo build --release
    ```
4.  The executable will be located at `target/release/rs-temp`. You can copy this to a directory in your system's `PATH`, such as `/usr/local/bin`:
    ```sh
    sudo cp target/release/rs-temp /usr/local/bin/
    ```

## Usage

The tool is controlled via command-line flags.

### Command-Line Options

| Flag (Long) | Flag (Short) | Argument      | Description                                          |
|-------------|--------------|---------------|------------------------------------------------------|
| `--json`    | `-j`         |               | Output temperature data in JSON format.              |
| `--short`   | `-s`         |               | Output a compact, single-line summary.               |
| `--device`  | `-d`         | `cpu` or `gpu`| Filter to show data for only the specified device. |
| `--help`    | `-h`         |               | Display the help message.                            |
| `--version` | `-V`         |               | Display the application version.                     |

### Examples

**Default Output (CPU & GPU)**
```sh
rs-temp
```
```
CPU Overall Temp: 45.1C
-------------------------------------------------------
Core 1          42.0C    Core 2          43.0C
Core 3          41.0C    Core 4          44.0C
...
-------------------------------------------------------
NVIDIA GeForce RTX 3080    55.2C
```

**Short Output**
```sh
rs-temp --short
```
```
CPU: 52.0C | GPUs: [43.0C, 27.8C]
```

**JSON Output for GPU Only**
```sh
rs-temp --device gpu --json
```
```json
{
  "gpu": {
    "timestamp": "2026-05-13T18:15:19.526534267Z",
    "gpus": [
      {
        "label": "NVIDIA GeForce RTX 4060 Laptop GPU",
        "current": 44.0,
        "high": null,
        "critical": null
      },
      {
        "label": "Intel UHD Graphics",
        "current": 27.8,
        "high": 27.8,
        "critical": null
      }
    ]
  }
}

```

**Get CPU temperature in short format**
```sh
rs-temp -d cpu -s
```
```
CPU: 53.0C
```