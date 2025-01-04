User[icon: user, label: "User query"]
PlanNextStep[label: "Plan next step"]
ExecuteStep[label: "Execute step", color: blue]
ObserveOutput[label: "Observe Output"]
AnotherIteration[shape: diamond, label: "Another iteration?", color: yellow]
FinalAnswer[label: "Final answer"]


User > PlanNextStep
PlanNextStep > ExecuteStep
ExecuteStep > ObserveOutput
ObserveOutput > AnotherIteration
AnotherIteration > FinalAnswer: No
AnotherIteration > PlanNextStep: Yes
a