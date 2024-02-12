# Codicefiscale

 A simple yet complete calculator for the italian fiscal code.

## Usage

This program comes with three commands, `generate`, `build-database` and `build-complete`. The `buld-complete` command build the autocompletition file for your shell to use with this program.
To calculate the code the program need a database with all the italian cities and all the nations is needed. To build one, download `gi_comuni.json` and `gi_nazioni.json` from [here](https://www.gardainformatica.it/database-comuni-italiani), place them in the directory of the executable and run the `build-database` command.
This will create and populate the database. The `generate` command is used to calculate the code. The command `codicefiscale help generate` will give this output:

```text
Generate the code

Usage: codicefiscale generate <NAME> <SURNAME> <SEX> <NATION> <CITY> <BIRTH_DATE> [SUBSTITUTION_DEPTH]

Arguments:
  <NAME>                Name
  <SURNAME>             Surname
  <SEX>                 Sex [possible values: m, f]
  <NATION>              Birth nation
  <CITY>                Birth city (irrelevant if nation is different from italy, but still needed)
  <BIRTH_DATE>          Birth date in format YYYY-MM-DD
  [SUBSTITUTION_DEPTH]  Substitution depth for homocodic code

Options:
  -h, --help     Print help
  -V, --version  Print version
```
