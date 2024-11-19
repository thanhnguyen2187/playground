## MongoDB Query

```javascript
db.users.aggregate([
    {
        $sort: { age: 1 },
    },
    {
        $match: {
            joinedDate: { $gt: new Date('2024-01-01') }
        },
    }
])
```

## Indexing Strategies

Depends on the queries, I would propose two options:

- One compound index for both `age` and `joinedDate`: if there are many queries 
  that involve both
- Two single field indexes for both `age` and `joinedDate`: if there are queries
  that involve each, and not many queries that involve both

The queries would reduce read time, but increase write (insert) time. The 
core idea is that the indexes are binary trees. They help reduce range queries'
complexity from `O(n)` to `O(log n)`. The trade-off is that insertion means 
creating a new item in the tree, and the complexity comes from `O(1)` without 
indexing to `O(log n)`.
