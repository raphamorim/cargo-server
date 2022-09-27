# ou

`ou` helps you serve a static site, single page application or just a static file (no matter if on your device or on the local network). It also provides a neat interface for listing the directory's contents.

Pre-built binaries are available on the [Github Releases tab](https://github.com/raphamorim/ou/releases).

You can use cargo to install:

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
$ ou
```

To specify the port, you can use `--port`:

```
$ ou --port 3000
```

To open in your browser after run the command just add `--open`:

```
$ ou --open
```

You can also set a custom path instead of the root:

```
$ ou ../path-to-site
```

Result:

![Result ou](resources/demo.png)
