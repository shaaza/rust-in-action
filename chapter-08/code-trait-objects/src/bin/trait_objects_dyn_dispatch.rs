use rand::rng;
use rand::prelude::IndexedRandom;

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
    let providers: Vec<&dyn LLMProvider> = vec![&Claude {}, &Gemini {}, &OpenAI {}];

    let mut rng = rng();
    let provider = providers.choose(&mut rng).expect("No providers available");
    println!("Response: {}", provider.send_request("barbaz".to_string()))
}
