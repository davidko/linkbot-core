
//use std::process::Command;
extern crate protoc_rust;
use std::env::var;

fn main() {
    println!("Building proto files...");
    let nanopb_dir = var("NANOPB_DIR").expect("Could not get environment variable NANOPB_DIR");

    protoc_rust::run( protoc_rust::Args {
        out_dir: "src/protos",
        input: &["linkbot-interfaces/dongle.proto",
                 "linkbot-interfaces/commontypes.proto",
                 "linkbot-interfaces/daemon.proto",
                 "linkbot-interfaces/radio.proto",
                 "linkbot-interfaces/robot.proto",
                 "linkbot-interfaces/bumpconnect.proto",
                ],
        includes: &[ nanopb_dir.as_str(),
                     "linkbot-interfaces",
                   ],
    }).expect("protoc");


    /*
    let output = Command::new("protoc")
                         .arg("--rust-out=src/proto")
                         .arg(format!("-I{}", nanopb_dir))
                         .arg("deps/linkbot-interfaces/dongle.proto")
                         .output()
                         .expect("Could not compile dongle.proto");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("protoc")
                         .arg("--rust-out=src/proto")
                         .arg(format!("-I{}", nanopb_dir))
                         .arg("deps/linkbot-interfaces/commontypes.proto")
                         .output()
                         .expect("Could not compile commontypes.proto");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("protoc")
                         .arg("--rust-out=src/proto")
                         .arg(format!("-I{}", nanopb_dir))
                         .arg("deps/linkbot-interfaces/daemon.proto")
                         .output()
                         .expect("Could not compile daemon.proto");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
    */
}
