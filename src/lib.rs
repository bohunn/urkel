mod config;
mod server;

#[cfg(test)]
mod tests {
    use crate::server;
    use crate::server::{KrServer, KrServerConfig};
    use std::net::{TcpListener, TcpStream};
    use std::io::{BufReader, BufWriter, Write, BufRead};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn start_server(){
        let config = KrServerConfig{
            server_name: "server1".to_string(),
            hostname: ("localhost".to_string()),
            port: "8080".to_string(),
            instance_count: 1
        };
        let mut server:KrServer = KrServer::new("server1".to_string(), config);

        server.listener_start();

        let player_stream = TcpStream::connect("127.0.0.1:8080").expect("Couldn't connect");
        let mut reader = BufReader::new(&player_stream);
        let mut response = String::new();
        reader.read_line(&mut response).expect("Could not read");
        println!("Player received >{}<", response.trim());

        let mut writer = BufWriter::new(&player_stream);
        writer.write_all("NAME\n".as_bytes()).expect("Could not write");

    }
}


