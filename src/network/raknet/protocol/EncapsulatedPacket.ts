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
import Reliability, { IReliability } from "./Reliability.ts";

export enum PacketState {
	Sent,
	Split,
	Encapsulated
}

export interface FrameInstruction {
	messageId?: number;
	sequenceId?: number;
	orderId?: number;
	orderChan?: number;
}

export interface FragmentInformation {
	size?: number;
	id?: number;
	index?: number;
}

// (https://wiki.vg/Raknet_Protocol#Frame_Set_Packet)
export default class EncapsulatedPacket {
	public reliability: IReliability = IReliability.Unreliable;
	public state: PacketState = PacketState.Encapsulated;
	public frameInstruction: FrameInstruction = {};
	public fragmentInfo: FragmentInformation = {};
	#buffer: Stream;

	public constructor(stream: Stream) {
		const flags = stream.readByte();
		const length = Math.ceil(stream.readUShort() / 8);
		this.reliability = (flags & 224) >> 5;

		if (Reliability.isReliable(this.reliability)) {
			this.frameInstruction.messageId = stream.readLTriad();
		}

		if (Reliability.isSequenced(this.reliability)) {
			this.frameInstruction.orderChan = stream.readLTriad();
		}

		if (Reliability.isOrdOrSeq(this.reliability)) {
			this.frameInstruction.orderId = stream.readLTriad();
			this.frameInstruction.orderChan = stream.readByte();
		}

		// Frame is fragmented.
		// to-do: Store known frames and send sorted and sequentially
		if ((flags & 0x10) > 0) {
			this.fragmentInfo.size = stream.readInt();
			this.fragmentInfo.id = stream.readShort();
			this.fragmentInfo.index = stream.readInt();
		}

		this.#buffer = new Stream(stream.read(length));
	}

	public get buffer(): Stream {
		return this.#buffer;
	}
}