#[derive(Debug)]
struct Definition {
    height: u8,
    width: u8,
}
#[derive(Debug)]
struct Image {
    definition: Definition,
    entry: Vec<u8>,
}

fn get_definition_from_line(line: String) -> Result<Definition, String> {
    let arg = line.split(" ").collect::<Vec<&str>>();
    Ok
    (Definition {
        height: if let Ok(e) = arg[0].split(":").collect::<Vec<&str>>()[1].parse() {e} else {eprint!("Error on height"); return Err("Error on height".to_string());},
        width:  if let Ok(e) = arg[1].split(":").collect::<Vec<&str>>()[1].parse() {e} else {eprint!("Error on width"); return Err("Error on width".to_string());} ,
    })
}

fn get_entry_from_line(line: String) -> Vec<u8> {
    let mut result = Vec::new();

    for i in line.split(";").collect::<Vec<&str>>() {
        if !i.is_empty() {
            if let Ok(e) = i.parse() {
                result.push(e);
            } else {eprint!("File error")}
        }
    }

    result
}

fn main() {
    for i in &std::env::args().collect::<Vec<String>>()[1..] {
        let file = if let Ok(e) = std::fs::read_to_string(i) {
            e
        } else {
            eprint!("Error: File not found");
            break;
        };
        let lines: Vec<&str> = file.split("\n").collect();
        let def = get_definition_from_line(lines[1].to_string());
        let image = Image {
            definition: if let Ok(e) = def {e} else {break;},
            entry: get_entry_from_line(lines[2].to_string()),
        };

        let mut line_cursor = 0;
        for i in &image.entry {
            line_cursor += 1;
            if i == &0 {
                print!("\x1b[38;5;16m"); // Black
            } else {
                print!("\x1b[38;5;15m"); // White
            }
            print!("█");
            if line_cursor == image.definition.width {
                line_cursor = 0;
                print!("\n");
            }
        }
        print!("\n");
    }
}