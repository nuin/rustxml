extern crate xml;

use std::fs::File;
use std::io::BufReader;

// use std::io::{self, Write};

use xml::reader::{EventReader, XmlEvent};
// use xml::writer::{EventWriter, EmitterConfig, XmlEvent, Result};


// fn handle_event<W: Write>(w: &mut EventWriter<W>, line: String) -> Result<()>{

// 	let line = line.trim();
// 	let eventL XmlEvent = if line.starts_with("+") && line.len() > 1{
// 		XmlEvent::start_element(&line[1..]).into()
// 	} else if line.starts_with("-"){
// 		XmlEvent::end_element().into()
// 	} else {
// 		XmlEvent::characters(&line).into()
// 	}
// 	w.write(event)

// }


fn indent(size: usize) -> String{

	const INDENT: &'static str = "    ";
	(0..size).map(|_| INDENT).fold(String::with_capacity(size*INDENT.len()), |r,s| r + s)
}

fn main(){

	let file = File::open("Gene_class.xml").unwrap();
	let file = BufReader::new(file);

	let parser = EventReader::new(file);
	let mut depth = 0;
	for e in parser {

		match e{
			Ok(XmlEvent::StartElement { name, .. }) => {
				println!("{}+{}", indent(depth), name);
				depth += 1;
			}
			Ok(XmlEvent::EndElement {name}) => {
				depth -= 1;
				println!("{}-{}", indent(depth), name);
			}
			Err(e) => {
				println!("Error: {}", e);
				break;
			}
			_ => {}
		}
	}
}
