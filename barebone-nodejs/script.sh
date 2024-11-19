curl -X POST \
    --data-raw '{
        "username": "thanh",
        "password": "thanh123",
        "email": "thanh@thanh"
    }' \
    localhost:3001/register \
;

curl -X POST \
    localhost:3000/compress \
;
