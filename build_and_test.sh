cargo build

# Test invalid urls
target/debug/curl "www.eecg.toronto.edu"
target/debug/curl "data://www.eecg.toronto.edu"
target/debug/curl "http//www.eecg.toronto.edu"
target/debug/curl "https://255.255.255.256"
target/debug/curl "https://[...1]"
target/debug/curl "http://127.0.0.1:65536"

# Test server errors
target/debug/curl "https://example.rs"
target/debug/curl "https://www.eecg.toronto.edu/~bli/ece1724/assignments/files/lab4.html"

# Test post
target/debug/curl "https://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST
target/debug/curl "dddd://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST

# Test json
target/debug/curl --json '{"title": "World", "userId": 5}' "https://dummyjson.com/posts/add"
target/debug/curl --json '{"title": "World"; "userId": 5}' "https://dummyjson.com/posts/add"