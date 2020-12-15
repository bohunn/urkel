mod config;
mod server;

#[cfg(test)]
mod tests {
    use crate::server;
    use crate::server::{KrServer, KrServerConfig};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn start_server(){
        let config = KrServerConfig{
            server_name: "server1".to_string(),
            server_address: ("127.0.0.1:7878".to_string()),
            instance_count: 1
        };
        let server = KrServer::new("server1".to_string(), config);

    }
}


