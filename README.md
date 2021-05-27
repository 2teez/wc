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
	 
If no `flag` is provided with no filename. The program drops into the standard input `stdin`. Then you can type in whatever you want, and when this is exited a temporary file `temp.txt` is created for the user. The user can then use `wc` to take several counts utilities that the `wc` provides.
	 
If only one file or more given, the progam displays all counts except for `number of characters` which is only avaliable via the flag `-c or --chars`.
	 
A character is defined by a single letter contained in a word.
	 
A line is defined as a string of characters delimited by a `newline` character. 

A word is defined as a string of characters delimited by white space characters.  

If wrong flag or flags are used, a default display is seen on the standard output. This is done intentionally to accommodate for user's mistype.  
	 

The following flags are available:

    -b | --bytes		Listing bytes count of the file or files
    -c | --chars		Listing character count of the file or files
    -h | --help		Help


##  Inspiration
From `wc` utility on *nix OS.
	
##  Caveat
1. While this rust program might not have been thoroughly tested like `wc` from the *nix OSes. It makes available `wc` provision on all OS that uses `@rust-lang`.
	
2. The flags were "hand-picked" i.e manually parsed, instead of using crate like `clap` or `structopt` and the rest. This is because the flags to be used are very little. Joke: Why kill an ant with a hammer or why kill at all :)! This may change later without affecting the functionality of the program. 
     
##  Not Included
The number of lines and other counts are displayed when the file or files are used on the `wc` program without any flag. So the flag `-l or line` is not included.

The total sum of all the counts are also not included just to make this `wc` different.

These are trivial display counters that might not "really" be needed to stand on their own in the programer's opinion. 
