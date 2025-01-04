LLM[icon: ai, label: "Agent"] {
  ActionGroups {
    Waiter[icon: aws-lambda] {
      API[icon: swagger, label: "OpenAPI schema"] {
        CreateOrder[icon: function, label: "create order"]
        Addflavor[icon: function, label: "add flavor"]
        Remove flavor[icon: function, label: "remove flavor"]
      }
    }
    IceCreamMaker[icon: aws-lambda, label: "Ice Cream Maker"] {
      FunctionDetails[icon: function, label: "Function Detail schema"] {
        PrepareIceCream[icon: function, label: "prepare ice cream"]

      }
    }
  }
}
AgentCaller[icon: aws-lambda, label: "Agent Caller (Client)"]
AgentCaller > LLM