# Notes

- Built on UDP
- Can only read and write files (or mail) from/to remote server.
- Begins with a request to read or write a file
- Fixed length blocks of 512 bytes
    - Data less than 512 bytes  signals termination of transfer
- 2 byte opcode in header
- Acknowledgement packet will contain the block number of the data packet being acknowledged
- Transfer identifiers are passed to the Datagram layer to be used as ports
- 1 Read request RRQ
- 2 Write request WRQ
- 3 Data DATA
- 4 Acknowledgement ACK
- 5 Read ERROR


**RRQ / WRQ**: | Opcode (2 bytes) | Filename (string) | 0 (1 byte) | Mode (string) | 0 (1 byte) |

- Mode
    - netascii
    - octet
    - mail

**DATA packet**: | Opcode | Block # | Data |

**ACK packet**: | Opcode | Block # |

**ERROR packet**: | Opcode | ErrorCode | ErrMsg | 0 |

