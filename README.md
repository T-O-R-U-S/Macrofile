# The why

I was unsatisfied with most macro software. Why? The GUI was too slow. Also I partially wanted to have fun.

### Note: Due to the library I'm using, Wayland display server (on Linux) is unsupported :(

# Inspiration

I was playing a game composed of mindless grinding. It was fun, but it was so brainless that it could be automated. Problem is, I wasn't satisfied with most macro software so now I'm making my own thing

# Aims

- Simplicity
- Development speed/conciseness

# Non-goals

- Ergonomics (This was designed only to be used by me). Due to this, you will find many unhelpful errors.
- Runtime speed. The game that inspired this tool is not dependent on speed at all.
- Good documentation. This was made to only be used by me. The docs will not be very good.
- Being actually good. The way things were designed in Shrimp3, there was a level of abstraction that meant that each function would behave the same given the same set of tokens. I didn't do that here (out of laziness) -- as a byproduct, some functions might not be able to accept variables.

# Usage

### Functions

- `inp` -- Simulates the typing of a string (enclosed in `"`)
- `kdn` -- Simulates holding a key down. Does not release until `kup` is called on the same key. Takes in one of the key-code aliases as an input (Specified below).
- `kup` -- Simulates the release of a key. Does not press down on any keys. Takes in one of the key-code aliases as input (specified below)
- `action` -- Specifies a function. 
##### e.g:
```
action mvfwd [
	kdn w
	# wait one second
	wait 1
	# release
	kup w
]

~mvfwd
```

Actions must be called via a special action tilde. They cannot be called like regular/built-in functions.

- `v` -- Specifies a variable. `v variable value`, access variables with `$`. `$variable`. Due to a design flaw in my language design, some functions might fail to accept variables. In cases like these, it is best to use actions instead of repeating code.

