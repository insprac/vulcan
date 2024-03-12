pub trait CompletionProvider {
    fn complete(&self, context: &str) -> &str;
}
