# Serverless Geo Location

[MaxMind](https://www.maxmind.com/) provides IP intelligence through the GeoIP brand. Over 5,000 companies use GeoIP data to locate their Internet visitors, show them relevant content and ads, perform analytics, enforce digital rights, and efficiently route Internet traffic.
[GeoIP2 IP](https://dev.maxmind.com/geoip) intelligence products and services can provide data on geolocation, network information, anonymizer status and offer three types of data

- Binary Database
- CSV Database
- WebService

This repository is how to use the Binary Database in a serverless context using the crates [maxminddb](https://crates.io/crates/maxminddb)

## Requirements

* [Create an AWS account](https://portal.aws.amazon.com/gp/aws/developer/registration/index.html) if you do not already have one and log in. The IAM user that you use must have sufficient permissions to make necessary AWS service calls and manage AWS resources.
* [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) installed and configured
* [Git Installed](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
* [AWS Serverless Application Model](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) (AWS SAM) installed
* [Rust](https://www.rust-lang.org/) 1.56.0 or higher
* [cargo-zigbuild](https://github.com/messense/cargo-zigbuild) and [Zig](https://ziglang.org/) for cross-compilation

## Documentation
- [Working with HTTP APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api.html)
- [Working with AWS Lambda proxy integrations for HTTP APIs](https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-integrations-lambda.html)
- [AWS Lambda - the Basics](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/aws-lambdathe-basics.html)
- [Lambda Function Handler](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-handler.html)
- [Function Event Object - Overview](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-event-object.html)
- [Function Event Object - HTTP API v2 Event](https://github.com/awsdocs/aws-lambda-developer-guide/blob/master/sample-apps/nodejs-apig/event-v2.json)
- [Function Context Object - Overview](https://docs.aws.amazon.com/whitepapers/latest/serverless-architectures-lambda/the-context-object.html)
- [Function Context Object in Node.js - Properties](https://docs.aws.amazon.com/lambda/latest/dg/nodejs-context.html)
- [Function Environment Variables](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html)

## Deployment Instructions

The SAM template deploys an Amazon API Gateway HTTP API and an AWS Lambda function.

1. Create a new directory, navigate to that directory in a terminal and clone the GitHub repository:
    ``` 
    git clone https://github.com/ymwjbxxq/rust_geoip
    ```
2. Change directory to the pattern directory:
    ```
    cd rust_geoip
    ```
3. Install dependencies and build:
    ```
    make build
    ```
4. From the command line, use AWS SAM to deploy the AWS resources for the pattern as specified in the template.yml file:
    ```
    make deploy
    ```
5. During the prompts:
    * Enter a stack name
    * Enter the desired AWS Region
    * Allow SAM CLI to create IAM roles with the required permissions.

    Once you have run `sam deploy -guided` mode once and saved arguments to a configuration file (samconfig.toml), you can use `sam deploy` in future to use these defaults.

6. Note the outputs from the SAM deployment process. These contain the resource names and ARNs which are used for testing.

### Testing

**Binary files are not provided**

Once the application is deployed, retrieve the HttpApiEndpoint value from CloudFormation Outputs. Then, either browse to the endpoint in a web browser or call the endpoint from Postman.

Example GET Request: https://{HttpApiId}.execute-api.{region}.amazonaws.com/countrycode

Response:
```
{
  "countrycode": "IT"
}

```


## Cleanup
 
1. Delete the stack
    ```bash
    make delete
    ```
----
