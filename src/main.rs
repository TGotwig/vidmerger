#![deny(warnings)]

use core::time;
use std::io::Error;
use std::path::Path;
use std::thread;

use cli::Cli;
use path_slash::PathExt;

mod cli;
mod commanders;
mod ffmpeg_args_factory;
mod helper;

use system_shutdown::shutdown;

use term_painter::Color::BrightBlue;
use term_painter::ToStyle;

fn main() -> Result<(), Error> {
    helper::exit_when_ffmpeg_not_available();
    let matches = Cli::init().get_matches();
    let target_dir = matches.value_of("TARGET_DIR").unwrap().to_string();
    let formats = matches
        .value_of("format")
        .unwrap_or("avchd,avi,flv,mkv,mov,mp4,webm,wmv")
        .to_string();
    let should_shutdown = matches.is_present("shutdown");

    for file_format in helper::split(formats) {
        let input_vids = Path::new(&target_dir);
        let output_vid = input_vids.join(format!("output.{}", file_format));

        helper::remove_file(&output_vid)?;

        let ffmpeg_input_content =
            helper::gen_ffmpeg_input_content(input_vids, file_format.as_str());

        if !ffmpeg_input_content.is_empty() {
            let tmp_dir = helper::create_tmp_dir();

            println!(
                "\n👇 Order of merging:\n\n{}\n",
                BrightBlue.paint(&ffmpeg_input_content)
            );
            println!("⏳ Starts after 3 seconds...\n");
            thread::sleep(time::Duration::from_secs(3));

            let list_txt = helper::create_list_txt(ffmpeg_input_content, tmp_dir);

            let ffmpeg_args = ffmpeg_args_factory::make_merge_args(
                &list_txt.to_slash().unwrap(),
                output_vid.to_slash().unwrap().to_string(),
            );

            commanders::merger::merge(ffmpeg_args, file_format);
        }
    }

    if should_shutdown {
        match shutdown() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    } else {
        Ok(())
    }
}
