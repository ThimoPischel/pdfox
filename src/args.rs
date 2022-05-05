use std::env;
use std::path::*;
use std::fs;

#[derive(Debug)]
pub struct Args {
    pub data_json:   String,
    pub data_dir:    Option<String>,
    pub layout_json: Option<String>,
    pub layout_dir:  Option<String>,
    pub output_dir:  String
}
impl Args {
    pub fn new() -> Args {
        let args: Vec<String> = env::args().collect();
        println!("**** Args ****\n{:?}\n**** Args - Ende ****", args);

        let mut data_json:   Option<String> = None;
        let mut data_dir:    Option<String> = None;
        let mut layout_json: Option<String> = None;
        let mut layout_dir:  Option<String> = None;
        let mut output_dir:  Option<String> = None;

        for i in (1..args.len()).step_by(2) {
            match args[i].as_str(){
                "-d" => {
                    if i+1 < args.len() {
                        data_json = Some(args[i+1].clone());
                    }
                },
                "-D" => {
                    if i+1 < args.len() {
                        data_dir = Some(args[i+1].clone());
                    }
                },
                "-l" => {
                    if i+1 < args.len() {
                        layout_json = Some(args[i+1].clone());
                    }
                },
                "-L" => {
                    if i+1 < args.len() {
                        layout_dir = Some(args[i+1].clone());
                    }
                },
                "-o" => {
                    if i+1 < args.len() {
                        output_dir = Some(args[i+1].clone());
                    }
                },
                "-h" => {
                    println!(
                    r#" -o   := output directory (gets created if not exists)
                        [-d] := data.json (Will grep the data.json file in the
                                root folder of the data_dir folder if not provided)
                        [-D] := data_dir
                        [-l] := layout.json (Will grep the layout.json file in the root
                                folder of the layout_dir folder if not provided)
                        [-L] := layout_dir"#);
                    panic!()
                },
                _ => panic!("-h for help")
            }
        }

        //Check setup json paths
        let data_json :  String = match data_json {
            Some(dj) => dj,
            None => {
                match &data_dir {
                    None => panic!("No data.json found! Please set file with -d or include a data.json file in the root in your data directory and include this with -D"),
                    Some(dir) => Path::new(&dir).join(Path::new("data.json")).to_str()
                        .expect("failed to join the data_dir with data.json to a path").to_string()
                }
            }
        };

        let layout_json : Option<String> = match layout_json {
            Some(ly) => Some(ly),
            None => match &layout_dir {
                None => None,
                Some(dir) => Some(Path::new(&dir).join(Path::new("layout.json")).to_str()
                    .expect("failed to join the data_dir with data.json to a path").to_string())
            }
        };

        //check if everything is exists
        let check = |path_string: &Option<String>, what: String, file: bool| {
            match path_string {
                Some(path) => {
                    println!("Checking {} ... {}", what, &path);
                    let p = Path::new(&path);
                    if file {
                        if !p.is_file() {
                            panic!("it has to be a file!")
                        }
                    } else {
                        if !p.is_dir() {
                            panic!("it has to be a dir!")
                        }
                    }

                    if !p.exists() {
                        panic!("Failed... Path is not existing");
                    }                   
                },
                None => println!("{} not included!", what)
            }
        };

        check(&Some(data_json.clone()), "data_json (-d)".to_string(),   true);
        check(&data_dir,        "data_dir (-D)".to_string(),    false);
        check(&layout_json,     "layout_json (-l)".to_string(), true);
        check(&layout_dir,      "layout_dir (-L)".to_string(),  false);
        
        let output_dir : String = match output_dir {     
            None => panic!("Output directory required! Please set it with '-o'"),
            Some(pb) => {
                let path = Path::new(&pb);
                if !path.is_dir() {
                    panic!("output need to be a dir.")
                }
                if !path.exists(){
                    fs::create_dir(&pb).expect("Could not create output dir!");
                }
                pb
            }
        };

        Args {

        	data_json,
        	data_dir,
        	layout_json,
        	layout_dir,
        	output_dir,
        }
    }
}
