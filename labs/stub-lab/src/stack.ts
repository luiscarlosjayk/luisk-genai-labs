import { RustLambdaFunctionBuilder } from '@luisk-genai-labs/shared-constructs/src/lambda/rust-lambda-function-builder';
import * as cdk from 'aws-cdk-lib';
import { IFunction } from 'aws-cdk-lib/aws-lambda';
import { join } from 'node:path';

export class StubStack extends cdk.Stack {
    protected _lambda: IFunction;
    
    constructor(scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props);
        
        this._lambda = new RustLambdaFunctionBuilder(this, `Lambda${id}`, {
            name: 'stub-lambda',
            path: join(__dirname, 'lambda', 'stub-lambda'),
        })
        .withLogGroup()
        .build();
    }
}
