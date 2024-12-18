Program: 2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0

*Take the lowest 3 bits of the input*
`B <- A mod 8`

*flip lowest bit of B*
`B <- B ^ 1`

*Set C to A div by 2 ** B*
`C <- A / 2 ** B`

*XOR B and C*
`B <- B ^ C`

*flip the third bit of B *
`B <- B ^ 4`

*Slide A over by 3 bits*
`A <- A / 2 ** 3`

*Output the lowest byte of B*
`Out B`

*If A is nonzero then jump to the start (pos 0)*
`Jnz 0`


101xxx
  0  



