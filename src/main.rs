use clap::{App, Arg};
use std::fs;
mod git;
mod web_dev;
use git::{add_commit_push, make_github_repo};
use std::io::{Error, ErrorKind};
use web_dev::{create_web_dev_folder, open_stackoverflow, open_google};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Ms.Hudson")
        .version("1.0")
        .author("ProCode <bpro249@gmail.com>")
        .about("lets sherlock focus on whats important, meanwhile I'll make the hot water!")
        .arg(
            Arg::with_name("New_Project")
                .short("n")
                .long("newproject")
                .value_name("filename")
                .help("creates a project folder for you")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Webdev project")
                .short("w")
                .long("webdev")
                .help("creates html, css, js files within project")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("github project")
                .short("g")
                .long("git")
                .help("creates a github repository(requires token)")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("gacp")
                .short("p")
                .long("push")
                .help("Stage/add files to commit, commit and push to given branch")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("git_branch")
                .short("b")
                .long("branch")
                .help("Commits and push changes to given branch")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("git_commit")
                .short("m")
                .long("message")
                .help("The commit message")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Search_Stackoverflow")
            .short("e")
            .long("error")
            .help("Opens and search stack overflow for given error message")
            .takes_value(true)
        )
		.arg(
			Arg::with_name("Search_Google")
			.short("i")
			.long("google")
			.help("Opens and search google for given input")
			.takes_value(true)
		)
        .get_matches();

    if matches.is_present("New_Project") {
        let filename = matches.value_of("New_Project").unwrap_or("newproject");

        if matches.is_present("github project") {
            //create a github repository
            match make_github_repo(&filename).await {
                Ok(_) => {
                    if matches.is_present("Webdev project") {
                        //making a general web dev file structure
                        match create_web_dev_folder(&filename) {
                            Ok(_) => {
                                println!("Created Web Dev directory");
                            }
                            Err(_) => {
                                println!("Could not create a web dev directory");
                            }
                        }
                    }
                }
                Err(_) => {
                    return Err(Error::new(ErrorKind::Other, "Could Not Create File"));
                }
            }
        } else {
            match fs::create_dir(filename) {
                Ok(_) => {
                    if matches.is_present("Webdev project") {
                        //making a general web dev file structure
                        match create_web_dev_folder(&filename) {
                            Ok(_) => println!("Created a web dev directory"),
                            Err(_) => println!("Could not create a web dev directory"),
                        }
                    }
                }
                Err(_) => {
                    return Err(Error::new(ErrorKind::Other, "Could Not Create File"));
                }
            }
        }
    }

    if matches.is_present("gacp") {
        let branch = matches.value_of("git_branch").unwrap_or("master");
        let message = matches.value_of("git_commit").unwrap_or("Initial Commit");

        match add_commit_push(&branch, &message) { 
            Ok(_) => {
                println!("{}", format!("Pushed to {}", branch));
            },
            Err(_) => {
                println!("Something went wrong, cannot push changes");
            }
        }
    }

    if matches.is_present("Search_Stackoverflow") {
        let error = matches.value_of("Search_Stackoverflow").unwrap_or("");

        match open_stackoverflow(&error) {
            Ok(_) => {
                println!("Opening StackOverflow");
            },
            Err(_) => {
                println!("Something went wrong, cannot open StackOverflow");
            }
        }
    }
	
	if matches.is_present("Search_Google") {
        let input = matches.value_of("Search_Google").unwrap_or("");

        match open_google(&input) {
            Ok(_) => {
                println!("Opening Google Search");
            },
            Err(_) => {
                println!("Something went wrong, cannot open Google");
            }
        }
    }
    Ok(())
}
