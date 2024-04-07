## IOT Communications Protocol V2 Guidelines
This MarkDown file is here to layout the structure of this IOT network and some of the messaging protocols it will use to keep up consistency and reusability.


## Table Of Contents

- [Devices](#Devices)
- [Sensors](#Sensors)
- [Inputs](#Inputs)


## Device 
A IOT Device boiled down records sensory data and sends that to a dedicated db. It can also take inputs to change internal data to control external lights and switches. 

Devices have two components:
The IN and OUT
or sensors and inputs

sensors relay data back to the server and update a database
inputs allow the server to change a devices state(EX: light ON/OFF)

Devices must have these identifiers for easy cataloging and management

- [] Name:String (Identifying name for management)
- [] ID:int (Unique ID to properly target device) 


## Sensors
Any sensory data that comes from an IOT device will have its data formate in this method.

Data will be sent to the server one of a few supported kinds.
Then that data will be tagged ALWAYS with a time stamp from the moment the server receives the data.

This simplification makes data handling much much easier for the server.


The server will aggregate this data into a dedicated db for every device. Each DB will have its own dedicated table to each sensor storing DATA with TIMESTAMP.

```
Path
│
└── DeviceName:ID.db
    │
    ├── Temperature(C)
    │   ├── (78.5, 2018:12:7:13:6:25)
    │   ├── (78.9, 2018:12:7:13:6:25)
    │   └── (78.2, 2018:12:7:13:6:25)
    │ 
    ├── BatteryVoltage(V)
    │   ├── (78.5, 2018:12:7:13:6:25)
    │   ├── (78.9, 2018:12:7:13:6:25)
    │   └── (78.2, 2018:12:7:13:6:25)
    │
    └── Humidity(%)
        ├── (78.5, 2018:12:7:13:6:25)
        ├── (78.9, 2018:12:7:13:6:25)
        └── (78.2, 2018:12:7:13:6:25)
```

This is the format that every database will take.
This layer is here to better organize the data so that you can process it easier upstream.


The DATA and TIME STAMP format works great but for data types like videos, photos, and audio files we run into issues.

Databases cannot store large files like these therefore instead the database will store Strings pointing to this data in its own dedicated folder.

So instead the format will be FILE PATH and TIMESTAMP

This allows for easier access and storage and allows programers to determine when they will send a file as most the time these sensors wont be very eventful.


## Inputs
Devices cant just send data to the server, but they must also me capable of receiving inputs. This can allow them to change their local state for example turning a light switch on or off.

Inputs are sent directly to the controller as some sort of data type. 

Supported types are 
```
Integer
Float
Boolean
```

This means that devices must be listening often for external input and must also notify the server of this ability of theirs


## Messages
When communicating there is a certain set of messages that both the IOT device and server must be able to send and receive

- [] Ping (Checks if recipient is awake)
- [] Update DataBase (Adds a new value to specified database table)
- [] Response (Responds to some request)
- [] 
 

Message length (How long the message will be)
2 bytes

Message Type (Specifies what type of message it expects)
2 bytes

The rest of the bytes are allocated to data
Total bytes - 2 bytes - 2 bytes


## Message Structure
Messages are used to transfer data between devices.
Messages must be properly structured to relay data appropriately.




## Connecting


