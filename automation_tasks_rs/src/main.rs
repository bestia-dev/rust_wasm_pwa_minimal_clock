// automation_tasks_rs for rust_wasm_pwa_minimal_clock

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "build_and_run" {
                    task_build_and_run();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate with wasm-pack, fmt
cargo auto build_and_run - builds the crate with wasm_pack and runs the web server
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
"#,
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "build_and_run","commit_and_push"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion.

// region: tasks

fn task_build() {
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");    
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/rust_wasm_pwa_minimal_clock/pkg/");
    println!(
        r#"
After `cargo auto build` 
run `basic-http-server -a 0.0.0.0:4000 ./web_server_folder`
and open the browser on `http://localhost:4000/rust_wasm_pwa_minimal_clock/`
Finally close the browser and stop the server with `Ctrl+c` in the VSode terminal
or run`cargo auto build_and_run`
Later run `cargo auto commit_and_push "msg"`
"#
    );
}


fn task_build_and_run() {
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");    
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::run_shell_command("wasm-pack build --target web");
    cl::run_shell_command("\\rsync -a --delete-after pkg/ web_server_folder/rust_wasm_pwa_minimal_clock/pkg/");
    println!(
        r#"
After `cargo auto build_and_run` 
open the browser on `http://localhost:4000/rust_wasm_pwa_minimal_clock/`
Finally close the browser and stop the server with `Ctrl+c` in the VSode terminal.
Later run `cargo auto commit_and_push "msg"`
"#
    );
    cl::run_shell_command("run `basic-http-server -a 0.0.0.0:4000 ./web_server_folder`");
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

//TODO: publish to web
/*
script = [
    "echo ",
    "echo $ git tag -f -a v${CARGO_MAKE_CRATE_VERSION} -m version_${CARGO_MAKE_CRATE_VERSION}",
    "git tag -f -a v${CARGO_MAKE_CRATE_VERSION} -m version_${CARGO_MAKE_CRATE_VERSION}",
    "echo $ rsync -a --info=progress2 --delete-after ~/rustprojects/rust_wasm_pwa_minimal_clock/web_server_folder/rust_wasm_pwa_minimal_clock/ ~/rustprojects/googlecloud/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/",
    "rsync -a --info=progress2 --delete-after ~/rustprojects/rust_wasm_pwa_minimal_clock/web_server_folder/rust_wasm_pwa_minimal_clock/ ~/rustprojects/googlecloud/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/",
    "echo $ rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/googlecloud/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/ luciano_bestia@bestia.dev:/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/",
    "rsync -e ssh -a --info=progress2 --delete-after ~/rustprojects/googlecloud/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/ luciano_bestia@bestia.dev:/var/www/bestia.dev/rust_wasm_pwa_minimal_clock/",
]
*/

// endregion: tasks
