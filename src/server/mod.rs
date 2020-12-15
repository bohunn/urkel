use std::net::TcpListener;

#[derive(Debug)]
pub(crate) struct KrServerConfig{
    pub(crate) server_name: String,
    pub(crate) server_address: String,
    pub(crate) instance_count: i8
}

struct CurrentRunning(String, KrServerConfig);

pub(crate) struct KrServer {
    config: KrServerConfig,
    //add TcpListener which will be started
    state: String
}

impl KrServer{

    //method to create KrServer and returns it
    pub(crate) fn new(name: String, config: KrServerConfig) -> KrServer {
        KrServer{
            config,
            state: "CREATED".to_string(),
        }
    }

    // method to start listening
    pub(crate) fn listener_start(&mut self) {
        //@TODO:create CurrentRunning entry to send it to server ClusterInfo order
        //cluster info should be a struct containing current state and all running instances with
        let server_addr = &self.config.server_address;
        println!("config parameter: {}", server_addr);
        //let new_cluster_entry = CurrentRunning ("placeholder", config);
        let listener = TcpListener::bind(server_addr).unwrap();

        self.state = "LISTENING".to_string();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
        }
    }
}