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

export default class OpenConnectRequest extends OfflinePacket {
	public id = OfflinePacketIds.OpenConnectRequest;
	#protocol?: number;
	#mtuSize?: number;

	public constructor(protocol: number, mtuSize: number) {
		super();
		this.#protocol = protocol;
		this.#mtuSize = mtuSize;
	}

	public static from(s: Stream): OpenConnectRequest {
		s.read(16);
		const proto: number = s.readByte();
		const mtu: number = s.buffer.byteLength + 1 + 28;
		return new OpenConnectRequest(proto, mtu);
	}

	public get protocol(): number {
		return this.#protocol || 10;
	}

	public get mtuSize(): number {
		return this.#mtuSize || 1024;
	}
}