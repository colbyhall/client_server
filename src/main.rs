use std::env;
use std::io;
use std::net;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";
const CLIENT_ADDRESS: &str = "127.0.0.1:3030";

fn server_main() -> io::Result<()> {
	let socket = net::UdpSocket::bind(SERVER_ADDRESS)?;

	println!("Starting server on \"{}\"", SERVER_ADDRESS);

	loop {
		let mut buf = [0; 1024];
		let recieved = socket.recv(&mut buf)?;

		println!("recieved {} bytes {:?}", recieved, &buf[..recieved])
	}
}

fn client_main() -> io::Result<()> {
	let socket = net::UdpSocket::bind(CLIENT_ADDRESS)?;
	socket.connect(SERVER_ADDRESS)?;

	println!(
		"Starting client on \"{}\". Connecting to \"{}\"",
		CLIENT_ADDRESS, SERVER_ADDRESS
	);

	loop {
		socket.send(&[0, 1, 2])?;
		std::thread::sleep(std::time::Duration::from_millis(16));
	}
}

fn main() -> io::Result<()> {
	let mut args = env::args();

	let is_server = match args.nth(1) {
		Some(arg) => arg == "-server",
		None => false,
	};

	if is_server {
		server_main()
	} else {
		client_main()
	}
}
