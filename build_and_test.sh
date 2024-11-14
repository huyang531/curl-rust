cargo build

# Test invalid urls
target/debug/curl "www.eecg.toronto.edu"
echo "-----------------------------------"
target/debug/curl "data://www.eecg.toronto.edu"
echo "-----------------------------------"
target/debug/curl "http//www.eecg.toronto.edu"
echo "-----------------------------------"
target/debug/curl "https://255.255.255.256"
echo "-----------------------------------"
target/debug/curl "https://[...1]"
echo "-----------------------------------"
target/debug/curl "http://127.0.0.1:65536"
echo "-----------------------------------"

# Test server errors
target/debug/curl "https://example.rs"
echo "-----------------------------------"
target/debug/curl "https://www.eecg.toronto.edu/~bli/ece1724/assignments/files/lab4.html"
echo "-----------------------------------"

# Test post
target/debug/curl "https://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST
echo "-----------------------------------"
target/debug/curl "dddd://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST
echo "-----------------------------------"

# Test json
target/debug/curl --json '{"title": "World", "userId": 5}' "https://dummyjson.com/posts/add"
echo "-----------------------------------"
target/debug/curl --json '{"title": "World"; "userId": 5}' "https://dummyjson.com/posts/add"
echo "-----------------------------------"