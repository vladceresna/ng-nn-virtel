<p align="center">
<img src='https://i.postimg.cc/d0WWH1MR/a2c68588eab85ad2459788d74bd36534.webp' border='0'/>
<h1 align="center"><u>Virtel</u>: Your best virtual enviroment</h1>
</p>

<p align="center">
<img src="https://camo.githubusercontent.com/8e31ce4df532515ac9a1c0418c03b7793471ff9e282dfc28e6473b65334fbac9/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f727573742d2532333030303030302e7376673f7374796c653d666f722d7468652d6261646765266c6f676f3d72757374266c6f676f436f6c6f723d7768697465">
<img src="https://camo.githubusercontent.com/c292429e232884db22e86c2ea2ea7695bc49dc4ae13344003a95879eeb7425d8/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f57696e646f77732d3030373844363f7374796c653d666f722d7468652d6261646765266c6f676f3d77696e646f7773266c6f676f436f6c6f723d7768697465">
<img src="https://camo.githubusercontent.com/02ad0a4530e3be8b708a7793a641add82aa4df8dd9987cc50d5c38ba609ea382/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6d61632532306f732d3030303030303f7374796c653d666f722d7468652d6261646765266c6f676f3d6d61636f73266c6f676f436f6c6f723d463046304630">
<img src="https://camo.githubusercontent.com/7eefb2ba052806d8a9ce69863c2eeb3b03cd5935ead7bd2e9245ae2e705a1adf/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f4c696e75782d4643433632343f7374796c653d666f722d7468652d6261646765266c6f676f3d6c696e7578266c6f676f436f6c6f723d626c61636b">
<img src="https://camo.githubusercontent.com/4fc6f0767c98026d209ad89ee9f04ebcde55eb232393fa232a4df01d50f2ef00/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f694f532d3030303030303f7374796c653d666f722d7468652d6261646765266c6f676f3d696f73266c6f676f436f6c6f723d7768697465">
<img src="https://camo.githubusercontent.com/214d3c29a72c22fe498ea0f7d6d2cdbde23331791f97be24d817338c699084b5/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f416e64726f69642d3344444338343f7374796c653d666f722d7468652d6261646765266c6f676f3d616e64726f6964266c6f676f436f6c6f723d7768697465">
<img src="https://camo.githubusercontent.com/bb2913a71d7513370db93a85a313af402b1a41e3ceccb2a87f46847601c05cda/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f446973636f72642d2532333538363546322e7376673f7374796c653d666f722d7468652d6261646765266c6f676f3d646973636f7264266c6f676f436f6c6f723d7768697465">
</p>

-------------
`❓` Virtel is the fastest, the most secure and safe virtual enviroment. Best suited for mathematical calculations, development or deployment of applications and servers, though can be used for any purpose. We call it the **operating system**.

`🎯` What is the point of Virtel? Our goal is pretty simple: instead of writing software for different platforms separately or adding compatibility with other OS(es) later, we want people to write programs via **Virtel** - this **allows your code to run on practically any machine**, as Virtel is specifically made to be compatible with most platforms.

Please see last releases on Releases page and join our Discord server: https://discord.gg/hNSyTvuy2v .

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
* [x] Linux
* [ ] Redox
* [ ] MacOS
* [ ] iOS
* [ ] Android
* [ ] Native (OS)
