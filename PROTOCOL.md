## Messaging Protocol

The protocol that will be the only one supported currently will be TCP
TCP offers a lot of safety as it almost guarantees data arrival with a simple
interface for most systems.

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

MessageTypes
(0) ping
(1) connect
(2) sending data
(3) responding


The second byte will specify what data-point to update
(byte = 4bits = 0x0000)

Specify what sensor and what data point to update


The rest will be the data in the form of key value pairs for now until
a compression algo is thought up
```

## Connecting

When a device attempts to connect it will send some data about itself like what it does, who it
is and some of the things it might need to track.

Its response will look something like this

```JSON
{
  "Name":"Solar Lamp",
  "ID": 235321,
  "Location": "Outside",
  "Sensors": [
    {
      "Name": "Battery_Level",
      "Type": "Voltage/Time",
      "DataType": "f64",
      "Timestamp": "time"
    },
    {
      "Name": "Solar_Charging",
      "Type": "SolarVoltage",
      "DataType": "f64",
      "Timestamp": "time"
    }
  ],
  "Inputs": [
    {
      "Name": "Lamp On/Off",
      "Data": [true]
    },
    {
      "Name": "Auto Adjust",
      "Data": [true]
    }
  ]
}
```

with this data the server can properly understand the device and know whether it
has history with it and can know how to deal with it in the future
