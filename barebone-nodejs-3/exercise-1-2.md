# Microservices Architecture for a High-frequency Trading Application

## Original Requirements

> You are tasked with designing a microservices architecture for a
> high-frequency trading application. The application must be able to handle a
> large number of trades per second while ensuring low latency and high
> reliability. Your design should cover the following areas:
>
> - Service Modularization: How would you break down the application into
>   individual services? Please list the services and explain the key
>   functionality of each.
> - Inter-service Communication Mechanisms: Discuss the communication mechanisms
>   you would employ between the services. Explain your choice of communication
>   protocols (e.g., REST, gRPC, message queues) and why they are suitable for
>   this application.
> - Scalability Considerations: Describe how you would ensure that the system
>   can scale to handle increasing numbers of trades. Include details on load
>   balancing, horizontal scaling, and any other strategies you think are
>   important.
> - Data Consistency and Reliability: Explain how you would ensure data
>   consistency and reliability across the services, especially considering the
>   high volume of trades.
>
> Please provide a detailed explanation for each aspect. Use examples where 
> necessary to illustrate your points.

## Service Modularization

Based on my understanding of quantitative trading, I would imagine the system at
it simplest is a black box that receives data, and execute trade instructions
(to buy or to sell particular tickers). Going into more details, I think the 
black box would have these components:

- Data Feeds Handler: when we receive data from other sources, we need to 
  have some check and parsing before we can put it to use. 
- Historical Data Storage: in this place, we store data for later strategies 
  back testing.
- Alpha Generator: from existing data and coming data, we build the states, and 
  give out decisions (is selling or buying a good idea)
- Execution Engine: this component receives the decisions from the Alpha 
  Generator, and then connect to other sources to execute the trades

## Inter-service Communication Mechanisms

Because this is for high-frequency trading, for most communication lines, I 
would go with gRPC, a binary protocol, or even raw TCP packets parsing. For 
a less performance-critical line: between Data Feeds Handler and Historical 
Data Storage, we can go with message queues like RabbitMQ or Kafka to 
throttle the throughput/avoid overwhelming the storage.

## Scalability Considerations

Data Feeds Handler should not have big scaling problem as we should know and 
verify and have throughout check the data sources. For Historical Data 
Storage, it's the best if we come up with a S3-compatible service, as they 
are scalable and easy to move. For Alpha Generator, scaling might be 
challenging as it is stateful. We should use have redundancy for Alpha 
Generator, and put a Load Balancer with sticky session before to ensure that 
the Alpha Generator always available no matter what. Execution Engine is 
stateless, so we can scale its process count up or down as needed.

## Data Consistency and Reliability

As mentioned before, I would go with a Historical Data Storage for past data 
to ensure reliability/the services, especially Alpha Generator, can always be 
rebuilt if needed. Before letting the data in, we might need strict schema 
checking Data Feeds Handler to ensure the correctness of the data.
