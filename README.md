# Umeboshi


<img src="./imgs/umeboshi.png" alt="umeboshi logo">

Umeboshi is japanese apricot pickles. I like :)

This application is something like Common Lisp REPL.

However, umeboshi is still under developing :)

## Usage

    $ cargo run

    umeboshi>> Hello world!
    Hello world!
    umeboshi>> (+ 1 2)
    3
    umeboshi>> (eq 4 (* 2 2))
    true
    umeboshi>> (define x 12)

    umeboshi>> (+ $x 18)
    30
    umeboshi>> (if (eq $x 12) Equal NotEqual)
    Equal

## Screenshot

<img src="./imgs/umeboshi-screenshot.png" alt="umeboshi screenshot">
