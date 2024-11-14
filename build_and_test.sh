cargo build
target/debug/curl "www.eecg.toronto.edu"
target/debug/curl "data://www.eecg.toronto.edu"
target/debug/curl "http//www.eecg.toronto.edu"
target/debug/curl "https://255.255.255.256"
target/debug/curl "https://[...1]"
target/debug/curl "http://127.0.0.1:65536"