fn main() {
    let cmd1 = BrowserCommand::new("navigate".to_owned(), "https://www.google.com".to_owned());

    let cmd2 = BrowserCommand {
        name: "zoom".to_string(),
        payload: 200,
    };

    cmd1.print_payload();
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
}

struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload)
    }
}

fn serialize_payload<T>(payload: T) -> String {
    // Convert payload to JSON string
    "placeholder".to_owned()
}
