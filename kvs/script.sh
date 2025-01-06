curl 127.0.0.1:4004/v1/get/2 &;
curl 127.0.0.1:4004/v1/get/3 &;
curl 127.0.0.1:4004/v1/get/4 &;

curl -X POST \
    127.0.0.1:4004/v1/set/2/abcd &;
curl -X POST \
    127.0.0.1:4004/v1/set/3/defg &;
curl -X POST \
    127.0.0.1:4004/v1/set/4/ghij &;

curl -X POST \
    127.0.0.1:4004/v1/rm/3 &;
curl 127.0.0.1:4004/v1/get/3 &;

curl 127.0.0.1:4004/v2

curl 127.0.0.1:4004/v1/get/1;
