# Best Practices for Managing and Querying Time-series Data in MongoDB

- Make sure that you have `metaFields` ready in your collection for 
  efficient data partitioning.
- Set appropriate `granularity` depends on your data ingestion rate to 
  ensure that the grouped (put into a bucket) records' count strike is not 
  too much, but not too few either.
- Batch `insert`s (use `insertMany` instead of `insertOne`) to improve 
  performance.
- Avoid `Distinct()` and use `$group`, as the index that time series 
  collections uses is not efficient for distinct values querying.
