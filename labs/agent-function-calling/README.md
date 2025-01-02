# Lab: Agent Function Calling

Contributors: [@luiscarlosjayk](https://github.com/luiscarlosjayk)
----

Description of the project should go here.

Ice Cream Shop

## Files

1. [Lambda function](./src/lambda/stub-lambda)
2. [CDK Project](./src/index.ts)
3. [CDK Stack](./src/stack.ts)

## Architecture

Architecture diagram or explanation goes here.

## How to Deploy

```bash
#!/bin/bash
pnpm install
cd stub-lab
pnpm run cdk deploy
```

## How to Run

Explanation and steps to run and check expected results of this lab.

## Agents

### Waiter

Actions:
- StartOrder: Creates order with client's name
- AddIceCream: Adds an ice cream flavor to an order.
- RemoveIceCream: Removes and ice cream flavor from an order.



| Action Name | Parameter Name | Parameter Type |
| ---- | ---- | ---- |
| StartOrder |||
|| client_name | string |
| AddIceCream |||
|| order_id | string |
|| flavor | string |
| RemoveIceCream |||
|| order_id | string |
|| flavor | string |

### Ice Maker

Actions:
- PrepareIceCream: Prepares an ice cream flavor in the ice cream maker machine.

| Action Name | Parameter Name | Parameter Type |
| ---- | ---- | ---- |
| PrepareIceCream |||
|| flavor | string |

## DynamoDB

| order_id (string)  | client_name (string) | flavors (List) |
| ---- | ---- | ---- |
| 67e55044-10b1-426f-9247-bb680e5fe0c8 | Nancy | ["Vanilla", "Chocolate"] |

## Inspiration:

- [Stack for creating an agent with function definitions](https://github.com/aws-samples/amazon-bedrock-samples/blob/main/agents-and-function-calling/bedrock-agents/agent-blueprint-templates/lib/stacks/01-agent-with-function-definitions/agent-with-function-definition-stack.ts)
