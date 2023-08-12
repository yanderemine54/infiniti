// Copyright (c) 2023 Yanderemine54
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use clap::{Arg, Command, ArgAction, ArgMatches};
fn main() {
    let cli: Command = Command::new("infiniti")
                    .author("Yanderemine54, ymtechnologies@mail.com")
                    .version("0.1.0")
                    .about("A source-based package manager designed for reliability and portability.")
                    .arg_required_else_help(true)
                    .subcommand_required(true)
                    .subcommand(
                        Command::new("install") // -i, --install subcommand
                            .short_flag('i')
                            .long_flag("install")
                            .about("Install one or more packages.")
                            .arg(
                                Arg::new("packages")
                                .action(ArgAction::Set)
                                .num_args(1..)
                            )
                    )
                    .subcommand(
                        Command::new("update") // -u, --update subcommand
                            .short_flag('u')
                            .long_flag("update")
                            .about("Update one or more packages or the system.")
                            .arg(
                                Arg::new("packages")
                                .action(ArgAction::Set)
                                .num_args(0..)
                            )
                    )
                    .subcommand(
                        Command::new("remove") // -r, --remove subcommand
                            .short_flag('r')
                            .long_flag("remove")
                            .about("Delete one or more packages.")
                            .arg(
                                Arg::new("packages")
                                .action(ArgAction::Set)
                                .num_args(0..)
                            )
                    );
    let matches: ArgMatches = cli.get_matches();
    
}
