# bitlist
rust cli app that shows the set bits of any binary, decimal, or hexadecimal number.  
Much like the bit toggling view of windows 10 calculator

## Usage:
``
bitlist <input type> <number>
``
## Examples:
``
$ bitlist -b 111110000
$ bitlist -h 1234abcde
$ bitlist -d 12344432
``
## Example output:
``
$ bitlist -d 12344432
      0    0    0    0      0    0    0    0      0    0    0    0       0     0    0    0
                    60                    56                    52                      48
      0    0    0    0      0    0    0    0      0    0    0    0       0     0    0    0
                    44                    40                    36                      32
      0    0    0    0      0    0    0    0      1    0    1    1       1     1    0    0
                    28                    24                    20                      16
      0    1    0    1      1    1    0    0      0    1    1    1       0     0    0    0
                    12                     8                     4                       0
``

