# Create your first Netlify serverless function with Rust

The packages being used in this crate are -

`tokio` is an async runtime and we need it because the function signature we'll be working with from `lambda_runtime` is async.

`lambda_runtime` is the Rust runtime for AWS Lambda maintained by a team at AWS, which is what Netlify runs on behind the scenes. It's responsible for the mechanics of how lambda requests get handled.

`http` is a crate that contains a set of general purpose HTTP types. Things like `StatusCode` or constants for headers like `ACCEPT` and `CONTENT_TYPE`.

`aws_lambda_events` is a package full of types for various AWS Lambda events. Since Netlify Functions only interacts with the API Gateway types, we'll be using those here, but these types can also be used when deploying say, AppSync resolvers or Cognito hooks on AWS.
