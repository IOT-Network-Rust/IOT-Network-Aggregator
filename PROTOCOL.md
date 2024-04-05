## Messaging Protocol

The protocol that will be the only one supported currently will be TCP
TCP offers a lot of safety as it almost guarantees data arrival with a simple
interface for most systems.


## Handshake

At the start of any conversation, you must greet the recipient you are talking to
and they must reply with their response.

When this happens we want to exchange some important information about 
both of our systems so we can properly understand each other.

When this server requests to connect to the IOT device it will send along with it
some information 
```Example
    Time
```
and in return, the device will send some of its data
```Example
    Name, Location, Sensors(with typing), Inputs(with typing)
```

The server is the only one who can request a connection with an IOT device.
The IOT device will not know the server and just will be listening for a 
socket connection from the server.


## Messaging

For this messaging system, we will be using a "send and receive" format
where for every message sent a reply is required from the other 
machine(There will be a TIMEOUT).

This means that both machines need to be capable of sending data and
responding to incoming messages
```
sending data requires these steps:
    1. Prepare message 

    2. Send message

    3. Wait for reply

    4. Get reply

    5. Handle reply
```
```
responding to data requires these steps:
    1. Wait for request

    2. Catch request

    3. Handle request 

    4. Return response
```

This functionality needs to be present in both the IOT device and 
in this server.


## Message Structure

Every message will have its bytes split up for compression and better
message management.

```
The buffer size of this messaging protocol is a constant
[0; 1024] or 1024 bytes

The first byte will specify the message type and some additional data
about its status.
(byte = 4bits = 0x0000)

The rest will allocated to data
```
