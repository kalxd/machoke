#[cxx::bridge(namespace = "XGLib")]
pub mod ffi {
	extern "Rust" {
		#[cxx_name = "sayHello"]
		fn say_hello();
	}
}

fn say_hello() {
	println!("hello world");
}
