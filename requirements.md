# Requirements for Irrad_Control successor software

## About this document
This is only a first draft, intended to create a common basis of understand which can then be refined into technical and concrete requirements.

## Motivation
As discussed previously, the current Software, used to run experiments at the Bonn Isochronus Cyclotron, has multiple issues, first and foremost in terms of reliability and extendability. Furthermore, it requires knowledge of all components to work on any, thus making it highly impractical for everyday use of scientists.

## Requirements for an alternative
### I) Lowest possible complexity
To be practical for everyday use, any solution must reduce complexity as far as possible. To that goal implementations of specific functions should be standalone, abstraction should be kept to a reasonable level within the code, custom protocols should be encapsulated in clear text alternatives.

### II) Minimal Dependencies
For ease of portability and, in the long term, to not run into problems with external package updates, where possible, external libraries should be avoided. This will make rebuilding on different (new) systems much easier and also prevent issues from updates to external libraries.
As a consequence, where functionality is needed that is unreasonable to implement, static solutions, such as header only libraries for C or similar solutions for other languages should be considered.
This extends to the choice of language. Languages, which do not provide a stable standard, along with long term supported tooling, are not ideal.

### III) Choice of language
The chose language for the software hast to be able to compile to statically linked binary files, as that ensures, precompiled binaries can be employed relatively independent of the target system.
The chosen language should have a clear specification, which doesn't change over time.
The chosen language needs longterm support in terms of both build system, standard library and external libraries (where those have to be used)
Performance is not less important then these other considerations.
If possible it should be widely known, so that developers are easily found.

### IV) Modularity
New devices, sensors or functionalities must be able to get implemented without understanding, affecting or changing existing structures.
Equally, swapping out a specific device or sensor for a functional equivalent should be possible without interacting with the rest of the system at all.
To this end, all seperable parts of the whole accelerator infrastructure should be as seperate as possible.

## Needed functionality
The Accelerator operations need live monitoring and recording of various sensors as well as control of various electronic or electromechanical devices.
Sensors may deliver data to one or multiple consumers, which may in turn, display, process or store that data or use it in control loops and autmated processes.
Active devices being controlled may receive commands from any part of the control net, without any special hierarchy. It can safely be assumed that no malicious commands will be sent.
The specific implementations of how to address and readout the sensors, display data or run devices must be implemented on a case by case basis.
These implementations will have very limited scope. The shared software thus needs to provide unification of those components into a coherent system.

## Proposed architecture
In order to achieve all requirements, a server client architecture is proposed, in which the server will receive and send all data, such that each client must only ever communicate with the server. Messages will be strings. Messages will start with structured information (priority, sender (device and script), type of contents (request, command, data, logging, error message, etc)) followed by freeform string data. The server will tag received messages with a unique and incrementing id, thus enabling requesting messages _after_ a certain one. For common types of content, shared formats should be defined and used (for example, numerical data with units should all be formatted similarly). In cases where those common formats are not sufficient, an arbitrary but documented string can be used for the application specific needs.
Clients may send _request_ messages, specifying what they want to get sent (from sender, types, other filters). The server will then send each received message to all clients who have standing requests for that kind of message, either just future messages or all after a given message id.
The server should aim to deliver messages of high priorities at close to real time.

The network component should be handled as HTTP requests, such that clients can easily be implemented in other languages or even as shell scripts. 
Alternatively, the network component can be designed to use TCP sockets for more fine grained client monitoriung and possibly lower latencies, at the cost of increased compexity and less flexibility. Additionally this could improve the overall performance due to lower overhead. Due to the overall small scope of the accelerator this does no seem needed at this time.

Using HTTP Getting and Posting instead of some custom network protocol ensures that the client libraries only perform string packing from more convenivent arguments but no complex tasts. This will ensure that it may be easily swapped out if needed. Thus the vast majority of concentrated complexity will lie in the server implementation. All implementations of software modules should only ever interact with the client library, thus can be swapped out or updated independently and do not need to perform any complex networking.

The client libraries should be provided as as pure python libraries and C libraries. If greater simplicity may be achieved by wrapping a c library in python, that option ust be considered. 
This library should provide a minimal number of functions. The needed functionality can be implemented with only the following functions:

config_runtime(client_name: str, server_ip: str, device_name:str | None = None) -> None

send_data(data: str, message_type: str="data", topic: str="",  priority: int=0) -> status_code

request_data(filter_from, filter_type="", filter_device_name="", min_msg_id: int=-1, timeout_secs=-1) -> str | None

This would implement the needed functionality while focusing on reducing complexity as much as possible, using only parts of the C and python std libraries with external dependencies. The languages to implement this should be python for the client side library and C for the server program. This complies with the requirements set out before for languages as these offer sufficient performance with the server programs C being very longterm stable, with established and well supported infrastructure and clear standards. Using python for the client library is a tradeoff. In this case it significantly reduces complexity and allows for this to be extremly simple to write while all features needed have been supported and unchanged for a long time in python. Should any part become deprecated in the future, an alternate client library, written in another language, can be swapped out without any other components needing to change.
