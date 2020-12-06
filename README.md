[comment]: # (lmake_md_to_doc_comments segment start A)

# rust_wasm_pwa_minimal_clock

[comment]: # (lmake_cargo_toml_to_md start)

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)

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
**Github** is for now the place for my repositories. And **Git** on Debian is logically the version control system for my source code. 
You probably found this file you read just now on [Github]().



## Development

We will need a web file server because security does not allow loading modules from local file.  
Install this basic one:
`cargo install basic-http-server`  
Run the server in a separate terminal so it can stay running all the time:  
Go to the content folder:  
`cd rustprojects/rust_wasm_pwa_minimal_clock/web_server_folder/web_content_folder`  
`basic-http-server`  
Open the browser on:  
`http://127.0.0.1:4000`  
