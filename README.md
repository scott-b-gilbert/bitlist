# bitlist
rust cli app that shows the set bits (in red) of any binary, decimal, or hexadecimal number.  
This is meant to look like the bit toggling view of windows 10 calculator

## Usage:
```
bitlist <input type> <number>
```
## Examples:
```
$ bitlist -b 111110000
$ bitlist -h 1234abcde
$ bitlist -d 12344432
```
## Example output:
![Screenshot](example.png?raw=true "output example")
