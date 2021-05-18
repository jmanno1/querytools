# Query Tool

Simple tool to query websites using clipboard content and your keyboard!

This program creates a custom URL using  the provide `front`,  then your clipboard content, and
will add the `back` content if provided.

## How to use

Create a configuration TOML file to indicate how you want the program to work.
The config file can be made up of multiple config table entries.
Example:

```
copy_mode = true

[[config]]
    front = "https://www.google.com/search?q="
    keys = "LAlt+Key1"

[[config]]
    front = "https://www.bing.com/search?q="
    back = "%20sauce"
    keys = "LAlt+Key2"
```

If my clipboard contained the word `taco`, by pressing `left alt+1` it would do a google query for "taco".
If I preass `left alt+2` it would do a Bing query for "taco sauce".

### config file parts

At the top level, The following options are available:

- `copy_mode` - Optional, bool value, indicates if you want the program to copy highlighted text prior to doing the web query.

The following fields are avaiable for the `[[config]]` table:
- `front` - Required, this is the website URL that is the base of the query.
- `keys` - Required, these are the keys that trigger the web query.Use `+` as a delimiter between keys.
- `back` - Optional, this is content to be added at the end of the URL.

### Valid keynames

The following key names are valid for use in Key field.
**NOTE**: Keys are named for US keyboard layouts, international keyboard keys may not map correctly in all cases:

```
"Key1"
"Key2"
"Key3"
"Key4"
"Key5"
"Key6"
"Key7"
"Key8"
"Key9"
"A"
"B"
"C"
"D"
"E"
"F"
"G"
"H"
"I"
"J"
"K"
"L"
"M"
"N"
"O"
"P"
"Q"
"R"
"S"
"T"
"U"
"V"
"W"
"X"
"Y"
"Z"
"F1"
"F2"
"F3"
"F4"
"F5"
"F6"
"F7"
"F8"
"F9"
"F10"
"F11"
"F12"
"Escape"
"Space"
"LControl"
"RControl"
"LShift"
"RShift"
"LAlt"
"RAlt"
"Meta"
"Enter"
"Up"
"Down"
"Left"
"Right"
"Backspace"
"CapsLock"
"Tab"
"Home"
"End"
"PageUp"
"PageDown"
"Insert"
"Delete"
"Grave"
"Minus"
"Equal"
"LeftBracket"
"RightBracket"
"BackSlash"
"Semicolon"
"Apostrophe"
"Comma"
"Dot"
"Slash"
```
