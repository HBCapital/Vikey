fn main() {
    // Only build on Windows
    #[cfg(not(target_os = "windows"))]
    {
        panic!("vikey-windows-tsf can only be built on Windows");
    }
    
    #[cfg(target_os = "windows")]
    {
        // Embed version resource
        embed_resource::compile("vikey_tip.rc", embed_resource::NONE);
        
        // Link COM libraries
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=uuid");
    }
}
