```markdown
mockery.rs 0.1.0
Louis Capitanchik <louis.capitanchik@launchbase.solutions>
Generate model data that can be inserted into a database

USAGE:
    mockery [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -s, --spec <SPEC_PATH>      
            Sets the spec file to use. By default, mockery will look for a 'spec.json' file in CWD, and will error if it
            can not be found
    -t, --type <OUTPUT_TYPE>    
            Sets the output type. This value defaults to CSV for higher compatibility and throughput [possible values:
            csv, json]

ARGS:
    <INPUT>     
            Sets the input file to use

    <OUTPUT>    
            Sets the output path. Setting to '-' will stream to stdout


To view help, use -h. For long form help, use --help
```