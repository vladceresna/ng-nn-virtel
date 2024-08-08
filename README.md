# Virtel

Join our discord server: https://discord.gg/hNSyTvuy2v

Release notes:
Added new functional:
modules and commands:
- sys
  - out (your-text-or-var)
- var
  - set (your-text)
  - get (your-var)
  - del (your-var)
- bin
  - run (name-your-steps-file)


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
