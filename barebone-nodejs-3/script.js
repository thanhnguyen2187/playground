db.transactions.insertOne({userId: 1, value: 6, timestamp: Date.now()})

db.transactions.find()

db.transactions.aggregate([
    {
        $sort: { value: -1 }
    },
    {
        $limit: 10
    }
])
