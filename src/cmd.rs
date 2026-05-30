use crate::config::*;
use crate::version::version;

pub async fn exec(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 2 && (args[1] == "version" || args[1] == "-v") {
        println!("{}", version());
        return Ok(());
    }

    if args.len() == 2 && (args[1] == "--write-default-config" || args[1] == "-wdc") {
        return CqllsConfig::write_default_config_file();
    }

    #[cfg(debug_assertions)]
    if args.len() >= 3 && (args[1] == "--debug" || args[1] == "-d") {
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
