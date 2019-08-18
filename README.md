Find Unicode
===

Find Unicode characters, the easy way!
A simple command line application to find unicode characters with minimum effort.

![alt tag](https://github.com/pierrechevalier83/find_unicode/blob/master/demo/demo.svg)

Installation
===

`cargo install fu`

Usage
===

* Run `fu` from your terminal:
```
fu
```

* Start typing. `fu` will show the unicode characters for which the description matches your query.
* If you're looking for a single character, hit Enter to select it and exit.
* If you're looking for multiple characters, hit Tab to select one and keep searching.
* By default, the searching expression is a regular expression.

Advanced usage
===

For more advanced configuration options, check out the help:
```
fu --help
```

```
fu 0.1.0
Pierre Chevalier <pierrechevalier83@gmail.com>

Find Unicode characters with ease.

Simply type a description of the character you are looking for. Once you found the character you were after, hit Enter.
Selecting multiple characters is also possible: hit tab to select a character and continue browsing.

USAGE:
    fu [OPTIONS] [initial_query]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --height <height>    Height of fu's window relative to the terminal window [default: 50%]
        --layout <layout>    Position of fu's window relative to the prompt [default: Below]  [possible values: Above,
                             Below]
        --search <search>    Search mode [default: Regex]  [possible values: Regex, Exact, Fuzzy]

ARGS:
    <initial_query>    Initial query, if any
```
