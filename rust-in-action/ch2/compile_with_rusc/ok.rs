fn main() {
    println!("OK")
}

// Compiling this with rustc ok.rs will result in a binary which will print "OK" to stdout
// Rustc isn't suitable for projects with multiple files, so we'll use cargo to compile those