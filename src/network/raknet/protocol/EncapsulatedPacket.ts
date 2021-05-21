import { Stream } from "../util/Stream.ts";

export enum PacketState {
	Sent,
	Split,
	Encapsulated
}

export interface PacketHeader {
	peerConnected: boolean;
	ack: boolean;
	nak: boolean;
	pair: boolean;
	continuous: boolean;
	bas: boolean;
}

export function getPacketHeader(byte: number): PacketHeader {
	return {
		peerConnected: (byte & 1 << 7) != 0,
		ack: (byte & 1 << 6) != 0,
		nak: (byte & 1 << 5) != 0,
		pair: (byte & 1 << 4) != 0,
		continuous: (byte & 1 << 3) != 0,
		bas: (byte & 1 << 2) != 0
	}
}

export default abstract class EncapsulatedPacket {
	public abstract readonly header: PacketHeader;
	public abstract state: PacketState;
}