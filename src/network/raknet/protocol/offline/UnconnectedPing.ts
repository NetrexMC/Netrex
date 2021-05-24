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
import { Stream } from "../../util/Stream.ts";
import OfflinePacket, { OfflinePacketIds } from "./OfflinePacket.ts";

export class UnconnectedPing extends OfflinePacket {
	public id = OfflinePacketIds.UnconnectedPing;
	public clientGUID: BigInt;
	public clientTime: BigInt;

	public constructor(time: BigInt, guid: BigInt) {
		super();
		this.clientGUID = guid;
		this.clientTime = time;
	}

	public static from(stream: Stream): UnconnectedPing {
		const time: BigInt = stream.readLong();
		const guid: BigInt = stream.readLong();
		return new UnconnectedPing(time, guid);
	}
}