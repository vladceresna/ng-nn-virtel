# Virtel

Join our discord server: https://discord.gg/hNSyTvuy2v

Virtel modules and commands:
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

Code sample:

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


Other using cases:
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
