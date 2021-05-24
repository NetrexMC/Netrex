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

export enum OfflinePacketIds {
	UnconnectedPing = 0x01,
	OpenConnectRequest = 0x05,
	OpenConnectReply = 0x06,
	SessionInfo = 0x07,
	SessionInfoReply = 0x08,
	UnconnectedPong = 0x1c
}

export abstract class OfflinePacket {
	public abstract id: OfflinePacketIds;
}

export default OfflinePacket;