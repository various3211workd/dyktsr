extern crate serde_derive;

use docopt::Docopt;
use dyktsr::modules::make_rhythm;

// Write the Docopt usage string.
const USAGE: &'static str = "
'dyktsr' stands for 'do_you_know_the_samba_rhythm'.
Convert input string to samba rhythm

Usage:
  dyktsr <rhythmtext> [-i <charicon>] [-p] [--html]
  dyktsr (--help | --version)

Options:
  -i <charicon>, --item=<charicon>    Put icon
  -p, --puertorico                    Put puertorico mode
  -h, --help                          Show this screen
  -v, --version                       Show version
";

#[warn(unused_must_use)]
fn main() {
  let version = "v1.0.0".to_owned();
  let args = Docopt::new(USAGE)
    .and_then(|dopt| dopt.version(Some(version)).parse())
    .unwrap_or_else(|e| e.exit());

  // default char icon
  let mut charicon = "🙏";
  if args.get_bool("--item") {
    charicon = args.get_str("--item");
  }

  if args.get_bool("--puertorico") {
    make_rhythm::puertorico_samba(args.get_str("<rhythmtext>").to_string(), charicon.to_string());
  }
  else {
    make_rhythm::plane_samba(args.get_str("<rhythmtext>").to_string(), charicon.to_string());
  }
}