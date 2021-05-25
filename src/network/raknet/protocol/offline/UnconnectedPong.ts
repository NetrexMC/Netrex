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
import { MAGIC } from "../../RakHandler.ts";
import MOTD from "../../util/MOTD.ts";
import { Stream } from "../../util/Stream.ts";
import { ClientBound } from "../RakPacket.ts";
import OfflinePacket, { OfflinePacketIds } from "./OfflinePacket.ts";

export class UnconnectedPong extends OfflinePacket implements ClientBound {
	public id = OfflinePacketIds.UnconnectedPong;
	public time: bigint;
	public serverId: bigint;
	public motd: MOTD;

	public constructor(time: bigint, serverid: bigint, motd: MOTD) {
		super();
		this.time = time;
		this.serverId = serverid;
		this.motd = motd;
	}

	public parse(): Stream {
		const stream = new Stream();
		stream.writeLong(this.time);
		stream.writeLong(this.serverId);
		stream.append(MAGIC);
		// write string method soon
		stream.writeShort(this.motd.toString().length);
		stream.append(new TextEncoder().encode(this.motd.toString()));
		return stream;
	}
}