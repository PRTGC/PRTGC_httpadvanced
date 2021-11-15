//################################################################################
//# Copyright 2021,  PRTG Consultants
//# support@prtgconsultants.com
//
// No warranty expressed or iplied
// For questions or issues related to this application please post messages in 
// issues section on github: https://github.com/PRTGC/PRTGC_httpadvanced/issues
//################################################################################

pub use reqwest::{
    Method,
    blocking::{
        Client,  ClientBuilder,
        Request, RequestBuilder,
        Response
    }
    
};
use clap::{Arg, App, 
    ArgMatches,
};


pub const PARAM_METHOD      : &str = "method";
pub const PARAM_BODY        : &str = "body";
pub const PARAM_URL         : &str = "url";
pub const PARAM_IGNORESSL   : &str = "ignoressl";
pub const PARAM_TIMEOUT     : &str = "timeout";
pub const PARAM_CREDS_LINUX : &str = "linux_creds";
pub const PARAM_CREDS_WIN   : &str = "windows_creds";


pub fn get_parameters<'a>() -> ArgMatches<'a> {
    let matches = App::new("prtgc_httpadvanced")
        .version("1.0")
        .author("JR Andreassen <support@prtgconsultants.com>")
        .about("Loopback ")
        .arg(Arg::with_name(PARAM_METHOD)
            .short("m")
            .long("method")
            .help("HTTP Method (GET|POST|HEAD|OPTIONS)")
            .default_value("GET")
            .required(true)
            )
        .arg(Arg::with_name(PARAM_URL)
            .long("url")
            .short("u")
            .help("URL to get")
            .takes_value(true)
            .required(false)
            )
        .arg(Arg::with_name(PARAM_BODY)
            .short("b")
            .long("body")
            .help("Post body")
            .required(false)
            .takes_value(true)
            )
        .arg(Arg::with_name(PARAM_IGNORESSL)
            .short("i")
            .long("ignoressl")
            .help("Ignore SSL error (Def false)")
            .required(false)
            .takes_value(false)
            )
        .arg(Arg::with_name(PARAM_TIMEOUT)
            .short("t")
            .long("timeout")
            .help("Query timeout (Def 30 sec)")
            .required(false)
            .takes_value(true)
            )
        .arg(Arg::with_name(PARAM_CREDS_LINUX)
            .short("L")
            .help("Use PRTG Linux Creds"))
        .arg(Arg::with_name(PARAM_CREDS_WIN)
            .short("W")
            .help("Use PRTG Windows Creds"))
        .get_matches();

    matches
}


pub enum CredType {
    Win,
    Linux,
    NoAuth
}
impl Default for CredType {
    fn default() -> Self {
        Self::NoAuth
    }
}

pub struct AppParameters {
    pub url: String,
    pub method: Method,
    pub body: Option<String>,
    pub creds: CredType,
    pub timeout: i32,
    pub ignoressl: bool,
}
impl Default for AppParameters {
    fn default() -> Self {
        Self {
            url: String::from("http://127.0.0.1"),
            method: Method::GET,
            body: None,
            creds: CredType::default(),
            timeout: 30,
            ignoressl: false,
        }
    }
}


impl TryFrom<ArgMatches<'_>> for AppParameters {
    type Error = String;
    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        let mut retval = Self::default();

        if matches.is_present(PARAM_CREDS_LINUX) {
          retval.creds = CredType::Linux;
        } 

        if let Some(val) = matches.value_of(PARAM_URL) {
            retval.url = val.into();
        }
  
        if let Some(val) = matches.value_of(PARAM_METHOD) {
          retval.method = 
            match val.to_uppercase().as_ref() {
                "POST"      => Method::POST,
                "OPTIONs"   => Method::OPTIONS,
                "HEAD"      => Method::HEAD,
                "GET"       => Method::GET,
                _ => {
                    return Err("Invalid Method (GET|POST)".into());
                }
            }
        };

        
        if matches.is_present(PARAM_IGNORESSL) {
            retval.ignoressl = true;
          } 
  
        if let Some(val) = matches.value_of(PARAM_BODY) {
            retval.body = Some(val.into())
        } else if retval.method == Method::POST {
            return Err("POST requires body".into());            
        }
  
        Ok(retval)
    }
}


