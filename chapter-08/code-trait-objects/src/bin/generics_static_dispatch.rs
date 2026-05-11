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

fn call_provider<P: LLMProvider>(provider: P) {
    println!("Response: {}", provider.send_request("barbaz".to_string()));
}

fn main() {
    call_provider(Claude {});
    call_provider(Gemini {});
    call_provider(OpenAI {});
}
