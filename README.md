
# TODO this is just a skeleton!

# Data Rate Converter

This is a command-line application that converts data rates between various units.

It takes an input data rate (e.g., "100 kb/s"), an output size unit (e.g., "mb"), and an output time unit (e.g., "hr"), and prints the converted data rate to the console. It also provides a verbose mode for more descriptive output, and an option to specify the number of decimal places in the output.

## Features

*   Converts data rates between various units (bits, bytes, kibibits, kilobytes, etc.)
*   Supports time units (milliseconds, seconds, minutes, hours, days)
*   Provides a verbose output option for more detailed descriptions.
*   Allows you to specify the number of decimal places in the output.

## Usage

```bash
rateconv <INPUT_RATE> <OUTPUT_SIZE_UNIT> <OUTPUT_TIME_UNIT> [OPTIONS]
```
- INPUT_RATE: The data rate to convert (e.g., "64 kb/s").

- OUTPUT_SIZE_UNIT: The desired output size unit (e.g., "mb").

- OUTPUT_TIME_UNIT: The desired output time unit (e.g., "hr").

### Supported Data Size Units
The following data size units are supported:

**Bits**: `b`

**Kilobits**: `kb`

**Megabits**: `mb`

**Gigabits**: `gb`

**Bytes**: `B`

**Kilobytes**: `kB`

**Megabytes**: `MB`

Gigabytes: GB

Kibibits: kib

Mebibits: mib

Gibibits: gib

Kibibytes: KiB

Mebibytes: MiB

Gibibytes: GiB

### Supported Time Units
The following time units are supported:

Milliseconds: ms

Seconds: s

Minutes: m

Hours: hr

Days: d

## Options:

- -v, --verbose: Enable verbose output with descriptions of the units being used.

- -d <NUMBER>, --decimal-places <NUMBER>: The number of decimal places to use in the output.

## Examples
```bash
rate-conv "100 kb/s" mb hr
rate-conv -v "100 kb/s" mb hr
rate-conv "100 kb/s" mb hr -d 4
rate-conv "1024 KiB/s" mib s
```

## Installation
To build this application you will need to have the Rust toolchain installed.
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
