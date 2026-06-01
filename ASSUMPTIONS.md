# Assumptions

## 1. Packet boundaries are well-formed
Each packet is assumed to start with a correct sync marker (A5 F0 EE) and end with a correct termination sequence (EE 0F 5A). I do not attempt to detect cases where the sync marker or termination bytes appear by coincidence within the data payload of another packet. For simplicity, each packet is treated as having clean start and end boundaries.

## 2. Header field order follows the datasheet diagram
The diagram shows the header field order as: Sync | Type | Src ID | Dst ID | Data Length | Reserved. This differs from the written description in the datasheet, which places Reserved before Data Length.
Verified empirically: temporarily swapping OFFSET_DATA_LEN to offset 8 and OFFSET_RESERVED to offset 6 (text order) caused the parser to panic with an out-of-bounds error, because the byte at offset 8 produced data_len values far exceeding the file length. Restoring to diagram order (OFFSET_DATA_LEN=6, OFFSET_RESERVED=7) gives 940 valid packets.

## 3. Padding counts only inter-packet bytes *(da chiedere)*
Only bytes that fall outside any packet boundary (i.e. bytes not part of any sync-to-termination sequence) are counted as padding. The bytes of invalid packets are not counted as padding, they are reported separately as invalid packets.

## 4. Null packets are considered valid without any validation
The datasheet states that Null packets (type 00) should be ignored and that no guarantees are made about their contents. I therefore accept them as-is: once the sync marker and type byte are identified, I scan forward to the next termination sequence and mark the packet as valid, without reading or validating any other field.

## 5. Checksum bytes are stored little-endian (LSB first)
The datasheet does not specify the byte order of the 2-byte checksum field. Reading it as
big-endian (MSB first) produced only 28 valid packets and 1012 invalid (of which 932 were checksum
failures). Switching to little-endian (LSB first) gave 940 valid packets and only 100 invalid, confirming little-endian is the correct interpretation.