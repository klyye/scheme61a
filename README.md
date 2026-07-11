# Scheme
A rust implementation of the last 61A project.

## TODO
- let's not just copy that random blogpost and actually try to translate the 61a python into rust

### 7/10/26
Note that doing iterator.peekable().peek() actually ADVANCES the value in the original iterator. In other words, peekable is returning a 2nd peekable iterator by advancing the original iterator and storing its next() value, and since the call to peekable() isn't saved into a variable it gets throw away. Peekable() doesn't turn the original iterator peekable, it creates a new iterator that's peekable. Confusing!

- Finished Part 1 of the project spec (the reader) and the repl loop, onto the evaluator!

### 7/9/26
uhhhhhh i got lost in the sauce
- so parse_list, if you see elements in the list, shouldn't consume those from the buffer and should let parse_expr handle them
- but if you see closing parens, you should consume them. so what gives

### 7/8/26
- I now understand why the function signatures of the 61a skeleton are the way they are, the scheme S-expr grammar naturally pushes you towards a parse_expr and parse_list using an iterator
- actually implement the mutual recursion properly

### 7/7/26
- let the parser accept iterator
  - modify tests to reflect this, you can bring back the 'buffer' datastructure from the 61a tests
- might be worth going back over this https://rust-exercises.com/100-exercises/04_traits/10_assoc_vs_generic.html
- also find something non-distracting to listen to while doing this and STOP BROWSING MOXFIELD/EDHREC!!!
~~- standardize the tests a little, get rid of the expr helper funcs in favor of using modules and importing expr within the scope~~
  - also use the parse expr line helper everywhere

### 7/6/26
~~- Start with the lexer/tokenizer~~
  - learn the difference between a lexer and a tokenizer
- then do the parser
  - there should be an outer function for parsing full lines that just passes slices of it into the inner parser that only does 1 expr at a time

### 7/5/26
- ~~refactor tests not to use "current" and "pop" and instead just read from indices of string vectors~~
- think about whether enum types should [derive](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) traits