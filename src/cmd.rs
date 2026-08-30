use crate::config::*;
use crate::version::{version, version_short};

const RESET: &str = "\x1b[0m";
const FG_PINK_HAIR: &str = "\x1b[38;2;255;160;202m";
const FG_PALE: &str = "\x1b[38;2;255;245;246m";
const FG_150: &str = "\x1b[38;2;150;150;150m";
const FG_CYANX: &str = "\x1b[38;2;146;228;250m";
const FG_MAGENTA: &str = "\x1b[35m";
const FG_YELLOW: &str = "\x1b[33m";
const FG_PINK: &str = "\x1b[38;2;255;182;202m";

pub async fn exec(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 2 && (args[1] == "help" || args[1] == "-h" || args[1] == "--help") {
        usage();
        return Ok(());
    }

    if args.len() == 2 && (args[1] == "version" || args[1] == "-v" || args[1] == "--version") {
        println!("{}", version());
        return Ok(());
    }

    if args.len() == 2 && (args[1] == "--write-default-config" || args[1] == "-wdc") {
        return CqllsConfig::write_default_config_file();
    }

    #[cfg(debug_assertions)]
    if args.len() >= 3 && (args[1] == "debug" || args[1] == "--debug" || args[1] == "-d") {
        match args[2].as_ref() {
            "fmt" => {
                use crate::test_base::debug_format;

                debug_format(&args[3]).await;
            }
            "cmt" => {
                use crate::test_base::debug_completion;

                let line: u32 = args[4].parse().expect("line must be a number");
                let character: u32 = args[5].parse().expect("character must be a number");
                debug_completion(&args[3], line, character).await;
            }
            _ => {}
        }
        return Ok(());
    }

    Ok(())
}

