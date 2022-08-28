# ou

`ou` helps you serve a static site, single page application or just a static file (no matter if on your device or on the local network). It also provides a neat interface for listing the directory's contents:

Pre-built binaries are available from the website or alternatively on the Github Releases tab. Since 7.8.0, checksums and signatures are also provided; see download documentation for details.

```
$ cargo install ou
```

With cargo-binstall:

```sh
$ cargo binstall ou
```

## Quick example

![Tree](resources/tree.png)

Once `ou` is installed, you can run this command inside your project's directory. It will create by default in `8000` port:

```
$ cargo ou .
```

Result:

![Result ou](resources/demo.png)
