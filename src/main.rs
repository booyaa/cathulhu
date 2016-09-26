extern crate hyper;
extern crate hubcaps;
extern crate clap;

use std::env;
use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug)]
struct Repository<'a> {
    owner: &'a str,
    name: &'a str,
}

fn main() {
    let matches = app_settings();

    // println!("subcom: {:?}", matches.subcommand_name());
    if let Some(ref issues_matches) = matches.subcommand_matches("issues") {
        // println!("issues matches: {:#?}", issues_matches);

        let repo = slug_to_repo(issues_matches.value_of("repository name").unwrap());
        let user_agent = format!("{}/{}", APP_NAME, APP_VERSION);
        let access_token = env::var("CATHULHU_GH_PAT")
                               .expect("Please store your GitHub PAT in an env var called \
                                        CATHULHU_GH_PAT!");
        let user_client = hyper::Client::new();
        let user_github =
            hubcaps::Github::new(user_agent,
                                 &user_client,
                                 hubcaps::Credentials::Token(access_token.to_string()));
        let repository = user_github.repo(repo.owner, repo.name);
        let issues_dataset = repository.issues();

        if issues_matches.is_present("list issues") {
            let issues = issues_dataset.list(&Default::default()).unwrap();
            for issue in issues {
                println!("{}|{}|{}", issue.number, issue.title, issue.html_url);
            }
        } else {
            println!("{}", issues_matches.usage());
            println!("Missing flag! use --help to see detailed help for the issue command!");
        }
    }
}

fn slug_to_repo<'a>(repository_slug: &str) -> Repository {
    let repo_parts: Vec<_> = repository_slug.split('/')
                                            .collect();
    Repository {
        owner: repo_parts[0],
        name: repo_parts[1],
    }
}

fn app_settings<'a>() -> ArgMatches<'a> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("issues")
                        .about("Issue related commands")
                        .arg(Arg::with_name("repository name")
                                 .help("Set repository name")
                                 .required(true)
                                 .index(1))
                        .arg(Arg::with_name("list issues")
                                 .short("l")
                                 .long("list")
                                 .help("lists open issues by number, title and url (pipe \
                                        delimited)")))
        .get_matches()
}
