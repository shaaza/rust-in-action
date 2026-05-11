use rand::rng;
use rand::seq::IndexedRandom;

struct OpenAI {}
struct Claude {}
struct Gemini {}

trait LLMProvider {
    fn send_request(&self, prompt: String) -> String;
}

impl LLMProvider for OpenAI {
    fn send_request(&self, _prompt: String) -> String {
        String::from("OpenAI")
    }
}

impl LLMProvider for Claude {
    fn send_request(&self, _prompt: String) -> String {
        String::from("Claude")
    }
}

impl LLMProvider for Gemini {
    fn send_request(&self, _prompt: String) -> String {
        String::from("Gemini")
    }
}

fn main() {
    let mut rng = rng();
    let provider_name = ["claude", "gemini", "openai"]
        .choose(&mut rng)
        .expect("No providers available");

    let response = match *provider_name {
        "claude" => Claude {}.send_request("barbaz".to_string()),
        "gemini" => Gemini {}.send_request("barbaz".to_string()),
        "openai" => OpenAI {}.send_request("barbaz".to_string()),
        _ => unreachable!(),
    };

    println!("Response: {}", response);
}
