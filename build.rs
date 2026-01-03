fn main() {
    // Embed Windows manifest for UAC elevation
    #[cfg(target_os = "windows")]
    {
        let _ = embed_resource::compile("app.manifest", embed_resource::NONE);
    }
}
