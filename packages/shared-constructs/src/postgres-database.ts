import { Construct } from 'constructs';
import * as rds from 'aws-cdk-lib/aws-rds';
import { RemovalPolicy } from 'aws-cdk-lib';

export interface PostgresDatabaseBuilderProps {
    name: string;
    username: string;
}

export class PostgresDatabaseBuilder extends Construct {
    protected _name: string;
    protected _id: string;
    protected _secret: rds.DatabaseSecret;
    protected _credentials: rds.Credentials;
    protected _cluster: rds.DatabaseCluster;
    
    constructor(scope: Construct, id: string, props: PostgresDatabaseBuilderProps) {
        super(scope, id);
        this._id = id;

        const { username } = props;

        this._secret = new rds.DatabaseSecret(this, `DatabaseSecret${id}`, {
            username,
        });
        this._credentials = rds.Credentials.fromSecret(this._secret);
    }

    build(): rds.DatabaseCluster {
        this._cluster = new rds.DatabaseCluster(this, `DatabaseCluster${this._id}`, {
            defaultDatabaseName: this._name,
            clusterIdentifier: `${this._name}-cluster`,
            engine: rds.DatabaseClusterEngine.auroraPostgres({
                version: rds.AuroraPostgresEngineVersion.VER_16_4,
            }),
            credentials: this._credentials,
            enableDataApi: true,
            removalPolicy: RemovalPolicy.DESTROY,
            serverlessV2MinCapacity: 0.5,
            serverlessV2MaxCapacity: 1,
            preferredMaintenanceWindow: 'Mon:04:00-Mon:05:00',
            writer: rds.ClusterInstance.serverlessV2('writer', {
                publiclyAccessible: false,
                autoMinorVersionUpgrade: true,
            }),
        });

        return this._cluster;
    }
}
