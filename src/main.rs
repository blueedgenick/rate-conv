//! # Data Rate Converter
//!
//! This is a command-line application that converts data rates between various units.
//!
//! It takes an input data rate (e.g., "100 kb/s"), an output size unit (e.g., "mb"), and
//! an output time unit (e.g., "hr"), and it prints the converted data rate to the console.
//! It also provides a verbose mode for more descriptive output.
//!
//! ## Usage
//!
//! ```bash
//! rateconv <INPUT_RATE> <OUTPUT_RATE>
//! ```
//!
//! For example:
//!
//! ```bash
//! drateconv "100 kb/s" mb/hr
//! ```
//!
//! Or, for more detailed output:
//! ```bash
//! rateconv -v "100 kb/s" mb/hr
//! ```
// mod error;
use core::fmt;
use nom::{
    bytes::complete::take_while,
    character::complete::{one_of, space0},
    combinator::map_res,
    error::{ErrorKind, FromExternalError, ParseError},
    number::complete::double,
    sequence::tuple,
    IResult,
};
use std::{
    f64,
    num::{ParseFloatError, ParseIntError},
    process::ExitCode,
    str::FromStr,
};
use structopt::StructOpt;

/// Represents a time unit, such as seconds, milliseconds, or hours
#[derive(Debug, PartialEq, Clone, Copy)]
enum TimeUnit {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

impl FromStr for TimeUnit {
    type Err = ConverterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ms" => Ok(TimeUnit::Millisecond),
            "s" | "sec" | "second" => Ok(TimeUnit::Second),
            "m" | "min" => Ok(TimeUnit::Minute),
            "h" | "hr" | "hour" => Ok(TimeUnit::Hour),
            "d" | "day" => Ok(TimeUnit::Day),
            _ => {
                println!("argh!!!!!");
                Err(ConverterError::TimeUnitParseError(s.to_string()))
            }
        }
    }
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_unit = match self {
            TimeUnit::Millisecond => "millisecond",
            TimeUnit::Second => "second",
            TimeUnit::Minute => "minute",
            TimeUnit::Hour => "hour",
            TimeUnit::Day => "day",
        };
        write!(f, "{}", base_unit)
    }
}

impl TimeUnit {
    fn convert_to_bits_per_second(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Millisecond => value * 1000.0,
            TimeUnit::Second => value,
            TimeUnit::Minute => value / 60.0,
            TimeUnit::Hour => value / 3600.0,
            TimeUnit::Day => value / (3600.0 * 24.0),
        }
    }

    fn convert_from_bits_per_second(&self, value: f64) -> f64 {
        match self {
            TimeUnit::Millisecond => value / 1000.0,
            TimeUnit::Second => value,
            TimeUnit::Minute => value * 60.0,
            TimeUnit::Hour => value * 3600.0,
            TimeUnit::Day => value * (3600.0 * 24.0),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DataSizeUnit {
    Bit,
    KiloBit,
    MegaBit,
    GigaBit,
    TeraBit,
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
    TeraByte,
    KibiByte,
    MebiByte,
    GibiByte,
    TebiByte,
}

impl FromStr for DataSizeUnit {
    type Err = crate::ConverterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "bits" | "Bits" => Ok(DataSizeUnit::Bit),
            "kb" | "kbits" | "KBits" => Ok(DataSizeUnit::KiloBit),
            "mb" | "mbits" | "MBits" => Ok(DataSizeUnit::MegaBit),
            "gb" | "gbits" | "GBits" => Ok(DataSizeUnit::GigaBit),
            "tb" | "tbits" | "TBits" => Ok(DataSizeUnit::TeraBit),
            "B" | "bytes" => Ok(DataSizeUnit::Byte),
            "kB" | "KB" | "kBytes" | "KBytes" => Ok(DataSizeUnit::KiloByte),
            "mB" | "MB" | "mBytes" | "MBytes" => Ok(DataSizeUnit::MegaByte),
            "gB" | "GB" | "gBytes" | "GBytes" => Ok(DataSizeUnit::GigaByte),
            "tB" | "TB" | "tBytes" | "TBytes" => Ok(DataSizeUnit::TeraByte),
            "kiB" | "KiB" | "kibiBytes" | "KibiBytes" => Ok(DataSizeUnit::KibiByte),
            "miB" | "MiB" | "mebiBytes" | "MebiBytes" => Ok(DataSizeUnit::MebiByte),
            "giB" | "GiB" | "gibiBytes" | "GibiBytes" => Ok(DataSizeUnit::GibiByte),
            "tiB" | "TiB" | "tebiBytes" | "TebiBytes" => Ok(DataSizeUnit::TebiByte),
            _ => Err(ConverterError::DataSizeUnitParseError(s.to_string())),
        }
    }
}

