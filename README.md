# Benchmarking droping resource in rust
This repository shows how long does it take to drop a resource in various ways in Rust programming language.
This repository was created to test if droping resource in a different thread can be faster.

## Test cases

### Drop function
This is a simple call to rusts function `drop`

### Default drop
Simple call to the funciton
```
fn default_drop<T>(_: T) {

}
```

#### Default drop with warning
This test was to see if rust optimizes for values that it does not use
```
fn default_drop_with_warning<T>(vec: T) {

}
```

### Explicit drop
```
fn explicit_drop<T>(vec: T) {
    drop(vec);
}
```

### Dropping in a different thread
```
fn droping_in_thread<T>(vec: T)
    where T: Send + 'static {
    std::thread::spawn(move|| {
        vec
    });
```



## Report on AMD® Ryzen 5 2600x six-core processor × 12
*Note*: There were other porcesses running in background when this report was generated

Times in table represnt avrage value of 1000 test run.

Calling an empty function measurement: 23.00ns 

|Number of elements for Vec <i8> | 100 | 1000 | 10000 | 100000 | 1000000 | 10000000 |
|-----|-----|-----|-----|-----|-----|-----|
| Warm up | 26.00ns | 27.00ns | 39.00ns | 76.00ns | 168.00ns | 1.54µs |
| Drop function | 27.00ns | 27.00ns | 35.00ns | 64.00ns | 105.00ns | 1.02µs |
| Default drop | 44.00ns | 26.00ns | 44.00ns | 54.00ns | 130.00ns | 1.06µs |
| Default drop with warning | 29.00ns | 31.00ns | 36.00ns | 62.00ns | 122.00ns | 937.00ns |
| Explicit drop | 35.00ns | 28.00ns | 37.00ns | 61.00ns | 153.00ns | 999.00ns |
| Dropping in a different thread | 14.98µs | 11.08µs | 13.25µs | 13.90µs | 16.29µs | 51.91µs |

 |Number of elements for Vec <i32> | 100 | 1000 | 10000 | 100000 | 1000000 | 10000000 |
|-----|-----|-----|-----|-----|-----|-----|
| Warm up | 26.00ns | 41.00ns | 38.00ns | 72.00ns | 186.00ns | 2.28ms |
| Drop function | 27.00ns | 37.00ns | 43.00ns | 82.00ns | 178.00ns | 2.24ms |
| Default drop | 26.00ns | 40.00ns | 49.00ns | 86.00ns | 175.00ns | 2.27ms |
| Default drop with warning | 39.00ns | 53.00ns | 83.00ns | 98.00ns | 162.00ns | 2.19ms |
| Explicit drop | 26.00ns | 36.00ns | 57.00ns | 77.00ns | 168.00ns | 2.13ms |
| Dropping in a different thread | 12.78µs | 11.20µs | 14.83µs | 28.09µs | 22.76µs | 56.21µs |

|Number of elements for Vec <i64> | 100 | 1000 | 10000 | 100000 | 1000000 | 10000000 |
|-----|-----|-----|-----|-----|-----|-----|
| Warm up | 40.00ns | 53.00ns | 62.00ns | 113.00ns | 525.00ns | 3.53ms |
| Drop function | 39.00ns | 61.00ns | 57.00ns | 122.00ns | 574.00ns | 4.13ms |
| Default drop | 27.00ns | 42.00ns | 44.00ns | 102.00ns | 438.00ns | 4.57ms |
| Default drop with warning | 26.00ns | 42.00ns | 52.00ns | 104.00ns | 551.00ns | 4.69ms |
| Explicit drop | 28.00ns | 45.00ns | 61.00ns | 101.00ns | 386.00ns | 4.65ms |
| Dropping in a different thread | 15.66µs | 16.91µs | 14.34µs | 16.69µs | 40.80µs | 57.12µs |

|Droping a file | time |
|-----|-----|
|Warm up| 2.70µs |
|Drop function| 2.02µs |
|Default drop| 2.27µs |
|Explicit drop| 2.43µs |
|Dropping in a different thread| 17.84µs |