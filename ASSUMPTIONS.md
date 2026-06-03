# Assumptions

## 1. Packet boundaries are well-formed
Each packet is assumed to start with a correct sync marker (A5 F0 EE) and end with a correct termination sequence (EE 0F 5A). I do not attempt to detect cases where the sync marker or termination bytes appear by coincidence within the data payload of another packet. For simplicity, each packet is treated as having clean start and end boundaries.

## 2. Header field order follows the datasheet diagram
The diagram shows the header field order as: Sync | Type | Src ID | Dst ID | Data Length | Reserved. This differs from the written description in the datasheet, which places Reserved before Data Length.

## 3. Only bytes between different packets are considered not useful data
Bytes that fall between packet boundaries are the only data considered "not useful". Bytes that belong to an invalid packet are not considered "not useful" but they are still part of a packet, just an invalid one.

## 4. Null packets are discarded
The datasheet says that Null packets should be ignored and that no guarantees are made about their contents. They are therefore discarded entirely.

## 5. Sender and receiver are validated based on packet type
The datasheet describes which device sends (and receives) each packet type:

- The sender must match the packet type: Request, OutboundData and Command are sent by the controller; Telemetry, InboundData and AckNack are sent by OOPS nodes.
- The destination must also match: Telemetry, InboundData and AckNack must go to the controller only (no broadcast). OutboundData must go to a specific OOPS node (the datasheet says "a specific OOPS system", so broadcast is not allowed). Request and Command may be broadcast.

## 6. Checksum bytes are stored little-endian (LSB first)
The datasheet does not specify the byte order of the 2-byte checksum field. Reading it as big-endian (MSB first) produced only 7 valid packets and 1012 invalid (of which 932 were checksum failures). Switching to little-endian (LSB first) gave 919 valid packets and only 100 invalid, confirming little-endian is the correct interpretation.