Try to decompress request bodies in axum.

Curl command:
```
curl -v -g 'http://localhost:8000/' \
    -H "Content-Type: application/json" \
    -H "Content-Encoding: gzip" \
    --data-binary @data/products.json.zip
```

Response:
```
*   Trying 127.0.0.1:8000...
* Connected to localhost (127.0.0.1) port 8000 (#0)
> POST / HTTP/1.1
> Host: localhost:8000
> User-Agent: curl/8.1.2
> Accept: */*
> authorization: Bearer 1234
> Content-Type: application/json
> Content-Encoding: gzip
> Content-Length: 559
>
< HTTP/1.1 400 Bad Request
< content-type: text/plain; charset=utf-8
< content-length: 54
< date: Wed, 30 Aug 2023 11:43:02 GMT
<
* Connection #0 to host localhost left intact
Failed to buffer the request body: Invalid gzip header%
```

non-gzip
```
curl -v -g 'http://localhost:8000/' \
    -H "Content-Type: application/json" \
    -d @data/products.json
```
