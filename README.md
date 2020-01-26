# xd
A little toy written in Rust to preview a hex dump of a given file, just like xxd 

## Compile

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended rustup installation method) and then clone this repository:

```bash
$ git clone https://github.com/Ainevsia/xd.git
$ cd xd

// build release
$ cargo build --release
```

If you are using Rust for the first time and the progress bar stucks at the first step like this: 

```bash
    Updating crates.io index
       Fetch [==================>                                    ]  34.94%
```

You should [change the source](https://blog.csdn.net/s_lisheng/article/details/80172549) of the `crates.io`. Add these lines to `$HOME/.cargo/config` (if not exists, create it):

```bash
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

and then run `cargo build --release`. The Rust compile process will take pretty much time, which involves downloading and compiling the depended packages. It took me `4m 38s` to finish.

```bash
 Finished release [optimized] target(s) in 4m 38s
```

## Available features

**Colored** output with **length**-specificed limitation.

## Usage

the binary is in the `target/release`.

![help](/image/usage-help.png)

![xd](/image/usage-xd.png)

## Contributing

1. Fork it (<https://github.com/Ainevsia/xd.git>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## Contributors

<a href="https://github.com/Ainevsia/xd/graphs/contributors">
    <img src="https://contributors-img.firebaseapp.com/image?repo=Ainevsia/xd" />
</a>

Made with [contributors-img](https://contributors-img.firebaseapp.com).

## License

**xd** © [Ainevsia](https://github.com/Ainevsia/), Released under the [MIT](./LICENSE) License.

Authored and maintained by DIYgod with help from contributors ([list](https://github.com/Ainevsia/xd/contributors)).