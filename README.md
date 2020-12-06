[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_wasm_pwa_minimal_clock

[comment]: # (lmake_cargo_toml_to_md start)

***version: 2020.1206.1830  date: 2020-12-06 authors: Luciano Bestia***  
**tutorial for a minimal example of rust wasm PWA**

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-63-green.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-6-blue.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-4-purple.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock/)

## Try it

<https://bestia.dev/rust_wasm_pwa_minimal_clock/>

## what to do?

First I need to decide what to do. It will be primarily a tutorial. But it will be also a useful piece of software. This makes it more interesting, motivating and fun.
\
The readme.md will have all the steps I took to make a finished app. And most of my inner monologue. Probably the readme.md will be much larger than the source code itself.
\
A **big digital clock** on the screen is nice to have. On any device. At least I want to have it. I have a big TV screen connected to my PC that is turned on all the time. Win10 has the clock on the taskbar, but it is miniscule and hard to read. There are a lot of applications with all types of clocks out there on the wild internet, but do you really want to install any unknown software this days? I fear malware, viruses, pandemics and all the rest. Web pages are trust-worthy, because they don't have access to 100% of my local machine, operating system and file system.
\
So it has to be a web page. I like the **rust programming language** and it is the best choice for **wasm/webassembly** that runs inside any modern browser on any platform. And it has great performance. 
\
A little of **HTML** is mandatory for the user interface. With a little of **CSS** to make it look fancy. This is than very easy to modify to suit everyone's personal taste and preferences.
\
Let's make it look like a native application with some simple boilerplate **PWA (progressive web app)** magic. It is supported by all the modern browsers in all the platforms. And the user experience is great.
\
I will try to make it **minimal** as possible so to have an easy starting point for learning about the subject.
\
It looks like a plan. Let's start.

## development environment

I work on a [lenovo ideapad flex 5 14](https://www.digitaltrends.com/laptop-reviews/lenovo-ideapad-flex-5-14-review/) and I am happy. Win10 is my primary OS, but all the programming is made inside **WSL2 (windows subsystem for linux)**, where I installed **Debian stretch**. 
\
My editor of choice is **VSCode** with plugins:
- Remote - WSL
- rust-analyzer
- rainbow brackets
- crates
- code spell checker
- vscode-icons


The rust toolchain installation is very easy for Debian on WSL, just follow the instructions how to use **rustup** on [rust-lang.org](https://www.rust-lang.org/tools/install).
\
For wasm compilation we need the tool [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
\
You probably found this file you read just now on [Github](https://github.com/LucianoBestia/rust_wasm_pwa_minimal_clock).**Github** (Microsoft) is for now the place for my repositories. 
\
And **Git** on Debian is logically the version control system for my source code. 
\
Licence is a thing today and the easiest one is the MIT licence. It basically means it is completely free of everything: money, profit, responsibility, liability and anything. It is a tutorial for everybody. Just use it.

## HTML, CSS

Let start with a simple static HTML and even simpler CSS files. Just to see if the web server  works. I like to have a separate **web_server_folder** and then **web_content_folder**, because of my previous complex web apps. It is a wise choice.
\ 
The code inside my 'index.html' and 'rust_wasm_pwa_minimal_clock.css' will evolve. But nothing too extravagant to not understand. Mainly boring boilerplate copied from somewhere.

## Development

We will need a **web file server** because browser security does not allow loading wasm modules from local file. 
\
Install this basic one:
`cargo install basic-http-server`  
Run the server in a separate terminal so it can stay running all the time:  
Go to the web content folder:  
`cd ~/rustprojects/rust_wasm_pwa_minimal_clock/web_server_folder/web_content_folder`  
Run the server:  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  
\
Ok. It should work. Just a static html+css for now.

## Rust wasm

Cargo.toml is very important to define the output as wasm library and the required dependencies to web-sys, js-sys and wasm-bindgen.
We need a src/lib.rs for our code. We start just with a simple wasm_bindgen_start function.
Compile the code with:  
`wasm-pack build --target web --release`
With a little luck we now have a pkg folder with all the goodies.
We must copy it to our web_content_folder. I like to use the rsync utility.
`sudo apt-get install rsync`  
`\rsync -a --delete-after pkg/ web_server_folder/web_content_folder/pkg/`  

Just refresh the browser and watch the F12 developer tools.

## PWA (progressive web app)

It is easy to make a simple PWA. We need to add some files and some code and it is done.  
- manifest.json 
- start_service_worker.js
- service_worker.js
- in the folder /images/ we need a lot of png files with different sizes of the app icon
- in the index.html header there is "A lot of metadata just for PWA"

When updating the files on the server, we must also update the app version number. It is in 2 places:
1. service_worker.js - in this constant: const CACHE_NAME = '2020.1206.1112'
2. Cargo.toml - in this line: version = "2020.1206.1112"

## cargo-make

It is boring to repeat the same commands every time we compile: change version number, build, copy pkg.
We can automate this with the utilities: cargo-make and the lmake_version_from_date.
`cargo install cargo-make`
`cargo install lmake_version_from_date`
In the file Makefile.toml we write the automation scripts.
Run  
`cargo make` - for help
Run
`cargo make release` - increment version in Cargo.toml and service_worker.js, build release version, copy pkg to content folder  

## rust code

Finally we come to the sweet spot. We have to write some code in rust. This is a very simple example.  
The wasm code starts with the function `wasm_bindgen_start()`.  
We `debug_write()` in the `F12 developer tools` console of the browser the name and version of the PWA, just for debug purposes.  
Then we `set_interval()` to execute the function `write_time_to_screen()` every 1 second.  
Wasm is using basically the same engine that javascript uses, so most of the javascript functions like `setInterval` are also accessible in rust via `js_sys` and `web_sys`, just the names are not equal, but similar enough. Great job Rust developers!  
We use some more of javascript functions: `Date::new_0()`, `get_hours()`,...
Finally we create a string with the HTML and then inject it into `div_for_wasm_html_injecting` with `set_inner_html()`.  
Done!

## Conclusion

Writing PWA with Rust is fairly easy. Just do it. Use this tutorial a minimal use case scenario. 
You can then set any HTML you can imagine. There is no end to the possibilities.  
There are many libraries to make the work with Rust and wasm easier.  
