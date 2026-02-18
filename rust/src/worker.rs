use crate::messages::{Task, ResultMsg};
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub async fn run_worker() -> tokio::io::Result<()> {
	let mut stream = TcpStream::connect("10.10.10.1:8080").await?;
	println!("Worker conectado al coordinador");

	//Aqui recibe la tarea
	let mut buf = vec![0; 1024];
	let n = stream.read(&mut buf).await?;
	let task: Task = serde_json::from_slice(&buf[..n]).unwrap();
	println!("Tarea recibida: {:?}", task);

	//Aqui se procesa la tarea (dymmy)
	let result = ResultMsg { id: task.id, output: "resultado dummy".to_string() };
	let serialized = serde_json::to_string(&result).unwrap();

	//Aqui se envia el resultado
	stream.write_all(serialized.as_bytes()).await?;
	Ok(()) 
}
