## Messaging Protocol

For this messaging system we will be using a send and receive format
where for every message sent a reply is required from the other 
machine(There will be a TIMEOUT).

This means that both machines need to be capable of sending data and
responding to incoming messages

sending data requires these steps:
    -Prepare message 
    -Send message
    -Wait for reply
    -Get reply
    -Handle reply

responding to data requires these steps:
    -Wait for request
    -Catch request
    -Handle request 
    -Return response

This functionality needs to be present in both the IOT device and 
in this server.

## Message Structure

Every message will have its bytes split up for compression and better
message management.

The first byte will specify the message type and some additional data
about its status.
(byte = 4bits = 0x0000)