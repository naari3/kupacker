# kupacker

Pack/Unpack kupack file

## usage

```
kupacker 0.1.0
naari3 <naari.named@gmail.com>
Pack/Unpack kupack file

USAGE:
    kupacker.exe <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    pack      Pack kupack file
    unpack    Unpack kupack file
```

### unpack

```
kupacker.exe-unpack 0.1.0
naari3 <naari.named@gmail.com>
Unpack kupack file

USAGE:
    kupacker.exe unpack [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <destination>        extraction destination path [default: dest]

ARGS:
    <INPUT>    input kufile path
```

### pack

```
kupacker.exe-pack 0.1.0
naari3 <naari.named@gmail.com>
Pack kupack file

USAGE:
    kupacker.exe pack [OPTIONS] <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input directory>        extracted audio directory [default: dest]

ARGS:
    <OUTPUT>    output kufile path
```

## example

```bash
kupacker unpack path/to/1.kupack
mv favorite_sound.wav dest/moaiGong1mb.wav
kupacker pack 1_ex.kupack
```