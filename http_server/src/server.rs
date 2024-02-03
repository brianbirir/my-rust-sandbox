use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String
}


impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        // on another terminal run  echo "TEST" | netcat 127.0.0.1 8080 to test receiving a request
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Failed to read from connection : {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }

        // tuple example:
        // let tup = (5, "a", listener);
    }
}