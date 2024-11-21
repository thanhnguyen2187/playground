## Original Requirements

> You are working with a trading application that stores large volumes of
> trading data in a MongoDB database. Over time, the database accumulates a
> significant amount of historical data, which can impact performance and
> storage efficiency.
> 
> Discuss the best practices for managing and archiving old trading data in a
> MongoDB database to maintain performance and storage efficiency. Your response
> should include strategies for archiving data, optimizing queries, index
> management, and any relevant MongoDB features or tools. Provide a thorough
> explanation of each practice and its benefits.

## Strategy

I would put the data into a time series collection to help with archiving 
and querying, as MongoDB has that specialized type of collection for that. 
In more details, it has `metaFields`, which acts as specialized keys and are
auto-indexed. For queries optimizing, we should also batch the writes (
use `insertMany` instead of `insertOne`), and avoid `Distinct` as it does not
fit with the way time series collection works. 
