use std::net::{TcpStream,TcpListener};
use std::io::Read;
use std::thread;

#[derive(Debug)]
pub(crate) struct KrServerConfig{
    pub(crate) server_name: String,
    pub(crate) hostname: String,
    pub(crate) port: String,
    pub(crate) instance_count: i8
}

struct CurrentRunning(String, KrServerConfig);

impl CurrentRunning {
    pub(crate) fn init(name: String, server_config: KrServerConfig){
        CurrentRunning{
            0: name,
            1: server_config
        };
    }
}

pub(crate) struct KrServer {
    config: KrServerConfig,
    //add TcpListener which will be started
    state: String
}
fn handle_connection(mut stream: TcpStream){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

impl KrServer{
    //let scontrol = CurrentRunning::init();
    //method to create KrServer and returns it
    pub(crate) fn new(name: String, config: KrServerConfig) -> KrServer {
        println!("Creating Server:{}, current state is:{}",name,name);
        KrServer{
            config,
            state: "CREATED".to_string(),
        }
    }


    // method to start listening
    pub(crate) fn listener_start(&mut self) {
        //@TODO:create CurrentRunning entry to send it to server ClusterInfo order
        //cluster info should be a struct containing current state and all running instances with
        let mut server_addr: String = "".to_string();
        server_addr.push_str(&self.config.hostname);
        server_addr.push_str(":");
        server_addr.push_str(&self.config.port);
        //= &self.config.hostname + ":" + &self.config.port;
        println!("config parameter: {}", server_addr);
        //let new_cluster_entry = CurrentRunning ("placeholder", config);
        let listener = TcpListener::bind(server_addr).unwrap();

        self.state = "LISTENING".to_string();
        println!("Started listener: {}, current STATE: {}", self.config.server_name, self.state);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");
            thread::spawn(|| {
                handle_connection(stream);
            });
        }
    }
}