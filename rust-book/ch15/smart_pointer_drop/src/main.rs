struct CustomSmartPointer {
    data: String,
}

// This code is executed at the end of main(), when "c" and "d" go out of scope, because
//  we've implemented the Drop trait onto our CustomSmartPointer struct.
// Note: Drop is included in the prelude, which is why we don't need to bring it into
//  scope.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
