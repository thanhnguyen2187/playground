curl -X POST \
    --data-raw '{"hello": "world"}' \
    127.0.0.1:3000/process

curl -X POST \
    --data-raw '
{
    "transactionId": "12345",
    "amount": 100.00,
    "status": "pending"
}' \
    127.0.0.1:3000/process
