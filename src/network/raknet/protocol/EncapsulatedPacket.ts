/**
 *  _   _      _
 * | \ | |    | |
 * |  \| | ___| |_ _ __ _____  __
 * | . ` |/ _ \ __| '__/ _ \ \/ /
 * | |\  |  __/ |_| | |  __/>  <
 * |_| \_|\___|\__|_|  \___/_/\_\
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * @author Netrex Team
 * @link https://github.com/NetrexMC
 *
 * © Netrex 2020 - 2021
 */
import { Stream } from "../util/Stream.ts";
import RakPacket from "./RakPacket.ts";

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
	public get packets(): RakPacket[] {
		return [];
	}
}