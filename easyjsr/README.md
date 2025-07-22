# easyjsr
Default runtime for easyjs. Built on top of rquickjs[https://crates.io/crates/rquickjs/0.9.0]


## usage
To use with `easyjs` you can set the runtime to `easyjsr`.

```bash
easyjs repl --runtime easyjsr
> import 'std'
> @print('Hello World')
```

This is not ready yet and only supports VERY BASIC JS.