# TSDB Tools

My Tools for playing with TSDBs.

## Usage
Subcommands:
```
Usage: tsdb-tools <COMMAND>

Commands:
  influx  Subcommand for InfluxDB target
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Influx
Playing with InfluxDB's [line protocol](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/).
```
Usage: tsdb-tools influx <COMMAND>

Commands:
  to-csv    Line protocol to CSV
  from-csv  CSV to line protocol
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Converting line protocol file into CSV file.
```
tsdb-tools influx to-csv -i /path/to/line-protocol-file.lp -o ./path/to/csv-file.csv
```

Converting CSV file to line protocol file.
```
tsdb-tools influx from-csv -i /path/to/csv-file.csv -o /path/to/line-protocol-file.lp
```
