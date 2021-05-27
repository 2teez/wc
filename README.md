# wc
     
## Name

`wc` -- bytes, characters, words, lines count
	 
## Synopsis
     
    wc [Flags] <files...>
	 
     Flags: 
           -b | --bytes 
           -c | --chars 
           -h | --help  

## Description
     
The `wc` rust program displays the number of bytes, characters, words, lines count avaliable in the file or files given to it from the standard input. 
	 
If no `flag` is provided with no filename. The program drops into the standard input `stdin`. The can type into this whatever, and when this is exited a temporary file `temp.txt` is created for the user. The user can then use `wc` to take several counts utilities that the `wc` provides.
	 
If only file or files are given, the progam displays all counts expect for `number of characters` which is only avaliable via the flag `-c or --chars`.
	 
A character is defined by a single letter contained in a word.
	 
A line is defined as a string of characters delimited by a `newline` character. 

A word is defined as a string of characters delimited by white space characters.  
	 

The following flags are available:

    -b | --bytes		Listing bytes count of the file or files
    -c | --chars		Listing character count of the file or files
    -h | --help     Help


##  Inspiration
From `wc` utility on *nix OS.
	
##  Caveat
1. While this rust program might not be throughly test like `wc` from the *nix OSes. It makes avaliable `wc` provision on all OS that uses `@rust-lang`.
	
2. The flags were "hand-picked" i.e manually parsed, instead of using crate like `clap` or `structopt` and the rest. This is because the flag to be used are very little, why kill an ant with an harmer or why kill at all :)! This may change later without affect the function of the program.
     
