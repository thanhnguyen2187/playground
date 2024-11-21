## Original Requirements

> You are provided with a MongoDB collection named transactions which
> contains documents representing financial transactions. Each document has the
> following structure: `{ _id: ObjectId, userId: Number, value: Number,
> timestamp: Date }`. Your task is to:
>
> - Write a MongoDB query to efficiently retrieve the top 10 highest value
>   transactions from this collection.
> - Discuss the indexing strategies you would employ to optimize this query.
>   Consider the performance implications for both the write and read
>   operations in your discussion.

## MongoDB Query

```javascript
db.transactions.aggregate([
    {
        $sort: { value: -1 }
    },
    {
        $limit: 10
    }
])
```

## Indexing Strategies

I would use a single field index to `value` to speed up the query. It would 
speed up the reads, but slow down the writes, as the single field index of 
MongoDB is a binary tree in the end. Without it, sorting takes `n log(n)`. 
With it enabled when we query by `value`, it will take `log(n)`. The 
trade-off is with write/insert, as it takes more time to put the new record 
into the right place in the binary tree.
