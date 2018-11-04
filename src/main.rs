#[derive(Debug)]
struct Triangle {
	a_name: String,
	b_name: String,
	c_name: String,

	ab_edge: f64,
	bc_edge: f64,
	ac_edge: f64,
}

trait CounterClockwise {
	// add code here
}

impl CounterClockwise for Triangle {
	// add code here
	// fn name(arg: Type) -> RetType {
	// 	unimplemented!();
	// }
}

fn main() {

	let tri = Triangle{
		a_name: "NodeA".to_string(),
		b_name: "NodeB".to_string(),
		c_name: "NodeC".to_string(),

		ab_edge:		23.4,
		bc_edge:		432433.3,
		ac_edge:		32.5
		
	};
    println!("{:?}", tri);
    // println!("Hello, world!");
}
 