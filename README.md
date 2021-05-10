# Yet Another Lox Implementation In Rust

This is an implementation of [Lox language](https://github.com/munificent/craftinginterpreters) in Rust.  
Crafting Interpreters book by Bob Nystrom: https://www.craftinginterpreters.com

## What is Lox
Lox is a small dynamically typed language intended to be implemented for educational purposes.

Syntax examples (taken from book):
```lox
// Your first Lox program!
print "Hello, world!";

// variables
var a;
var b = a;

// branching
if (b == nil) {
  print "b is nil";
} else {
  print "b is something else";
}

// for loop
for (var a = 1; a < 10; a = a + 1) {
  print a;
}

// functions
fun addPair(a, b) {
  return a + b;
}

fun identity(a) {
  return a;
}

print identity(addPair)(1, 2); // Prints "3".

// class
class Breakfast {
  cook() {
    print "Eggs a-fryin'!";
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
```

### Features:
- data types: boolean, number (f64), string, nil (no value)
- arithmetics: + - * /
- comparison: < <= > >= ==
- logical operators: ! and or
- scopes { }
- variable assignment
- control flow: if-else while for
- first-class function support
- classes + inheritance
- a few functions in standard library
