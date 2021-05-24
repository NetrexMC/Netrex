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
export enum IReliability {
	Unreliable = 0x0,
	UnreliableSeq = 0x01,
	Reliable,
	ReliableOrd,
	ReliableSeq,
	UnreliableAck,
	ReliableAck,
	ReliableOrdAck
};

export default class Reliability {
	public static isReliable(r: number): boolean {
		return (
			r === IReliability.Reliable ||
			r === IReliability.ReliableOrd ||
			r === IReliability.ReliableOrdAck ||
			r === IReliability.ReliableAck ||
			r === IReliability.ReliableSeq
		);
	}

	public static isSequenced(r: number): boolean {
		return r === IReliability.ReliableSeq || r === IReliability.UnreliableSeq;
	}

	public static isOrdered(r: number): boolean {
		return r === IReliability.ReliableOrd || r === IReliability.ReliableOrdAck;
	}

	public static isOrdOrSeq(r: number): boolean {
		return this.isOrdered(r) || this.isSequenced(r);
	}
}