pub mod net {

    use super::super::super::error;
    use error::Error;
    use std::time::Duration;
    use tokio::net::TcpStream;

    ///Returns wether or not it is possible to conect to input port
    pub async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
        let mut addr = String::from(host);
        addr.push_str(":");
        addr.push_str(port.to_string().as_str()); //concatenate port and host with ':' character inbetween
        let test_addr = addr.clone(); //create a variable to be consumed by the lookup_host function
        let _ = tokio::net::lookup_host(test_addr).await?; //if wrong host adress an error is returned before trying a connexion
        let test_connexion =
            tokio::time::timeout(Duration::from_millis(3000), TcpStream::connect(addr)).await; //timeout of 3 seconds for connexion
        let test_connexion = test_connexion.map_err(Error::from)?; //map error with Error structure type
        match test_connexion {
            //TcpStream connect function returns Ok if the connexion could establish and an error otherwise
            Ok(_) => return Ok(true),
            Err(_) => return Ok(false),
        }
    }
}