impl fmt::Display for DataSizeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_unit = match self {
            DataSizeUnit::Bit => "bit",
            DataSizeUnit::KiloBit => "kilobit",
            DataSizeUnit::MegaBit => "megabit",
            DataSizeUnit::GigaBit => "gigabit",
            DataSizeUnit::TeraBit => "terabit",
            DataSizeUnit::Byte => "byte",
            DataSizeUnit::KiloByte => "kilobyte",
            DataSizeUnit::MegaByte => "megabyte",
            DataSizeUnit::GigaByte => "gigabyte",
            DataSizeUnit::TeraByte => "terabyte",
            DataSizeUnit::KibiByte => "kibibyte",
            DataSizeUnit::MebiByte => "mebibyte",
            DataSizeUnit::GibiByte => "gibibyte",
            DataSizeUnit::TebiByte => "tebibyte",
        };
        write!(f, "{}", base_unit)
    }
}

impl DataSizeUnit {
    fn convert_to_bits(&self, value: f64) -> f64 {
        match self {
            DataSizeUnit::Bit => value,
            DataSizeUnit::KiloBit => value * 1_000.0,
            DataSizeUnit::MegaBit => value * 1_000_000.0,
            DataSizeUnit::GigaBit => value * 1_000_000_000.0,
            DataSizeUnit::TeraBit => value * 1_000_000_000_000.0,
            DataSizeUnit::Byte => value * 8.0,
            DataSizeUnit::KiloByte => value * 8_000.0,
            DataSizeUnit::MegaByte => value * 8_000_000.0,
            DataSizeUnit::GigaByte => value * 8_000_000_000.0,
            DataSizeUnit::TeraByte => value * 8_000_000_000_000.0,
            DataSizeUnit::KibiByte => value * (8 * 1024) as f64,
            DataSizeUnit::MebiByte => value * (8 * 1024 * 1024) as f64,
            DataSizeUnit::GibiByte => value * (8f64 * 1024f64 * 1024f64 * 1024f64),
            DataSizeUnit::TebiByte => value * (8f64 * 1024f64 * 1024f64 * 1024f64 * 1024f64),
        }
    }

    fn convert_from_bits(&self, value: f64) -> f64 {
        match self {
            DataSizeUnit::Bit => value,
            DataSizeUnit::KiloBit => value / 1_000.0,
            DataSizeUnit::MegaBit => value / 1_000_000.0,
            DataSizeUnit::GigaBit => value / 1_000_000_000.0,
            DataSizeUnit::TeraBit => value / 1_000_000_000_000.0,
            DataSizeUnit::Byte => value / 8.0,
            DataSizeUnit::KiloByte => value / 8_000.0,
            DataSizeUnit::MegaByte => value / 8_000_000.0,
            DataSizeUnit::GigaByte => value / 8_000_000_000.0,
            DataSizeUnit::TeraByte => value / 8_000_000_000_000.0,
            DataSizeUnit::KibiByte => value / (8 * 1024) as f64,
            DataSizeUnit::MebiByte => value / (8 * 1024 * 1024) as f64,
            DataSizeUnit::GibiByte => value / (8f64 * 1024f64 * 1024f64 * 1024f64),
            DataSizeUnit::TebiByte => value / (8f64 * 1024f64 * 1024f64 * 1024f64 * 1024f64),
        }
    }
}

/// Describes a data rate with a given size and time unit.
#[derive(Debug, PartialEq)]
struct DataRate {
    // quantity: f64,
    size_unit: DataSizeUnit,
    time_unit: TimeUnit,
}

