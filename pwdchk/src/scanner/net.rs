pub mod net {

    use super::super::super::error;
    use error::Error;
    use futures::prelude::*;
    use ipnet::Ipv4Net;
    use std::time::Duration;
    use tokio::net::TcpStream;

    ///Returns wether or not it is possible to conect to input port
    async fn tcp_ping(host: &str, port: u16) -> Result<bool, Error> {
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

    ///Call in parallel tcp_ping() function on a list of host and ports
    async fn tcp_ping_many<'a>(
        targets: &[(&'a str, u16)],
    ) -> Vec<(&'a str, u16, Result<bool, Error>)> {
        let res = stream::iter(targets); //put targets in a stream
                                         //For each (str,u16) tuple call tcp_ping and returns a tuple with the result in an async bloc in order to have a future
        let res = res.map(|x| async { (x.0, x.1, tcp_ping(x.0, x.1).await) });
        let res = res.buffer_unordered(100); //100 unachived futures can execute in parallel (arbitrary number)
        let res = res.collect::<Vec<_>>().await; //collect tuple in a vector and wait for futures completion with await
        res
    }

    ///Set a list of ports and a list of hosts in a list of (host,port) and call tcp_ping_many on it
    pub async fn tcp_mping<'a>(
        targets: &[&'a str],
        ports: &[u16],
    ) -> Vec<(&'a str, u16, Result<bool, Error>)> {
        let mut tuple_vec: Vec<(&str, u16)> = vec![];
        targets.iter().for_each(|host| {
            ports.iter().for_each(|port| tuple_vec.push((*host, *port)));
        });

        tcp_ping_many(&tuple_vec).await //send slice &tuple_tab to function tcp_ping_many and return result
    }

    //Returns all adresses corresponding to CIDR notation or submitted string otherwise
    pub fn expand_net(host: &str) -> Vec<String> {
        if host.contains('/') {
            let net: Ipv4Net = String::from(host).parse().unwrap(); //creates a net of ipv4 adresses
            let adresses = net.hosts().map(|x| x.to_string()).collect::<Vec<String>>(); //iteration on adresses which are converted into strings and collect
            adresses
        } else {
            vec![String::from(host)]
        }
    }
}