pub fn make_error(pfix: &str, message: &str) ->String {
    format!("{{\"prtg\": {{\"error\": 1, \"text\": \"{}: {}\"}} }}", pfix, message)
}


pub fn get_env_creds(req_bldr: RequestBuilder, usr: &str, pwd: &str) 
    -> Result<RequestBuilder, String>
{
    let username = 
    match std::env::var(usr) {
        Ok(val) => val,
        Err(e) => return Err(format!("Username[{}] Not in env: {:?}", usr, e)),
    };
    let password = 
    match std::env::var(pwd) {
        Ok(val) => Some(val),
        Err(e) => return Err(format!("Password[{}] Not in env: {:?}", pwd, e)),
    };
    Ok(req_bldr.basic_auth(username, password) )
}


fn build_client(params: &AppParameters) 
  -> Result<Client, reqwest::Error> 
{
     //let mut headers = header::HeaderMap::new();
     //headers.insert("X-MY-HEADER", header::HeaderValue::from_static("value"));
     //headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("secret"));
    
     // Consider marking security-sensitive headers with `set_sensitive`.
     //let mut auth_value = header::HeaderValue::from_static("secret");
     //auth_value.set_sensitive(true);
     //headers.insert(header::AUTHORIZATION, auth_value);
    
     // get a client builder
     let client = Client::builder()
         //.default_headers(headers)
         //.timeout(params.timeout)
         .danger_accept_invalid_certs(params.ignoressl)
         ;
     //let res = client.meth ("https://www.rust-lang.org").send()?;
     client.build()
}

fn build_reqwest(cli: Client,  params: &AppParameters) 
  -> Result<Response, String> 
{
    let mut req_bldr =  cli.request(
            params.method.clone(), 
            params.url.clone());
    req_bldr = 
    match params.creds {
        CredType::Win => {
            get_env_creds(req_bldr, "windowsuser", "windowspassword")?
        },
        CredType::Linux => {
            get_env_creds(req_bldr, "linuxuser", "linuxpassword")?
        },
        _ => req_bldr,
    };
    req_bldr = 
     if let Some(val) = params.body.clone() {
        req_bldr.body(val)
     } else {req_bldr};
    
     match req_bldr.send() {
         Ok(val)  => Ok(val),
         Err(err) => Err(format!("Failed to create Reqeust {}", err)),
     }
}

fn main()  {
    let matches = get_parameters();
    let params: AppParameters = 
     match matches.try_into() {
         Ok(val)  => val,
         Err(err) => {
            let msg = make_error("Parameters", &err.to_string());
            println!("{}", msg);
            return;
         },
     };
    
    let cli = 
    match build_client(&params) {
        Ok(val)  => val,
        Err(err) => {
            let msg = make_error("Client", &err.to_string());
            println!("{}", msg);
            return;
        },
    };

    let resp = 
    match build_reqwest(cli, &params) {
        Ok(val)  => {
            match val.text()  {
                Ok(val)  => {val},
                Err(err) => {
                    make_error("Result", err.to_string().as_ref())
                },
            }
        },
        Err(err) => {
            make_error("Reqwest", err.as_ref())
        },
    };
    println!("{}", resp);
}

/*
https://sensepost.com/blog/2019/being-stubborn-pays-off-pt.-1-cve-2018-19204/
-url=
-Method=GET
-SSLMethod=sslvSSLv3SSLvTLSv1
-ProtocolVersion=1_1
-Agent=
-SensorId=
-CheckCertificate=false
-MaxDownload=
-writeresult=*PATH*
-timeout=60
-Proxy
-proxyUser=
-proxyPassword=

> measure-command {./HttpAdvancedSensor -url=http://127.0.0.2:34567/ping}
Milliseconds      : 119
Ticks             : 1190968
TotalSeconds      : 0.1190968
TotalMilliseconds : 119.0968

> measure-command {.\target\debug\reqwst_test --url "http://127.0.0.2:34567/ping"}
Milliseconds      : 79
Ticks             : 790343
TotalSeconds      : 0.0790343
TotalMilliseconds : 79.0343

> measure-command {.\target\release\reqwst_test --url "http://127.0.0.2:34567/ping"}
Milliseconds      : 51
Ticks             : 516736
TotalSeconds      : 0.0516736
TotalMilliseconds : 51.6736

*/
