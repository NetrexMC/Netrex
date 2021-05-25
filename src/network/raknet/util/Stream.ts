/**
 *  _   _      _
 * | \ | |    | |
 * |  \| | ___| |_ _ __ _____  __
 * | . ` |/ _ \ __| '__/ _ \ \/ /
 * | |\  |  __/ |_| | |  __/>  <
 * |_| \_|\___|\__|_|  \___/_/\_\
 *
 * stream program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * @author Netrex Team
 * @link https://github.com/NetrexMC
 *
 * © Netrex 2020 - 2021
 */
import {
	BinaryStream as Stream
} from 'https://raw.githubusercontent.com/RaptorsMC/BinaryUtils/master/mod.ts';
import Address from "../../common/Address.ts";
export { Stream };
// to do: Actual class here

export function writeString(stream: Stream, value: string): void {
	stream.writeUShort(value.length);
	stream.append(new TextEncoder().encode(value));
}

export function readString(stream: Stream): string {
	return new TextDecoder().decode(
		stream.read(stream.readUShort())
	);
}

export function readAddress(stream: Stream): Address {
	let ver = stream.readByte() || 6;
	if (ver == 4) {
		 // Read 4 bytes
		 let ipBytes = stream.buffer.slice(stream.offset, stream.addOffset(4, true));
		 let addr = `${(-ipBytes[0]-1)&0xff}.${(-ipBytes[1]-1)&0xff}.${(-ipBytes[2]-1)&0xff}.${(-ipBytes[3]-1)&0xff}`;
		 let port = stream.readShort();
		 return new Address(addr, port, ver);
	} else {
		 stream.offset += 2; // Skip 2 bytes
		 let port = stream.readShort();
		 stream.offset += 4; // Skip 4 bytes
		 let addr = stream.buffer.slice(stream.offset, stream.offset += 16).toString('utf8');
		 stream.offset += 4;  // Skip 4 bytes
		 return new Address(addr, port, ver as 6);
	}
}

// Writes an IPv4 address into the buffer
// Needs to get refactored, also needs to be added support for IPv6
export function writeAddress(stream: Stream, address: Address): void {
	stream.writeByte(address.version || 4);
	address.ip.split('.', 4).forEach((b: string) => stream.writeByte(-b-1));
	stream.writeShort(address.port);
}