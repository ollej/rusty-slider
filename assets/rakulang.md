# Raku

An introduction to Raku programming language

---

## What is Raku?

 * A language definition
 * Multiple implementations
 * Not Perl 5
 * Used to be Perl 6
 * Released on December 25th 2015

---

## Rakudo

A compiler for Raku with multiple virtual machine backends.

---

## Backends

 * MoarVM
 * JVM
 * Parrot

---

## Rakudo Star

* A "useful" early adopter distribution
* Includes Rakudo and useful modules.

---

## Raku features

A short overview of new features.

---

## Grammar

 * Named regexes
 * New syntax
 * Grouping regexes with Grammar class
 * Decorators: regex, token, rule

---

## Command line scripts

Built in syntax for command line option parsing.

---

## Positional command line options

```raku
sub MAIN($x, $y) { ...  }
sub USAGE() {
    say "Usage: foo.pl <num1> <num2>";
}

# foo.pl 42 1337
```

---

## Named parameters

```raku
sub MAIN(Bool :$verbose) { ... }

# foo.pl --verbose
```

---

## Options with arguments

```raku
sub MAIN(:$foo = 'bar') { ... }

# foo.pl --foo=baz
```

---

## Gradual typing

```raku
method foo($bar) { ... }
```
vs
```raku
method foo(Str $bar) { ... }
```

---

## User defineable operators

```raku
'bar' ¨  $foo;
```
```raku
sub infix:<¨ >($method, $obj) {
    say $obj."$method"();
}
# $foo->bar();
```

---

## Threading

 * Implemented using promises and channels
 * Wraps implementations in different backends.

---

## Promises

A synchronization primitive.
```raku
my $kept_in_10 = Promise.in(10);
```

---

## Channels

Sending results between threads.

```raku
my $dest = Channel.new;
start {
    loop {
        $dest.send('foo');
    }
    $dest.close;
}
```

---

## Who should use Raku?

* Interested in language development
* Grammar parsing

---

# Olle Wreede

@ollej
