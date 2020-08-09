use crate::info::terminal::terminal;
use clap::ArgMatches;

#[cfg(feature = "pretty_output")]
pub async fn async_terminal(matches: &ArgMatches<'_>) {
    if matches.is_present("terminal") {
        let mut table = crate::shared_functions::table(' ', prettytable::format::LinePosition::Intern, 5);
        table.add_row(row!["Terminal", &terminal().trim()]);
        table.printstd();
    }
}

#[cfg(feature = "plain_output")]
pub async fn async_terminal(matches: &ArgMatches<'_>) {
    if matches.is_present("terminal") {
        println!("Terminal:     {}", terminal().trim());
    }
}