pub fn usage() {
    println!("{}", RESET);
    println!(
        "    {}⣇⣿⠘⣿⣿⣿⡿⡿⣟⣟⢟⢟⢝⠵⡝⣿⡿⢂{}{}⣼⣷⣌{}{}⠩⡫⡻⣝⠹⢿⣿⣷{}",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_PINK_HAIR, RESET
    );
    println!(
        "    {}⡆⣿⣆⠱⣝⡵⣝⢅⠙⣿⢕⢕⢕⢕⢝⣥⢒⠅{}{}⣿⣿⣿⡿⣳⣌{}{}⠪⡪⣡⢑⢝⣇{}",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_PINK_HAIR, RESET
    );
    println!(
        "    {}⡆⣿⣿⣦⠹⣳⣳⣕⢅⠈⢗⢕⢕⢕⢕⢕⢈⢆{}{}⠟⠋⠉⠁⠉⠉⠁{}{}⠈⠼⢐⢕⢽{}",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_PINK_HAIR, RESET
    );
    println!(
        "    {}⡗{}{}⢰⣶⣶⣦{}{}⣝⢝⢕⢕⠅{}{}⡆{}{}⢕⢕⢕⢕{}⢕⣴⠏⣠⡶⠛⡉⡉⡛⢶⣦⡀{}⠐⣕⢕{}        {}cqlls (CQL Language Server){} {}^_^{}",
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_150,
        RESET,
        FG_CYANX,
        RESET
    );
    println!(
        "    {}⡝⡄{}{}⢻⢟⣿⣿⣷{}{}⣕⣕⣅{}{}⣿⣔{}{}⣕{}⣵{}⣵⣿⣿{}⢠⣿{}⢠⣮⡈{}{}⣌{}⠨⠅⠹⣷⡀{}⢱⢕{}",
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_CYANX,
        RESET,
        FG_YELLOW,
        RESET,
        FG_PINK_HAIR,
        RESET
    );
    println!(
        "    {}⡝⡵{}⠟⠈⢀⣀⣀⡀⠉{}⢿⣿⣿⣿⣿⣿⣿⣿{}⣼⣿⢈⡋{}⠴⢿⡟{}{}⣡⡇{}⣿⡇⡀{}⢕{}        {}Version{} {}  >{} {}{}{}",
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_YELLOW,
        RESET,
        FG_CYANX,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_150,
        RESET,
        FG_CYANX,
        RESET,
        FG_MAGENTA,
        version_short(),
        RESET
    );
    println!(
        "    {}⡝{}⠁⣠⣾⠟⡉⡉⡉⠻⣦{}⣻⣿⣿⣿⣿⣿⣿⣿{}⣿⣧{}⠸⣿⣦⣥⣿⡇{}⡿⣰⢗⢄",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_CYANX, RESET
    );
    println!(
        "    {}⠁{}⢰⣿⡏{}⣴⣌{}⠈{}⣌{}⠡⠈⢻⣿{}⣿⣿⣿⣿⣿⣿⣿⣿⣿⣬{}{}⣉⣉⣁{}⣄⢖⢕⢕⢕        {}Copyright{} {}>{} {}アクゼスティア{}",
        FG_PINK_HAIR,
        RESET,
        FG_CYANX,
        RESET,
        FG_YELLOW,
        RESET,
        FG_PALE,
        RESET,
        FG_CYANX,
        RESET,
        FG_150,
        RESET,
        FG_CYANX,
        RESET,
        FG_MAGENTA,
        RESET
    );
    println!(
        "    {}⡀{}⢻⣿⡇{}⢙⠁{}{}⠴⢿⡟{}{}⣡⡆{}{}⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣵⣵⣿{}",
        FG_PINK_HAIR, RESET, FG_CYANX, RESET, FG_YELLOW, RESET, FG_CYANX, RESET, FG_PALE, RESET
    );
    println!(
        "    {}⡻⣄{}⣻⣿⣌{}⠘⢿⣷⣥⣿⠇{}{}⣿⣿⣿⣿⣿⣿⠛⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿{}        {}License{} {}  >{} {}MIT{}",
        FG_PINK_HAIR,
        RESET,
        FG_CYANX,
        RESET,
        FG_PALE,
        RESET,
        FG_150,
        RESET,
        FG_CYANX,
        RESET,
        FG_YELLOW,
        RESET
    );
    println!(
        "    {}⣷⢄{}{}⠻⣿{}⣟⠿⠦{}⠍⠉{}{}⣡⣾⣿⣿⣿⣿⣿⣿{}{}⢸⣿⣦{}{}⠙⣿⣿⣿⣿⣿⣿⣿⣿⠟{}",
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_CYANX,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK,
        RESET,
        FG_PALE,
        RESET
    );
    println!(
        "    {}⡕⡑⣑{}{}⣈⣻⢗⢟⢞⢝⣻⣿⣿⣿⣿⣿⣿⣿{}{}⠸⣿⠿⠃{}{}⣿⣿⣿⣿⣿⣿⡿⠁{}{}⣠{}        {}Github{} {}   >{} {}https://github.com/Akzestia/cqlls{}",
        FG_PINK_HAIR,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK,
        RESET,
        FG_PALE,
        RESET,
        FG_PINK_HAIR,
        RESET,
        FG_150,
        RESET,
        FG_CYANX,
        RESET,
        FG_MAGENTA,
        RESET
    );
    println!(
        "    {}⡝⡵⡈{}⢟⢕⢕⢕⢕{}⣵⣿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣶⣿⣿⣿⣿⣿⠿⠋{}{}⣀⣈⠙ {}",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_PINK_HAIR, RESET
    );
    println!(
        "    {}⡝⡵⡕⡀{}⠑{}⠳⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛{}{}⢉⡠⡲⡫⡪⡪⡣{}",
        FG_PINK_HAIR, RESET, FG_PALE, RESET, FG_PINK_HAIR, RESET
    );

    println!("\n{}Commands:{}", FG_MAGENTA, RESET);
    println!(
        "    {}--version{}              | {}Prints language server version{}",
        FG_CYANX, RESET, FG_150, RESET
    );
    println!(
        "    {}--debug{} <fmt|cmt>      | {}Dev thingy used for debugging ^_^{}",
        FG_CYANX, RESET, FG_150, RESET
    );
    println!(
        "    {}--write-default-config{} | {}Writes a default .cqlls configuration file{}",
        FG_CYANX, RESET, FG_150, RESET,
    );
    println!(
        "    {}--help{}                 | {}Prints this message ^_^{}",
        FG_CYANX, RESET, FG_150, RESET
    );

    println!();

    println!("{}Aliases:{}", FG_MAGENTA, RESET);
    println!(
        "    {}-v|version{}         | {}Prints language server version{}",
        FG_CYANX, RESET, FG_150, RESET
    );
    println!(
        "    {}-d|debug{} <fmt|cmt> | {}Dev thingy used for debugging ^_^{}",
        FG_CYANX, RESET, FG_150, RESET
    );
    println!(
        "    {}-wdc{}               | {}Writes a default .cqlls configuration file{}",
        FG_CYANX, RESET, FG_150, RESET
    );
    println!(
        "    {}-h|help{}            | {}Prints this message ^_^{}",
        FG_CYANX, RESET, FG_150, RESET
    );

    println!();
}
