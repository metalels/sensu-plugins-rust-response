extern crate getopts;
extern crate hyper;
pub extern crate openssl;
extern crate hyper_openssl;

use std::env;
use std::time::{SystemTime, Duration};
use getopts::Options;
use std::io::Write;
use openssl::ssl::{SslMethod, SslConnectorBuilder, SSL_VERIFY_NONE};
use hyper::Client;
use hyper_openssl::OpensslClient;
use hyper::net::HttpsConnector;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} URLS [options]\n\nRequires:\n    URLS: URL,URL,...  set target url(s: use [,] to joins multi urls)", program);
  print!("{}", opts.usage(&brief));
}

fn print_value(targetname: &str, value: f64) {
  println!("url.{} {} {:?}", targetname, value, time_now_unix());
}

fn print_value_with_prefix(groupname: &str, targetname: &str, value: f64) {
  println!("{}.url.{} {} {:?}", groupname, targetname, value, time_now_unix());
}

fn time_now_unix() -> u64 {
  SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}
  

fn elapsed_secs(dur: Duration) -> f64 {
  (dur.as_secs()) as f64 + (dur.subsec_nanos()) as f64 / 1_000_000_000.0
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.optopt("p", "prefix", "set prefix to output keys", "PREFIX");
  opts.optopt("t", "timeout", "set timeout secounds to http1/2 client", "TIMEOUT");
  opts.optflag("2", "http2", "enable to http2 access(current not support)");
  opts.optflag("i", "ignore", "enable to ignore ssl certification error");
  opts.optflag("D", "debug", "print debug logs");
  opts.optflag("H", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => {
      let _ = writeln!(&mut std::io::stderr(), "{}", f);
      return
    }
  };

  if matches.opt_present("H") {
    print_usage(&program, opts);
    return;
  };

  if matches.free.len() < 1 {
    let _ = writeln!(&mut std::io::stderr(), "Target urls(URL,URL...) must be specified.");
    return;
  };

  let mut debug = false;
  let mut ignore = false;
  let mut http2 = false;

  if matches.opt_present("D") {
    debug = true;
  };
  if matches.opt_present("i") {
    ignore = true;
  };
  if matches.opt_present("2") {
    http2 = true;
  };

  let urls = matches.free[0].clone();
  let targets: Vec<&str> = urls.split(',').collect();

  let prefix = match matches.opt_str("p") {
    Some(m) => { m }
    None => { "".to_string() }
  };
  let timeout = match matches.opt_str("t") {
    Some(m) => {
      match m.parse::<u64>() {
        Ok(v) => { Duration::new(v, 0) }
        Err(_) => { Duration::new(5, 0) }
      }
    }
    None => { Duration::new(5, 0) }
  };
  if debug {
    println!("prefix: {:?}", prefix);
    println!("timeout: {:?}", timeout);
  };

  let ssl_ctr;
  if ignore {
    ssl_ctr = SslConnectorBuilder::new_with_ssl_mode(SslMethod::tls(), SSL_VERIFY_NONE).unwrap();
  } else {
    ssl_ctr = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
  }
  let ssl = OpensslClient::from(ssl_ctr.build());
  
  let hyper_connector = HttpsConnector::new(ssl);
  let mut client;
  if http2 {
    //current stable hyper not support http2protocol
    //client = Client::with_protocol(Http2Protocol::with_connector(ssl_connector));
    client = Client::with_connector(hyper_connector);
  } else {
    client = Client::with_connector(hyper_connector);
  }
  client.set_read_timeout(Some(timeout));

  for target in targets {
    if debug {
      println!("target: {}", target);
    };
    let start = SystemTime::now();
    match client.get(target).send(){
      Ok(_) => {  }
      Err(f) => {
        let _ = writeln!(&mut std::io::stderr(), "{:?}", f);
        continue
      }
    };
    let elapsed = elapsed_secs(start.elapsed().unwrap());
    if debug {
      println!("duration: {}", elapsed);
    };
    if prefix.len() > 0 {
      print_value_with_prefix(&*prefix, target, elapsed);
    } else {
      print_value(target, elapsed);
    }
  }
}
