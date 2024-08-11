<p align="center">
<img src='https://i.postimg.cc/d0WWH1MR/a2c68588eab85ad2459788d74bd36534.webp' border='0'/>
<h1 align="center"><u>Virtel</u>: Your best virtual enviroment</h1>
</p>

-------------
`❓` Virtel is the fastest, the most secure and safe virtual enviroment. Best suited for mathematical calculations, development or deployment of applications and servers, though can be used for any purpose. We call it the **operating system**.

`🎯` What is the point of Virtel? Our goal is pretty simple: instead of writing software for different platforms separately or adding compatibility with other OS(es) later, we want people to write programs via **Virtel** - this **allows your code to run on practically any machine**, as Virtel is specifically made to be compatible with most platforms.

--------------
### We need help!
`🫂` Virtel is currently in the stage of early development, which means that as for now, most of the desired functionality is not here yet. For that reason, we need **your help** to develop Virtel.

If you know the programming language **Rust**, you can help us to develop Virtel itself. At current stage we lack an **IDE to write programs that will run on Virtel**, and there aren't any programming languages **that would compile into <u>Steps</u>**.

-----------
### What is "Steps"?
`❓` **Steps** is a programming language made specifically to run on Virtel. As have been said before, Steps also lacks lots of functionality, but is constantly improved - creating programs in Steps isn't that far away from reality. The file extension that Steps uses is pretty straightforward: `.steps`. The same thing can be said about the language itself, as the syntax is pretty easy to learn. If you don't like Steps, in the near future you will be able to write code via another language that compiles into Steps.

------------
### Quick overview of Steps
`🔬` In the future, there will be a dedicated Steps documentation, where you will be able to read and see lots of examples. **This section exists only for the sole purpose of providing some basic information about the Steps language**, as the documentation **hasn't been written yet**.

`🛠️` List of currently available commands:
* `sys out {name of the variable or any text}`
* `var set {var name} {var value}`
* `var get {var name}`
* `var del {name of the variable}`
* `bin run {name of the .steps script}`
* `math plus {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math min {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math mult {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math div {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math exp {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math root {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math mod {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math floor {num as text or var name} {num as text or var name} {name of the var to store the result}`
* `math incr {name of the var to increment}`
* `math decr {name of the var to decrement}`

### Different code examples:
#### Basic example, working with `bin run`: 
`start.steps` is always a starting point for any Steps application. In this example we also jump to the `run.steps` file.

Path of the file (may depend on the system): `C:/Virtel/apps/vladceresna.virtel.launcher/bin/start.steps`

Script in `start.steps`:
```
var set greet "Hello!";
sys out greet;
var set greet "Hi!";
sys out greet;
var del greet;
sys out greet;
bin run run.steps;
```
Script in `run.steps`:
```
sys out "Hello from run";
sys out "This is run";
```

#### Another example, working with math commmands:
This example briefly shows how you can use mathematics in Steps.

Steps Code:
```
var set a 5;
var set b 5;
math plus a b c;
sys out c;
```
Output of the code:
```
10
```

#### Here's another mathematical example showing the root command:
```
math root "27" "3" c;
sys out c;
```
Outputs:
```
3
```

---------
### List of currently supported platforms:
* [x] Windows
* [ ] Linux
* [ ] Redox
* [ ] MacOS
* [ ] iOS
* [ ] Android
* [ ] Native (OS)
