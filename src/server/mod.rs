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
    state: String
}

impl KrServer{

    //method to create KrServer and returns it
    pub(crate) fn new(name: String, config: KrServerConfig) -> TcpListener {
        //create CurrentRunning entry to send it to server ClusterInfo order
        //cluster info should be a struct containing current state and all running instances with
        println!("config parameter: {}", config.server_address);
        //let new_cluster_entry = CurrentRunning ("placeholder", config);
        let listener = TcpListener::bind(config.server_address).unwrap();

        return listener
    }

    // method to start listening
    fn listener_start(listener: TcpListener) {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
        }
    }
}