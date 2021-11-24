use std::collections::HashMap;

fn main() {
    let action = std::env::args().nth(1).expect("Please, specify an action you sexy specimen");
    let item = std::env::args().nth(2).expect("Please, specify an item gorgeous 😘");
    // let binds value to var

    // std::env::args func from env module. returns args program started with. it's an iterator
    //  so we can access values stored in each position with nth() function

    // nth(0) -- argument as pos 0 is the program itself that's why we read from 1
    //  expect either returns value or
    //  if not present terminates immediately returning msg (termination = panic in rust terms)

    // print!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed 😢");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) { // match option returned by todo.complete(&item) method
            None => println!("'{}' is not present in the list", item), // if None, print warning. Fun fact, we're only able to pass &item because it was passed into the line before, without this it would have been owned by "complete" and dropped there
            Some(_) => match todo.save() { // if Some, call todo.save to store the change permanently into our file
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    // reads content of db.txt and gives us back our Todo populated with previous stored value
    fn new() -> Result<Todo, std::io::Error> { // new function returns either a Todo struct or an io:Error
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?; // file extension db.json instead of db.txt

        // seralize json as HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }), // deserializes file for us
            Err(e) if e.is_eof() => Ok(Todo { // Match guard. If file empty, return empty HashMap.
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e), // All other errors, exit immediately
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box <dyn std::error::Error>> { // Box returns pointer to error rather than error itself so caller can handle it. This is because it may return a file system error when opening file or a serde error when converting it, we don't know which it'll return
        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?; // writes our hashmap as a JSON
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> { // function return type: empty Option
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false), // * operator used to de-reference the value and set it to false
            None => None,
        }
    // self.map.get_mut gives mutable reference to value of key, or None if value isn't present
    // returns result of Match expression, either Some() or None
    }
}
