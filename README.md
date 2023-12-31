Try to decompress request bodies in axum.

Curl command:
```
curl -v -g 'http://localhost:8000/' \
    -H "Content-Type: application/json" \
    -H "Content-Encoding: gzip" \
    --compressed \
    --data-binary @data/products.json.gz
```

Response:
```
*   Trying 127.0.0.1:8000...
* Connected to localhost (127.0.0.1) port 8000 (#0)
> POST / HTTP/1.1
> Host: localhost:8000
> User-Agent: curl/8.1.2
> Accept: */*
> Accept-Encoding: deflate, gzip
> Content-Type: application/json
> Content-Encoding: gzip
> Content-Length: 99
>
< HTTP/1.1 200 OK
< content-type: application/json
< content-encoding: gzip
< transfer-encoding: chunked
< date: Fri, 08 Sep 2023 11:26:42 GMT
<
* Connection #0 to host localhost left intact
{"products":[{"id":1,"name":"Product 1"},{"id":2,"name":"Product 2"}]}
```

Notice the `Content-Encoding: gzip` in the request, and `content-encoding: gzip` in the response.

non-gzip
```
curl -v -g 'http://localhost:8000/' \
    -H "Content-Type: application/json" \
    -d @data/products.json
```
