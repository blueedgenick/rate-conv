
# Data Rate Converter

This is a command-line application that converts data rates between various units.

It takes an input data rate (e.g., "100 kb/s"), an output data rate (e.g., "mb/hr"), and prints the converted data rate to the console. It also provides a verbose mode for more descriptive output, and an option to specify the number of decimal places in the output.

## Features

*   Converts data rates between various size and time units (bits, bytes, megabytes, per hour or minute, etc.)
*   Provides a verbose output option for more detailed descriptions. (`-v`)
*   Allows you to specify the number of decimal places in the output. (`-d2`)
*   This app attempts to be as permissive as possible in how you specify the desired size and time units so that you don't have to memorize or guess a precise syntax. For instance, the following are all accpetable forms of specifying "kilobytes":
    * "kB"
    * "KB"
    * "kBytes"
    * "KBytes"
* Similarly for time units:
    * "h"
    * "hr"
    * "hour"
* Probably the most important thing to note when specifying inputs is that a lower-case 'b' always denotes 'bits' whereas an upper-case 'B' always means 'bytes'.

## Usage

```bash
rateconv <INPUT_RATE> <OUTPUT_RATE> [OPTIONS]
```
- INPUT_RATE: The data rate to convert (e.g., "64 kb/s").

- OUTPUT_RATE: The desired output rate (e.g., "mb/hr").

### Supported Data Size Units
The following data size units are supported:

**Bits**: `b`

**Kilobits**: `kb`

**Megabits**: `mb`

**Gigabits**: `gb`

**Terabits**: `tb`

**Bytes**: `B`

**Kilobytes**: `KB`

**Megabytes**: `MB`

**Gigabytes**: `GB`

**Terabytes**: `TB`

**Kibibytes**: `KiB`

**Mebibytes**: `MiB`

**Gibibytes**: `GiB`

**Tebibytes**: `TiB`

### Supported Time Units
The following time units are supported:

**Milliseconds**: `ms`

**Seconds**: `s`

**Minutes**: `m`

**Hours**: `h`

**Days**: `d`

## Options:

- -v, --verbose: Enable verbose output with descriptions of the units being used.

- -d <NUMBER>, --decimal-places <NUMBER>: The number of decimal places to use in the output, default 2.

## Examples
```bash
rateconv 100 kb/s mb/h
rateconv -v 100 kb/s mbph
rateconv 100kbps mb/hr -d 4
rateconv "1024 KiB/sec" mib/s
```

## Installation
To build this application locally you will need to have the Rust toolchain installed.
Install Rust https://www.rust-lang.org/tools/install
Once you have Rust installed, you can clone this repository, and use cargo to build the application
```bash
git clone <repository-url>
cd rate-conv
cargo build --release
```
You can then run the binary located in `target/release/rate-conv`

## Example Usage
Here are a couple of screenshots demonstrating example usage:
### Basic Usage

![Basic Usage](path-to-basic-usage-screenshot.png)

### Verbose Output With Decimal Places

![Verbose Output](path-to-verbose-output-screenshot.png)
