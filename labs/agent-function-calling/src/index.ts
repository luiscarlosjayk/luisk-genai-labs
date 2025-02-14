#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import 'source-map-support/register';
import { loadEnvFile } from '@luisk-genai-labs/utils/src/load-env';
import { AgentFunctionCallingStack } from './stack';
import { stackName } from '../package.json';

// Load .env file
if ('LOAD_ENVFILE' in process.env) {
    loadEnvFile();
}

const AWS_ACCOUNT = process.env.AWS_ACCOUNT || process.env.CDK_DEFAULT_ACCOUNT;
const AWS_REGION = process.env.AWS_REGION || process.env.CDK_DEFAULT_REGION;

const app = new cdk.App();

new AgentFunctionCallingStack(app, 'AgentFunctionCalling', {
    stackName,
    env: {
        account: AWS_ACCOUNT,
        region: AWS_REGION,
    },
    tags: {
        STACK: stackName,
    },
});

app.synth();
