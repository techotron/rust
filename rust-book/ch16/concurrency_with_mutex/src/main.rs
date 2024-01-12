use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // The lock method acquires the lock which allows us to access the data inside
        //  which is currently, 5. This code will block until it has the lock -  however
        //  as it's only single threaded, there's nothing else to contend with the lock so
        //  so using unwrap here is ok (otherwise it'd panic if another thread had a lock).
        // m is a Mutex<T>, which is a smart pointer called MutexGuard which is wrapped in
        //  a LockResult. It implements Drop which releases the lock automatically when the
        //  MutexGuard goes out of scope.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
