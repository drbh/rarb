use std::env;

extern crate time;
use time::PreciseTime;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct Triangle {
	a_name: String,
	b_name: String,
	c_name: String,
	ab_edge: f64,
	bc_edge: f64,
	ac_edge: f64,
}	

#[derive(Serialize, Deserialize)]
struct ComputedTriangle {
	triangle: Triangle,
	abca: f64,
	acba: f64,
	nanoduration: Option<i64>,
}
impl Triangle {
	fn counterclock(&self) -> f64 {
		return self.ac_edge / ( self.ab_edge * self.bc_edge );
	}
	fn clock(&self) -> f64 {
		return self.ab_edge / ( self.ac_edge  * (1.0/self.bc_edge) )
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	match args.len() {
		// no arguments passed
		7 => {
			let start = PreciseTime::now();
			let tri = Triangle{
				a_name: 		args[1].to_string(),
				b_name: 		args[2].to_string(),
				c_name: 		args[3].to_string(),
				ab_edge:		args[4].parse().unwrap(),
				bc_edge:		args[5].parse().unwrap(),
				ac_edge:		args[6].parse().unwrap(),
			};
			// do computations
			let abca = tri.counterclock();
			let acba = tri.clock();
			// build result triangle

			let end = PreciseTime::now();
			// println!("{} seconds for whatever you did.", start.to(end));
			let d = start.to(end).num_nanoseconds();
			let ctri = ComputedTriangle{
				triangle: tri,
				abca: abca,
				acba: acba,
				nanoduration: d
			};
			// build json function
			let j = || -> Result<(), Error> {
			    // Serialize it to a JSON string.
			    let j = serde_json::to_string(&ctri)?;

			    // Print, write to a file, or send to an HTTP server.
			    println!("{}", j);
			    // println!("{:#?}", j);
			    Ok(())
			};
			// create _result to run JSON conversion
			let _result = j();
		}
		// all the other cases
		_ => {
			// Err(());
		}
	}



}