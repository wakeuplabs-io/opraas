
## Debug

1. Install [cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html)
2. Run `cargo lambda watch`
2. Base url is `http://localhost:9000/lambda-url/opraas_server`, so for example you can try `curl http://localhost:9000/lambda-url/opraas_server/health`

## Build & Deploy

1. Install [cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html)
2. Build the function with `cargo lambda build --package opraas_server --release`
3. Deploy the function to AWS Lambda with `cargo lambda deploy opraas_server --tag customer=op-ruaas --enable-function-url`
