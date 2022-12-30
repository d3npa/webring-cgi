# webring-cgi
cgi program for the haunted webring (see [n0.lol/#webring](https://n0.lol/#webring))

live: https://pixeldreams.tokyo/cgi-bin/webring.cgi

## server setup

* openbsd 7.2
* httpd(8)
* slowcgi(8)

add a location rule in `/etc/httpd.conf` to trigger cgi

```
server "pixeldreams.tokyo" {
    ...
    location "/cgi-bin/webring.cgi" {
        fastcgi
    }
    ...
}
```

## building

build with `cargo build --release` then copy `target/release/webring-cgi` to your `.../cgi-bin/` directory
