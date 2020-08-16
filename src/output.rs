use clap::ArgMatches;
use nixinfo::{
    cpu, device, distro, env, environment, gpu, hostname, kernel, memory, music, packages,
    terminal, uptime,
};

fn the_temp(matches: &ArgMatches) -> String {
    let unit = matches.value_of("temperature").unwrap();
    if unit == "C" {
        nixinfo::temp() + "*C"
    } else if unit == "F" {
        let pre = nixinfo::temp().parse::<i64>().unwrap() * 9 / 5 + 32;
        pre.to_string() + "*F"
    } else {
        format!("N/A ({} is not a supported unit)", unit)
    }
}

#[cfg(feature = "pretty_output")]
pub fn main(matches: ArgMatches) {
    let corner: char = if matches.is_present("corner") {
        matches.value_of("corner").unwrap().parse::<char>().unwrap()
    } else {
        '+'
    };
    let mut table = prettytable::Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .padding(1, 1)
        .separators(
            &[prettytable::format::LinePosition::Top],
            prettytable::format::LineSeparator::new('─', '─', corner, corner),
        )
        .separators(
            &[prettytable::format::LinePosition::Bottom],
            prettytable::format::LineSeparator::new('─', '─', corner, corner),
        )
        .build();
    table.set_format(format);
    if matches.is_present("cpu") {
        if matches.is_present("temperature") {
            let temp = the_temp(&matches);
            let row = format!("{} [{}]", nixinfo::cpu().trim(), temp);
            table.add_row(row!["CPU", &row]);
        } else {
            table.add_row(row!["CPU", &cpu()]);
        }
    }
    if matches.is_present("distro") {
        table.add_row(row!["Distro", &distro()]);
    }
    if matches.is_present("device") {
        table.add_row(row!["Device", &device()]);
    }
    if matches.is_present("editor") {
        table.add_row(row!["Editor", &env("EDITOR")]);
    }
    if matches.is_present("environment") {
        table.add_row(row!["Environment", &environment()]);
    }
    if matches.is_present("gpu") {
        table.add_row(row!["GPU", &gpu()]);
    }
    if matches.is_present("hostname") {
        table.add_row(row!["Hostname", &hostname()]);
    }
    if matches.is_present("kernel") {
        table.add_row(row!["Kernel", &kernel()]);
    }
    if matches.is_present("memory") {
        table.add_row(row!["Memory", &memory()]);
    }
    if matches.is_present("packages") {
        table.add_row(row![
            "Packages",
            &packages(matches.value_of("packages").unwrap())
        ]);
    }
    if matches.is_present("shell") {
        table.add_row(row!["Shell", &env("SHELL")]);
    }
    if matches.is_present("terminal") {
        table.add_row(row!["Terminal", &terminal()]);
    }
    if matches.is_present("uptime") {
        table.add_row(row!["Uptime", &uptime()]);
    }
    if matches.is_present("user") {
        table.add_row(row!["User", &env("USER")]);
    }
    if matches.is_present("music") {
        table.add_row(row!["Music", &music()]);
    }
    table.printstd();
}

#[cfg(not(feature = "pretty_output"))]
pub fn main(matches: ArgMatches) {
    if matches.is_present("cpu") {
        if matches.is_present("temperature") {
            println!("CPU:          {} [{}]", cpu(), the_temp(&matches));
        } else {
            println!("CPU:          {}", cpu());
        }
    }
    if matches.is_present("device") {
        println!("Device:       {}", device());
    }
    if matches.is_present("distro") {
        println!("Distro:       {}", distro());
    }
    if matches.is_present("editor") {
        println!("Editor:       {}", env("EDITOR"));
    }
    if matches.is_present("environment") {
        println!("Environment:  {}", environment());
    }
    if matches.is_present("gpu") {
        println!("GPU:          {}", gpu());
    }
    if matches.is_present("hostname") {
        println!("Hostname:     {}", hostname());
    }
    if matches.is_present("kernel") {
        println!("Kernel:       {}", kernel());
    }
    if matches.is_present("memory") {
        println!("Memory:       {}", memory());
    }
    if matches.is_present("packages") {
        let manager = matches.value_of("packages").unwrap();
        println!("Packages:     {}", packages(manager));
    }
    if matches.is_present("shell") {
        println!("Shell:        {}", env("SHELL"));
    }
    if matches.is_present("terminal") {
        println!("Terminal:     {}", terminal());
    }
    if matches.is_present("uptime") {
        println!("Uptime:       {}", uptime());
    }
    if matches.is_present("user") {
        println!("User:         {}", env("USER"));
    }
    if matches.is_present("music") {
        println!("Music:        {}", music());
    }
}