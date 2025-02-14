import * as cdk from 'aws-cdk-lib';
import * as appsync from 'aws-cdk-lib/aws-appsync';
import { IFunction } from 'aws-cdk-lib/aws-lambda';
import * as logs from 'aws-cdk-lib/aws-logs';

export class WebSocketAITranslation extends cdk.Stack {
    protected _lambda: IFunction;

    constructor(scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props);

        const apiKeyProvider: appsync.AppSyncAuthProvider = {
            authorizationType: appsync.AppSyncAuthorizationType.API_KEY,
        };

        const api = new appsync.EventApi(this, `EventApi${id}`, {
            apiName: 'websocket-ai-translation',
            ownerContact: 'luisk-genai-labs',
            authorizationConfig: {
                authProviders: [
                    apiKeyProvider,
                ],
                connectionAuthModeTypes: [
                    appsync.AppSyncAuthorizationType.API_KEY,
                ],
                defaultPublishAuthModeTypes: [
                    appsync.AppSyncAuthorizationType.API_KEY,
                ],
                defaultSubscribeAuthModeTypes: [
                    appsync.AppSyncAuthorizationType.API_KEY,
                ],
            },
            logConfig: {
                fieldLogLevel: appsync.AppSyncFieldLogLevel.INFO,
                retention: logs.RetentionDays.ONE_DAY,
            },
        });

        api.addChannelNamespace('ChatChannel', {
            channelNamespaceName: 'chat',
        });

        api.addChannelNamespace('PrivateChatChannel', {
            channelNamespaceName: 'private-chat',
        });


        // this._lambda = new RustLambdaFunctionBuilder(this, `Lambda${id}`, {
        //     name: 'stub-lambda',
        //     path: join(__dirname, 'lambda', 'stub-lambda'),
        // })
        // .withLogGroup()
        // .build();

        new cdk.CfnOutput(this, 'OutputHttpDns', {
            value: api.httpDns,
            exportName: 'websocket-ai-translation-http-dns',
            description: 'The HTTP DNS of the AppSync API',
        });

        new cdk.CfnOutput(this, 'OutputRealtimeDns', {
            value: api.realtimeDns,
            exportName: 'websocket-ai-translation-realtime-dns',
            description: 'The realtime DNS of the AppSync API',
        });

        new cdk.CfnOutput(this, 'OutputApiKeys', {
            value: Object.values(api.apiKeys).map((key) => key.attrApiKey).join(','),
            exportName: 'websocket-ai-translation-api-keys',
            description: 'The API keys of the AppSync API',
        });

        new cdk.CfnOutput(this, 'OutputLogGroup', {
            value: api.logGroup.logGroupName,
            exportName: 'websocket-ai-translation-log-group',
            description: 'The log group of the AppSync API',
        });
    }
}
