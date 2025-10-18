Example for salvo [issue 1218](https://github.com/salvo-rs/salvo/issues/1218)

`curl -v "http://localhost:8080" --compressed -H "Accept-Encoding: br"`

<code>
* Host localhost:8080 was resolved.<br>
* IPv6: ::1<br>
* IPv4: 127.0.0.1<br>
*   Trying [::1]:8080...<br>
*   Trying 127.0.0.1:8080...<br>
* Connected to localhost (127.0.0.1) port 8080<br>
* using HTTP/1.x<br>
&gt; GET / HTTP/1.1<br>
&gt; Host: localhost:8080<br>
&gt; User-Agent: curl/8.14.1<br>
&gt; Accept: */*<br>
&gt; Accept-Encoding: br<br>
&gt; <br>
&lt; HTTP/1.1 200 OK<br>
&lt; content-disposition: inline<br>
&lt; content-type: <span style="color:red">text/plain</span>; charset=utf-8<br>
&lt; last-modified: Sat, 18 Oct 2025 16:08:36 GMT<br>
&lt; etag: "0-3a-68f3bb84-0"<br>
&lt; accept-ranges: bytes<br>
&lt; content-encoding: br<br>
&lt; content-length: 58<br>
&lt; date: Sat, 18 Oct 2025 16:10:41 GMT<br>
</code>

`content-type` is `text/plain`

It should be `text/html`

This was introduced by [commit 0cdfaa9](https://github.com/salvo-rs/salvo/commit/0cdfaa98f349e9ea22798c779095495ab173c509).

You can revert to salvo version 0.83 to compare with previous (correct) behavior.