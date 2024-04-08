# IOT Communications Protocol V2 Guidelines
This MarkDown file is here to layout the structure of this IOT network and some of the messaging protocols it will use to keep up consistency and reusability.


# Table Of Contents

- [Devices](#Devices)
- [Sensors](#Sensors)
- [Inputs](#Inputs)


# Device 
A IOT Device boiled down records sensory data and sends that to a dedicated db. It can also take inputs to change internal data to control external lights and switches. 

Devices have two components:
The IN and OUT
or sensors and inputs

sensors relay data back to the server and update a database
inputs allow the server to change a devices state(EX: light ON/OFF)

Devices must have these identifiers for easy cataloging and management

- [] Name:String (Identifying name for management)
- [] ID:int (Unique ID to properly target device) 


# Sensors
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

Data Types
1. Integer
2. Boolean
3. Float
4. String(Audio File)
5. String(Photo)
6. String(Video)


# Inputs
Devices cant just send data to the server, but they must also me capable of receiving inputs. This can allow them to change their local state for example turning a light switch on or off.

Inputs are sent directly to the controller as some sort of data type. 

Supported types are 
```
Integer
Float
Boolean
```

This means that devices must be listening often for external input and must also notify the server of this ability of theirs


# Messages
When communicating there is a certain set of messages that both the IOT device and server must be able to send and receive.

Every message has two parts.
1. Header
2. Data

## Header
Every message header stores the message length and the type of message to expect.

Message length (How long the message will be)
2 bytes

Message Type (Specifies what type of message it expects)
2 bytes

Types:
1. Ping (Check if recipient is still connected)
2. Update DataBase (Adds new value in table)
3. Response (Response to a previous request)
4. Connection (Holds device data)


## Data
The rest of the bytes are allocated to data
Total bytes - 2 bytes - 2 bytes

1. ping
```
Empty
```
2. Update Data Base(Table and data pairs )
```
BatteryVoltage:01001,SolarVoltage:10011
```
3. Response
```
```
4. Connection(Contains Device Data)
```json
{
    "Name":"...",
    "ID":"...",
    "Sensors":{
        "Battery Voltage":"Float",
        "Solar Power":"Float",
        "Camera":"Photo",
    },
    "Inputs":{
        "Flash":"Bool",
        "Take Photo":"Bool",
        "Flash Brightness":"Integer",
    }
}
```

## Update Database
First part of an update database message is the specifier for what table to update. This can be chained together.
```
header: BatteryVoltage:data, SolarVoltage:data
```


# Message Structure
Messages are used to transfer data between devices.
Messages must be properly structured to relay data appropriately.




# Connecting


