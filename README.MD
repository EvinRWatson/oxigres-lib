# Oxigres-Lib

Rust powered dynamic library for querying PostGRES databases. Created by [Evin Watson](https://github.com/EvinRWatson) & [Kyle Suter](https://github.com/ClosestExaminer).

## Overview
Oxigres is a dynamic library intended to query a PostGRES database using a C compatible query string as input and returning the data in a C compatible JSON array string. This was originally created to serve as an engine extension for GameMaker Studio. It works by reading database connection information stored in environment variables on the server which the DLL is being called from. It uses these variables to establish a connection to PostGRES, complete the query, and terminate the connection. Each query will be have it's own connection instance.

## Setup Steps
<ol>
    <li>Ensure your PostGRES database is set up
    <li>Download or Build the Dynamic Library
    <li>Download your PostGRES SSL Certificate
    <li>Setup Environment Variables
    <li>Call the DLL from your application
</ol>


### Environment Variables
<ul>
    <li>OXIGRES_DB_HOST - Host Name ex. `oxigres-database.postgres.database.azure.com`
    <li>OXIGRES_DB_PORT - Host Port ex. `5342`
    <li>OXIGRES_DB_USER - Username ex. `applicationuser`
    <li>OXIGRES_DB_PASSWD - User Password
    <li>OXIGRES_DB_NAME - Database Name ex. 'postgres'
    <li>OXIGRES_DB_CERT_PATH - Full path to your downloaded SSL Certificate
</ul>

### Build the DLL/DyLib/SO
Install the Rust toolchain. Clone the repository, and run the following terminal command from the oxigres-lib directory. The compiled dynamic library file (.dll/.dylib/.so) will be contained within the oxigres-lib/target/release folder.
~~~
cargo build --lib --release
~~~

## Example Query
Input Query:
~~~
SELECT * FROM test_table
~~~
Response:
~~~
[
        {
                "column1": 0,
                "column2": "data1"
        },
        {
                "column1": 1,
                "column2": "data2"
        }
]
~~~

