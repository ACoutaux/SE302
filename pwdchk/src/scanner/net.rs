pub mod net {

    use tokio::net::TcpStream;

    ///Returns wether or not it is possible to conect to input port
    pub async fn tcp_ping(host: &str, port: u16) -> bool {
        let mut addr = String::from(host);
        addr.push_str(":");
        addr.push_str(port.to_string().as_str()); //concatenate port and host with ':' character inbetween
        match TcpStream::connect(addr).await {
            //returns Ok if the connexion could establish and an error otherwise
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}
