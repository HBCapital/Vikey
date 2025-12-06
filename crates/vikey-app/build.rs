fn main() {
    #[cfg(target_os = "windows")]
    {
        embed_resource::compile("vikey-app.rc", embed_resource::NONE);
    }
}
