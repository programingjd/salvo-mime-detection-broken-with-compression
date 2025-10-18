Example for salvo [issue 1218](https://github.com/salvo-rs/salvo/issues/1218)

Go to http://localhost:8080 with a browser. You can see that the content-type is not `text/html` but `text/plain`.

---

`curl -v "http://localhost:8080" --compressed -H "Accept-Encoding: br"`

<pre>
* Host localhost:8080 was resolved.
* IPv6: ::1
* IPv4: 127.0.0.1
*   Trying [::1]:8080...
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080
* using HTTP/1.x
&gt; GET / HTTP/1.1
&gt; Host: localhost:8080
&gt; User-Agent: curl/8.14.1
&gt; Accept: */*
&gt; Accept-Encoding: br
&gt; 
&lt; HTTP/1.1 200 OK
&lt; content-disposition: inline
&lt; content-type: <span style="color:red">text/plain</span>; charset=utf-8
&lt; last-modified: Sat, 18 Oct 2025 16:08:36 GMT
&lt; etag: "0-3a-68f3bb84-0"
&lt; accept-ranges: bytes
&lt; content-encoding: br
&lt; content-length: 58
&lt; date: Sat, 18 Oct 2025 16:10:41 GMT
</pre>

`content-type` is `text/plain`

It should be `text/html`

This was introduced by [commit 0cdfaa9](https://github.com/salvo-rs/salvo/commit/0cdfaa98f349e9ea22798c779095495ab173c509).

You can revert to salvo version 0.83 to compare with previous (correct) behavior.