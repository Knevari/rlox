# Rlox

This is a Lox programming language interpreter written in Rust, I had been thinking about reading the Crafting Interpreters book by Robert Nystrom for some time, but going back to Java after so many years would only get me some terrible flashbacks I wouldn't mind looking back to. Since I'm learning Rust on my free time I decided to give it a go and translate the book's code to this amazing and developer friendly language instead \s :).

<hr>

I'll be updating this as I go.

#### Notes about scanning

The book considers only the line in which the given token is being read,
I don't like this too much, thinking about making my own version of the scanner which keeps track
of the line, row and column of a given token. Just a thought for now...
