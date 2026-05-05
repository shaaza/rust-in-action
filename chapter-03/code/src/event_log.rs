#[derive(Debug)]
enum EventType {
    Update,
    Delete,
    Unknown,
}

type Message = String;
type Event = (EventType, Message);

fn parse_log(line: &str) -> Event {
    let parts: Vec<_> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (EventType::Unknown, String::from(line))
    }
    let event_type = parts[0];
    let rest = String::from(parts[1]);
    match event_type {
        "UPDATE" | "update" => (EventType::Update, rest),
        "DELETE" | "delete" => (EventType::Delete, rest),
        _ => (EventType::Unknown, String::from(line)),
    }
}

pub fn parse_event_log() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let event = parse_log(line);
        println!("{:?}", event);
    }
}