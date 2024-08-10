# Virtel

Your safe, secure, speed and best virtual work environment. Best suited for math calculations, home use, development and deployment of common applications and servers. We call it the **operating system**.

Join our discord server: https://discord.gg/hNSyTvuy2v

Please see last releases on Releases page.

## Hello developer!
### If you know...

Rust, you can help to create:
- ide for writing programs to run on Virtel
- programming languages (and library registries, builder, etc) compilers that will compile programs to steps code

Nothing or any other language, you can help:
- with developing your own or someone else's software for Virtel.
- with writing documentation for Virtel

## Hello user!
### If you can run Virtel on your machine...

You can help:
- with testing and reporting about issues and bugs on platform

## Our target
Delete all software on windows/linux/android/etc, install Virtel, and feel awesome!

## Available platforms
- [x] Windows
- [ ] Linux
- [ ] Redox
- [ ] MacOS
- [ ] iOS
- [ ] Android
- [ ] Native (OS)

## Virtel modules and commands
- sys
  - out (your-text-or-var)
- var
  - set (your-text)
  - get (your-var)
  - del (your-var)
- bin
  - run (name-your-steps-file)
- math
  - plus (addition)
  - min (subtraction)
  - mult (multiplication)
  - div (division)
  - exp (exponentiation)
  - root (root)
  - mod (modulus)
  - floor (floor division)
  - incr (increment)
  - decr (decrement)

## Code samples

C:/Virtel/apps/vladceresna.virtel.launcher/bin/start.steps
```
var set greet "Hello!";
sys out greet;
var set greet "Hi!";
sys out greet;
var del greet;
sys out greet;
bin run run.steps;
```

C:/Virtel/apps/vladceresna.virtel.launcher/bin/run.steps
```
sys out "Hello from run";
sys out "This is run";
```


### Other using cases
```
var set a 5;
var set b 5;
math plus a b c;
sys out c;
```

In out:
```
10
```
Or:
```
math root "27" "3" c;
sys out c;
```

In out:
```
3
```
