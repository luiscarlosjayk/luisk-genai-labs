import { RustLambdaFunctionBuilder } from '@luisk-genai-labs/shared-constructs/src/lambda/rust-lambda-function-builder';
import * as cdk from 'aws-cdk-lib';
import * as bedrock from 'aws-cdk-lib/aws-bedrock';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as iam from 'aws-cdk-lib/aws-iam';
import { IFunction } from 'aws-cdk-lib/aws-lambda';
import { join } from 'node:path';

export class AgentFunctionCallingStack extends cdk.Stack {
    protected _waiterLambda: IFunction;
    protected _iceCreamMakerLambda: IFunction;
    protected _agentCaller: IFunction;
    protected _table: dynamodb.Table;

    constructor(scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props);

        const foundationModelId = bedrock.FoundationModelIdentifier.ANTHROPIC_CLAUDE_3_SONNET_20240229_V1_0;
        const foundationalModel = bedrock.FoundationModel.fromFoundationModelId(this, `FoundationalModel${id}`, foundationModelId);

        this._table = new dynamodb.Table(this, `Table${id}`, {
            partitionKey: {
                name: 'order_id',
                type: dynamodb.AttributeType.STRING,
            },
        });

        this._waiterLambda = new RustLambdaFunctionBuilder(this, `WaiterLambda${id}`, {
            name: 'waiter',
            path: join(__dirname, 'lambda', 'waiter'),
        })
            .withLogGroup()
            .withDuration(10)
            .withDynamoDBTable(this._table, 'TABLE_NAME')
            .build();

        this._iceCreamMakerLambda = new RustLambdaFunctionBuilder(this, `IceCreamMakerLambda${id}`, {
            name: 'ice-cream-maker',
            path: join(__dirname, 'lambda', 'ice-cream-maker'),
        })
            .withLogGroup()
            .withDuration(10)
            .withDynamoDBTable(this._table, 'TABLE_NAME')
            .build();

        this._agentCaller = new RustLambdaFunctionBuilder(this, `AgentCallerLambda${id}`, {
            name: 'agent-caller',
            path: join(__dirname, 'lambda', 'agent-caller'),
        })
            .withLogGroup()
            .withEnvironmentVariable("MODEL_ID", foundationModelId.modelId)
            .withEnvironmentVariable("ICE_CREAM_MAKER_LAMBDA", this._iceCreamMakerLambda.functionArn)
            .withEnvironmentVariable("WAITER_LAMBDA", this._waiterLambda.functionArn)
            .withDuration(30)
            .attachInlinePolicy(
                new iam.Policy(this, `BedrockPolicy${id}`, {
                    statements: [
                        new iam.PolicyStatement({
                            effect: iam.Effect.ALLOW,
                            actions: [
                                'bedrock:InvokeModel',
                            ],
                            resources: [
                                foundationalModel.modelArn,
                            ],
                        }),
                        new iam.PolicyStatement({
                            effect: iam.Effect.ALLOW,
                            actions: [
                                'bedrock:InvokeInlineAgent',
                            ],
                            resources: [
                                `arn:aws:bedrock:${cdk.Aws.REGION}:${cdk.Aws.ACCOUNT_ID}:agent/InlineAgent`,
                            ],
                        }),
                        new iam.PolicyStatement({
                            effect: iam.Effect.ALLOW,
                            actions: [
                                'lambda:InvokeFunction',
                            ],
                            resources: [
                                this._iceCreamMakerLambda.functionArn,
                                this._waiterLambda.functionArn,
                            ],
                        }),
                    ],
                })
            )
            .build();
    }
}
