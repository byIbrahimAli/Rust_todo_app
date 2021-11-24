use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

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
    fn new() -> Result<Todo, std::io::Error> {
    // new function returns either a Todo struct or an io:Error

        let mut f = std::fs::OpenOptions::new() // configures how to open db.txt
            .write(true)
            .create(true)// creates db.txt if not already present
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();

        f.read_to_string(&mut content)?; // reads all bytes in file and appends them into the content String

        let map: HashMap<String, bool> = content // converts from String type of db.txt to HashMap by binding a map variable
            .lines()// creates iterator over each line of a string
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) // splits lines on tab char
            .map(|v| (v[0], v[1]))// collect::<Vec<&str>>() transforms an iterator into a relevant collection then transform into tuple using .map(v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))// Convert the two elements of tuple into a String and a boolean
            .collect(); // Collect the above into our HashMap

        Ok(Todo { map }) // If no errors return struct to caller with Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)

        // -> annotates the returned type from the function. We are returning a Result.
        // We iterate over the map, and format each string, separating key and value with a tab
        //  character and each line with a new line.
        // We push the formatted string into a content variable.
        // We write content inside a file called db.txt.
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
