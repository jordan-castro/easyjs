# easyjsr
Default runtime for easyjs. Built on top of rquickjs[https://crates.io/crates/rquickjs/0.9.0]


## usage
To use with `easyjs` you can set the runtime to `easyjsr`.

```bash
easyjs repl --runtime easyjsr
> import 'std'
> @print('Hello World')
```

Realize there is still a bit missing in the easyjs runtime. To have a look at what is missing vs what has been implemented check: implemented.md