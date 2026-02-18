use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
	pub id: u32,
	pub payload: String, // Aqui es donde iria la tarea real
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultMsg {
	pub id: u32,
	pub output: String, // Aqui va el resultado parcial
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerInfo {
	pub id: u32,
	pub address: String, 
}
