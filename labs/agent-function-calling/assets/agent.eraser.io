title Agent Function Calling

Agent {
  ActionGroup[icon: aws-lambda] {
    OpenAPISchema[icon: swagger, label: "OpenAPI Schema"]
    FunctionDetails[icon: function, label: "Function Details"]
  }
  KnowledgeBase[icon: data]
  LLM [icon: ai] {
    Instructions[icon: file-text]
  }
}

LLM > KnowledgeBase: We won't use in this lab
LLM > ActionGroup