impl DataRate {
    #[allow(dead_code)] //used by tests
    const fn new(size_unit: DataSizeUnit, time_unit: TimeUnit) -> DataRate {
        Self {
            size_unit,
            time_unit,
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ConverterError {
    // #[error("Error parsing input rate ")]
    // ParseInputRateError,
    #[error("Error parsing number")]
    ParseNumberError,
    #[error("Could not parse the provided data size unit {0:?}")]
    DataSizeUnitParseError(String),
    #[error("Could not parse the provided time unit {0:?}")]
    TimeUnitParseError(String),
    //TODO this could provide the actual attempted value and rates?
    #[error("Could not convert to the requested output unit")]
    ConversionError,
    #[error("One or more required input arguments missing")]
    MissingArguments,
    #[error("Could not parse {0:?} via Nom({1:?})")]
    NomError(String, ErrorKind),
}

impl From<ParseIntError> for ConverterError {
    fn from(_value: ParseIntError) -> Self {
        Self::ParseNumberError
    }
}

impl From<ParseFloatError> for ConverterError {
    fn from(_value: ParseFloatError) -> Self {
        Self::ParseNumberError
    }
}

impl ParseError<&str> for ConverterError {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        ConverterError::NomError(input.to_owned(), kind)
    }

    fn append(input: &str, kind: ErrorKind, _other: Self) -> Self {
        ConverterError::NomError(input.to_owned(), kind)
    }
}

impl<'a, T> FromExternalError<&'a str, T> for ConverterError {
    fn from_external_error(input: &'a str, kind: ErrorKind, _e: T) -> Self {
        ConverterError::NomError(input.to_owned(), kind)
    }
}

fn parse_data_size_unit(input: &str) -> IResult<&str, DataSizeUnit, ConverterError> {
    let (input, unit_str) =
        take_while(|c: char| c.is_ascii_alphabetic() && !(c.eq_ignore_ascii_case(&'p')))(input)?;
    let unit_val = DataSizeUnit::from_str(unit_str);
    match unit_val {
        Ok(valid_unit_val) => Ok((input, valid_unit_val)),
        Err(e) => Err(nom::Err::Error(e)),
    }
}

fn parse_time_unit(input: &str) -> IResult<&str, TimeUnit, ConverterError> {
    map_res(take_while(|c: char| c.is_ascii_alphabetic()), |unit| {
        TimeUnit::from_str(unit)
    })(input)
}

///Parses a string of the form "kb/s" into a `DataRate` struct
fn parse_data_rate(input: &str) -> IResult<&str, DataRate, ConverterError> {
    let (input, (_, size_unit, _per, time_unit)) =
        tuple((space0, parse_data_size_unit, one_of("p/"), parse_time_unit))(input)?;
    Ok((
        input,
        DataRate {
            size_unit,
            time_unit,
        },
    ))
}

///Parses a supplied string into a decimal quantity and a `DataRate` struct
fn parse_input_rate(input: &str) -> IResult<&str, (f64, DataRate), ConverterError> {
    let (input, (_, qty, rate_unit)) = tuple((space0, double, parse_data_rate))(input)?;
    Ok((input, (qty, rate_unit)))
}

fn convert_data_rate(quantity: f64, rate: &DataRate, target_rate: &DataRate) -> Option<f64> {
    let value_in_bits = rate.size_unit.convert_to_bits(quantity);
    dbg!(value_in_bits);
    let value_in_bits_per_second = rate.time_unit.convert_to_bits_per_second(value_in_bits);
    dbg!(value_in_bits_per_second);

    let converted_value_in_bits = target_rate
        .size_unit
        .convert_from_bits(value_in_bits_per_second);
    dbg!(converted_value_in_bits);
    let converted_value = target_rate
        .time_unit
        .convert_from_bits_per_second(converted_value_in_bits);
    dbg!(converted_value);

    Some(converted_value)
}

fn unwrap_nom_error<T>(
    nom_res: IResult<&str, T, ConverterError>,
) -> std::result::Result<(&str, T), ConverterError> {
    match nom_res {
        Ok(v) => Ok(v),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => panic!("Nom Incomplete error"),
    }
}

//TODO move fn onto the enum? also re-use from `format_output` so we don't amange plurals twice
fn describe_data_rate(rate: &DataRate, plural: bool) -> String {
    if plural {
        format!("{}s per {}", rate.size_unit, rate.time_unit)
    } else {
        format!("{} per {}", rate.size_unit, rate.time_unit)
    }
}

///Formats the output based on the defined precision, and the size and time units
fn format_output(converted_qty: f64, rate: &DataRate, decimal_places: usize) -> String {
    format!(
        "{:.precision$} {:?}/{:?}",
        converted_qty,
        rate.size_unit,
        rate.time_unit,
        precision = decimal_places
    )
}

/// The command-line arguments for the application
#[derive(Debug, StructOpt)]
#[structopt( //TODO add note about upper/lower case 'b' etc
    name = "data-rate-converter",
    about = "Converts data rates e.g. `56 kb/s` between different size and time units"
)]
struct Opt {
    /// The data rate to convert (e.g. 64 kb/s)
    #[structopt(name = "INPUT_RATE")]
    input_rate: String,

