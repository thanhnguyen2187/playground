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

curl -i 127.0.0.1:4004/v1/get/5;

echo 'GET key1' | nc localhost 4004 -N &;
echo 'GET key2' | nc localhost 4004 -N &;
echo 'GET key3' | nc localhost 4004 -N &;
