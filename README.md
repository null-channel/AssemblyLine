# Assembly Line
Our goal is that Assembly line can lay the groudwork for modernizing your build and testing pipelines, regardless of your cloud, code or infra. Have you ever wanted to run the same pipeline to build your project, test your project or you know, lint it locally the same way your build server can?

This project is young and moving fast! Join in the discord and our office hours if you would like to help out! Otherwise checkout out our [getting started](docs/getting_started.md)!

## High level design

### Goals
We set out with a few goals in mind.
1. agent and server are disconected, meaning how an agent does something is none of the servers business. While the initial community supported agent will run much like dagger leveraging containers for repeatable builds, this opens it up to letting there be agents like a mac agent that does not leverage containers. This also lets us build other agents for other env's like a kubernetes agent or even an argo workflow agent.
1. Local builds just like server builds. As a dev, I want to be able to run `al run pipeline x` and run the x pipeline just like the server would, meaning I can test my pipeline while using it.
1. Configuration by yaml or lua (lua support is a "when we get to it")
1. Branching and converging logic will be supported.
1. al pipelines can be used without any server meaning the tool could be leveraged without any need of a server running for pure local pipelines.
1. the server will be responsible for all things veriable, event and secret driven
1. Server should be able to leverage any platform (aka, kubernetes) but does not need kubernetes to run

### HLD

We will use an agent server architecture, the server does not run pipelines, just manages who does what when.


Office hours will be held every first the third tuesday of a month.

Join the discord and start talking all things assembly line with us!
[discord](https://discord.gg/SaJFQ8V)

[meeting notes](https://docs.google.com/document/d/1Tx57fBuWZfVuG4Qu5IBoVEy7O17-7fRpiPF1Yph8i9w/edit?usp=sharing)

Thanks!
