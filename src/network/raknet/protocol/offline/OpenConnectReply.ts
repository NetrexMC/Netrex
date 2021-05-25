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
import { Stream } from "../../util/Stream.ts";
import { ClientBound } from "../RakPacket.ts";
import OfflinePacket, { OfflinePacketIds } from "./OfflinePacket.ts";

export default class OpenConnectReply extends OfflinePacket implements ClientBound {
	public id = OfflinePacketIds.OpenConnectReply;
	public serverId: bigint;
	public secure: boolean;
	public mtu: number;

	public constructor(serverid: bigint, secure: boolean, mtu: number) {
		super();
		this.serverId = serverid;
		this.secure = secure;
		this.mtu = mtu;
	}

	public parse(): Stream {
		const stream = new Stream();
		stream.append(MAGIC);
		stream.writeLong(this.serverId);
		stream.writeBool(this.secure);
		stream.writeShort(this.mtu);
		return stream;
	}
}