mod cell;

//Function to obtain a tcp stream to the desired address.
// This will be modified in the future to return a Tcp stream over a TLS connection
fn get_tcp_stream(ip: u32, port: u16) -> std::io::Result<std::net::TcpStream> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }

}
