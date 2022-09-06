
use std::time::{Instant, Duration};

fn build_vec<G>(n: usize) -> Vec<G>
    where G: Default {
    let mut vec = Vec::new();
    vec.reserve(n);

    for _i in 1..n {
        vec.push(Default::default());
    }

    vec
}

fn default_drop<T>(_: T) {

}

fn default_drop_with_warning<T>(vec: T) {

}

fn explicit_drop<T>(vec: T) {
    drop(vec);
}

fn droping_in_thread<T>(vec: T)
    where T: Send + 'static {
    std::thread::spawn(move|| {
        vec
    });
}

fn droping_in_thread_explicit<T>(vec: T)
    where T: Send + 'static {
    std::thread::spawn(move|| {
        drop(vec);
    });
}

fn profile_a_funtion<T, F, G>(function: F, argument: G, num_of_samples: usize) 
    where F: Fn(T) -> (),
          G: Fn() -> T
    {
        let mut total_duration: Duration = Default::default();
        for _ in 0..num_of_samples {
            let arguments = argument();
            let now = Instant::now();
            function(arguments);
            let elapsed = now.elapsed();
            total_duration = total_duration + elapsed;
        }
        total_duration  = total_duration / num_of_samples.try_into().unwrap();
        print!(" {:.2?} |", total_duration);
}


fn draw_header_for_vec(type_tested: &str, max_number_of_elements_in_vec: usize) {

    {
        print!("\n|Number of elements for {} |", type_tested);
        let mut i = 100;
        while i <= max_number_of_elements_in_vec {
            print!(" {} |", i);
            i = i*10;
        }
        println!("");
        let mut i = 100;
        print!("|-----|");
        while i <= max_number_of_elements_in_vec {
            print!("-----|");
            i = i*10;
        }
        println!("");
    }
}
fn profile_on_vec<F, G, T>(name_of_test: &str, function: F, argument_function: G, num_of_samples: usize, max_number_of_elements_in_vec: usize)
    where F: Fn(Vec<T>) -> (), G: Fn(usize) -> Vec<T> {

    {
        print!("| {} |", name_of_test);
        let mut i = 100;

        while i <= max_number_of_elements_in_vec {
            profile_a_funtion(&function, || { argument_function(i)}, num_of_samples);
            i = i*10;
        }
        println!("");
    }
}

fn open_file() -> std::fs::File {
    std::fs::File::create("/tmp/test").unwrap()
}

fn main() {
    let num_of_samples = 1000;
    {
        println!("Calling an empty function measurement");
        profile_a_funtion(default_drop,|| {()},num_of_samples);
    }

    {
        let max_number_of_elements_in_vec = 10000000;
        draw_header_for_vec("Vec <i8>", max_number_of_elements_in_vec);
        profile_on_vec("Warm up", default_drop, build_vec::<i8> ,num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Drop function", drop, build_vec::<i8>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop", default_drop, build_vec::<i8>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop with warning", default_drop_with_warning, build_vec::<i8>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Explicit drop", explicit_drop, build_vec::<i8>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Dropping in a different thread", droping_in_thread, build_vec::<i8>, num_of_samples, max_number_of_elements_in_vec);
        println!("");
    }

    {
        let max_number_of_elements_in_vec = 10000000;
        draw_header_for_vec("Vec <i32>", max_number_of_elements_in_vec);
        profile_on_vec("Warm up", default_drop, build_vec::<i32> ,num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Drop function", drop, build_vec::<i32>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop", default_drop, build_vec::<i32>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop with warning", default_drop_with_warning, build_vec::<i32>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Explicit drop", explicit_drop, build_vec::<i32>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Dropping in a different thread", droping_in_thread, build_vec::<i32>, num_of_samples, max_number_of_elements_in_vec);
        println!("");
    }

    {
        let max_number_of_elements_in_vec = 10000000;
        draw_header_for_vec("Vec <i64>", max_number_of_elements_in_vec);
        profile_on_vec("Warm up", default_drop, build_vec::<i64> ,num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Drop function", drop, build_vec::<i64>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop", default_drop, build_vec::<i64>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Default drop with warning", default_drop_with_warning, build_vec::<i64>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Explicit drop", explicit_drop, build_vec::<i64>, num_of_samples, max_number_of_elements_in_vec);
        profile_on_vec("Dropping in a different thread", droping_in_thread, build_vec::<i64>, num_of_samples, max_number_of_elements_in_vec);
        println!("");
    }


    {
        println!("|Droping a file | time |");
        println!("|-----|-----|");
        print!("|Warm up|");
        profile_a_funtion(default_drop, open_file, num_of_samples);
        print!("\n|Drop function|");
        profile_a_funtion(drop, open_file, num_of_samples);
        print!("\n|Default drop|");
        profile_a_funtion(default_drop, open_file, num_of_samples);
        print!("\n|Explicit drop|");
        profile_a_funtion(explicit_drop, open_file, num_of_samples);
        print!("\n|Dropping in a different thread|");
        profile_a_funtion(droping_in_thread, open_file, num_of_samples);
        println!("");
    }
}
