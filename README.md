# md2hash

## About
This is a utility for calculating the MD2 hash of either a string of text 
provided as command-line arguments, or the contents of a file. Differentiation 
is provided via the `f` (for file) and `s` (for string) switches. 

## Why
I wanted to learn more about hashing functions, and I wanted to learn more 
about Rust. I combined these by implementing MD2 from scratch in Rust 
for my utility.

## Limitations
The major current limitation is that a file must be able to fit into 
memory in order for it to be hashed. I'm working on a solution that 
enables files of any size to be hashed, by reading blocks into a buffer 
and updating the hash context.
