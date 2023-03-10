//! Tools for InfluxDB target.

use chrono::{TimeZone, Utc};
use clap::Parser;
use csv::Writer;
use influxdb_line_protocol::{self, EscapedStr, FieldValue};
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

/// InfluxDB command.
#[derive(Debug, Parser)]
pub struct InfluxCommand {
    #[clap(subcommand)]
    subcmd: InfluxSubcommand,
}

impl InfluxCommand {
    /// Run this command.
    pub fn run(self) {
        match self.subcmd {
            InfluxSubcommand::ToCsv(c) => c.run(),
        }
    }
}

/// Subcommands for InfluxDB.
#[derive(Debug, Parser)]
enum InfluxSubcommand {
    /// Line protocol to CSV.
    ToCsv(ToCsv),
}

/// Program to convert line protocol file to CSV file.
#[derive(Parser, Debug)]
struct ToCsv {
    /// Input line protocol file path.
    #[arg(short, long)]
    input: String,
    /// Output CSV file path.
    #[arg(short, long)]
    output: String,
}

impl ToCsv {
    fn run(self) {
        let input_file = File::open(&self.input).expect("Open line protocol file");
        let output_file = File::create(&self.output).expect("Open CSV file");

        line_protocol_to_csv(input_file, output_file);
    }
}

#[derive(Debug, Serialize)]
enum Value {
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    String(String),
    Boolean(bool),
}

impl From<EscapedStr<'_>> for Value {
    fn from(value: EscapedStr) -> Value {
        Value::String(value.into())
    }
}

impl From<FieldValue<'_>> for Value {
    fn from(value: FieldValue) -> Value {
        match value {
            FieldValue::I64(v) => Value::Int64(v),
            FieldValue::U64(v) => Value::UInt64(v),
            FieldValue::F64(v) => Value::Float64(v),
            FieldValue::String(v) => Value::String(v.into()),
            FieldValue::Boolean(v) => Value::Boolean(v),
        }
    }
}

fn line_protocol_to_csv<R: Read, W: Write>(source: R, dest: W) -> W {
    let mut reader = BufReader::new(source);
    let mut buffer = String::new();
    let mut writer = Writer::from_writer(dest);
    let mut row = Vec::new();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        let parsed_lines = influxdb_line_protocol::parse_lines(&buffer);
        for line in parsed_lines {
            let line = line.unwrap();

            if let Some(tag_set) = line.series.tag_set {
                for (_tagk, tagv) in tag_set {
                    row.push(Value::from(tagv));
                }
            }
            for (_fieldk, fieldv) in line.field_set {
                row.push(Value::from(fieldv));
            }
            if let Some(timestamp) = line.timestamp {
                let dt = Utc.timestamp_nanos(timestamp);
                row.push(Value::String(dt.to_rfc3339()));
            }
        }

        writer.serialize(&row).unwrap();

        buffer.clear();
        row.clear();
    }

    writer.into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_line_protocol_to_csv() {
        let input = "cpu,hostname=host_0,region=eu-central-1,datacenter=eu-central-1a,rack=6,os=Ubuntu15.10,arch=x86,team=SF,service=19,service_version=1,service_environment=test usage_user=58i,usage_system=2i,usage_idle=24i,usage_nice=61i,usage_iowait=22i,usage_irq=63i,usage_softirq=6i,usage_steal=44i,usage_guest=80i,usage_guest_nice=38i 1451606400000000000
cpu,hostname=host_1,region=us-west-1,datacenter=us-west-1a,rack=41,os=Ubuntu15.10,arch=x64,team=NYC,service=9,service_version=1,service_environment=staging usage_user=84i,usage_system=11i,usage_idle=53i,usage_nice=87i,usage_iowait=29i,usage_irq=20i,usage_softirq=54i,usage_steal=77i,usage_guest=53i,usage_guest_nice=74i 1451606400000000000
cpu,hostname=host_2,region=sa-east-1,datacenter=sa-east-1a,rack=89,os=Ubuntu16.04LTS,arch=x86,team=LON,service=13,service_version=0,service_environment=staging usage_user=29i,usage_system=48i,usage_idle=5i,usage_nice=63i,usage_iowait=17i,usage_irq=52i,usage_softirq=60i,usage_steal=49i,usage_guest=93i,usage_guest_nice=1i 1451606400000000000";
        let expect = "host_0,eu-central-1,eu-central-1a,6,Ubuntu15.10,x86,SF,19,1,test,58,2,24,61,22,63,6,44,80,38,1451606400000000000
host_1,us-west-1,us-west-1a,41,Ubuntu15.10,x64,NYC,9,1,staging,84,11,53,87,29,20,54,77,53,74,1451606400000000000
host_2,sa-east-1,sa-east-1a,89,Ubuntu16.04LTS,x86,LON,13,0,staging,29,48,5,63,17,52,60,49,93,1,1451606400000000000
";
        let output = line_protocol_to_csv(Cursor::new(input), Vec::new());
        assert_eq!(expect, String::from_utf8(output).unwrap());
    }
}
