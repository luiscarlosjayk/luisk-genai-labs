import { RustLambdaFunctionBuilder } from '@luisk-genai-labs/shared-constructs/src/lambda/rust-lambda-function-builder';
import * as cdk from 'aws-cdk-lib';
import * as bedrock from 'aws-cdk-lib/aws-bedrock';
import * as iam from 'aws-cdk-lib/aws-iam';
import { IFunction } from 'aws-cdk-lib/aws-lambda';
import { S3EventSourceV2 } from 'aws-cdk-lib/aws-lambda-event-sources';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { join } from 'node:path';

export class ZeroShotChatWithDocumentStack extends cdk.Stack {
    protected _bucket: s3.Bucket;
    protected _lambda: IFunction;
    
    constructor(scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props);
        
        const foundationModelId = bedrock.FoundationModelIdentifier.ANTHROPIC_CLAUDE_3_SONNET_20240229_V1_0;
        const foundationalModel = bedrock.FoundationModel.fromFoundationModelId(this, `FoundationalModel${id}`, foundationModelId);
        
        this._bucket = new s3.Bucket(this, `Bucket${id}`, {
            bucketName: `zero-shot-chat-with-document-${id.toLowerCase()}`,
        });
        
        this._lambda = new RustLambdaFunctionBuilder(this, `Lambda${id}`, {
            name: 'zero-shot-chat-with-document',
            path: join(__dirname, 'lambda', 'zero-shot-chat-with-document'),
        })
        .withLogGroup()
        .withDuration(30)
        .withBucket(this._bucket, 'BUCKET_NAME')
        .withEnvironmentVariable('MODEL_ARN', foundationalModel.modelArn)
        .withEnvironmentVariable('AWS_LAMBDA_DISABLE_CLOUDWATCH_LOGS_DATA_PROTECTION', 'true')
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
                            'bedrock:RetrieveAndGenerate',
                        ],
                        resources: [
                            '*',
                        ],
                    }),
                ],
            })
        )
        .build();
        
        // Invoke lambda function when a file is uploaded to S3 bucket
        this._lambda.addEventSource(
            new S3EventSourceV2(this._bucket, {
                events: [
                    s3.EventType.OBJECT_CREATED,
                ],
            })
        );
    }
}
