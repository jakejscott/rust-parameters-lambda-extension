import { Stack, StackProps, Duration } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as lambdanodejs from "aws-cdk-lib/aws-lambda-nodejs";

export class RustParametersStackStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const parametersLambdaExtension = new lambda.LayerVersion(
      this,
      "ParametersLambdaExtension",
      {
        code: lambda.Code.fromAsset("../bin/extensions.zip"),
      }
    );

    parametersLambdaExtension.addPermission(
      "ParametersLambdaExtensionPermission",
      {
        accountId: this.account,
      }
    );

    new lambdanodejs.NodejsFunction(this, "ExampleFunction", {
      functionName: `${this.stackName}-example`,
      entry: "../functions/example.ts",
      runtime: lambda.Runtime.NODEJS_14_X,
      timeout: Duration.seconds(10),
      memorySize: 1024,
      layers: [parametersLambdaExtension],
    });
  }
}