    /// The desired output size and time units (e.g., mb/sec)
    #[structopt(name = "OUTPUT_RATE", default_value = "kB/s")]
    output_rate: String,

    ///Enable verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// The number of decimal places in the output.
    #[structopt(short = "d", long = "decimals", default_value = "2")]
    decimal_places: usize,
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), ConverterError> {
    let opt = Opt::from_args(); //aborts program on failure
    if opt.input_rate.is_empty() || opt.output_rate.is_empty() {
        //TODO tests for this
        return Err(ConverterError::MissingArguments);
    }
    let (_, (data_quantity, input_data_rate)) =
        unwrap_nom_error(parse_input_rate(&opt.input_rate))?;

    let (_, output_rate) = unwrap_nom_error(parse_data_rate(&opt.output_rate))?;

    let converted_rate = convert_data_rate(data_quantity, &input_data_rate, &output_rate)
        .ok_or(ConverterError::ConversionError)?;

    if opt.verbose {
        let input_rate_desc = describe_data_rate(&input_data_rate, data_quantity != 1.0);
        let output_rate_desc = describe_data_rate(&output_rate, converted_rate != 1.0);

        println!(
            "{} {} is equivalent to {:.precision$} {}",
            data_quantity,
            input_rate_desc,
            converted_rate,
            output_rate_desc,
            precision = opt.decimal_places
        );
    } else {
        println!(
            "Converted rate: {}",
            format_output(converted_rate, &output_rate, opt.decimal_places)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use DataSizeUnit::*;
    use TimeUnit::*;

    /// Round an f64 value to a specific number of significant digits
    fn precision_f64(x: f64, decimals: u32) -> f64 {
        if x == 0. || decimals == 0 {
            0.0
        } else {
            let shift = decimals as i32 - x.abs().log10().ceil() as i32;
            let shift_factor = 10_f64.powi(shift);

            (x * shift_factor).round() / shift_factor
        }
    }

    #[rstest]
    #[case::nnn_kb_s("123 kb/s", 123.0, DataRate::new(KiloBit, Second))]
    #[case::nnn_kbph("123 kbph", 123.0, DataRate::new(KiloBit, Hour))]
    #[case::nnnn_mb_hr("1024 MB/hr", 1024.0, DataRate::new(MegaByte, Hour))]
    #[case::nnnn_mib_h("1024 MiB/h", 1024.0, DataRate::new(MebiByte, Hour))]
    #[case::nnnn_mb_hr_no_spaces("1024MB/hr", 1024.0, DataRate::new(MegaByte, Hour))]
    #[case::nnnn_mb_hr_leading_spaces("   1024 MB/hr", 1024.0, DataRate::new(MegaByte, Hour))]
    #[case::zero_b_ms("0 b/ms", 0.0, DataRate::new(Bit, Millisecond))]
    #[case::nnn_point_mmm_kb_d("123.456 kB/d", 123.456, DataRate::new(KiloByte, Day))]
    #[case::zero_point_mmm_kb_d("0.456 kB/d", 0.456, DataRate::new(KiloByte, Day))]
    #[case::point_mmm_kb_d(".456 kB/d", 0.456, DataRate::new(KiloByte, Day))]
    #[case::nnn_point_zero_kb_d("123.0 kB/d", 123.0, DataRate::new(KiloByte, Day))]
    fn test_parse_data_rate(#[case] input: &str, #[case] qty: f64, #[case] rate: DataRate) {
        let (_, parsed_rate) = parse_input_rate(input).unwrap();
        assert_eq!(parsed_rate.0, qty);
        assert_eq!(parsed_rate.1, rate);
    }

    #[rstest]
    #[case::invalid_unit_no_digits("abc kb/s")]
    #[case::missing_per("123 kbs")]
    #[case::invalid_time_unit("123 kb/abc")]
    #[case::invalid_value_some_digits("123abc kb/s")]
    #[case::missing_value("kb/s")]
    #[case::invalid_per("123 kb s")]
    #[case::invalid_per_char("123 kbXs")]
    fn test_parse_data_rate_error(#[case] input: &str) {
        assert!(parse_input_rate(input).is_err());
    }

    #[rstest]
    #[case::kbit_s_to_mbit_s(
        1000.0,
        DataRate::new(KiloBit, Second),
        DataRate::new(MegaBit, Second),
        1.0
    )]
    #[case::gb_hr_to_gb_s(
        1.0,
        DataRate::new(GigaByte, Hour),
        DataRate::new(GigaByte, Second),
        1.0 / 3600.0)]
    #[case::mib_min_to_gib_hr(
        500.0,
        DataRate::new(MebiByte, Minute),
        DataRate::new(GibiByte, Hour),
        500.0*60.0/1024.0
    )]
    #[case::mbit_min_to_gbit_hr(
        500.0,
        DataRate::new(MegaBit, Minute),
        DataRate::new(GigaBit, Hour),
        30.0
    )]
    #[case::kbit_ms_to_kbit_s(
        1000.0,
        DataRate::new(KiloBit, Millisecond),
        DataRate::new(KiloBit, Second),
        1_000_000.0
    )]
    #[case::byte_day_to_byte_sec(
        1.0,
        DataRate::new(Byte, Day),
        DataRate::new(Byte, Second),
        1.0 / (3600.0 * 24.0)
    )]
    fn test_convert_data_rate(
        #[case] qty: f64,
        #[case] input_rate: DataRate,
        #[case] output_rate: DataRate,
        #[case] expected: f64,
    ) {
        let result = convert_data_rate(qty, &input_rate, &output_rate).unwrap();
        // some tests are only accurate to around 6 decimal places due to floating point inaccuracies with very large or small values so we'll round them off a bit before testing equality here
        let rounded_result = precision_f64(result, 6);
        let rounded_expect = precision_f64(expected, 6);
        assert_eq!(rounded_result, rounded_expect);
    }

    #[test]
    fn test_decimal_output_format() {
        let result = convert_data_rate(
            1.0,
            &DataRate::new(GigaByte, Hour),
            &DataRate::new(GigaByte, Second),
        )
        .unwrap();

        assert_eq!(
            format!("{:.1$}", result, 6),
            format!("{:.1$}", (1.0 / 3600.0), 6)
        );
    }

    #[test]
    fn test_convert_data_rate_error() {
        //TODO test something more useful here?
        assert!(convert_data_rate(
            1000.0,
            &DataRate::new(KiloBit, Second),
            &DataRate::new(Bit, Hour)
        )
        .is_some());
        assert!(convert_data_rate(
            1000.0,
            &DataRate::new(KiloBit, Second),
            &DataRate::new(Byte, Hour)
        )
        .is_some());
    }

    #[test]
    fn test_parse_units() {
        assert_eq!(parse_data_size_unit("mb").unwrap().1, DataSizeUnit::MegaBit);
        assert_eq!(parse_time_unit("s").unwrap().1, TimeUnit::Second);
        assert!(parse_data_size_unit("abc").is_err());
        assert!(parse_time_unit("xyz").is_err());
    }
}
