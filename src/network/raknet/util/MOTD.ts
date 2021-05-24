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
interface PlayerCount {
	online: number;
	max: number;
}

class MOTD {
	public motd: string = 'Netrex Server';
	public name: string = 'NetrexRakNet';
	public protocol: number  = 420;
	public version: string = '1.17.0';
	public players: PlayerCount = {
		 online: 0,
		 max: 100
	};
	public gamemode: string = 'Creative';
	public serverId: bigint = BigInt(0);

	public toString(): string {
		 return [
			  'MCPE',
			  this.motd,
			  this.protocol,
			  this.version,
			  this.players.online,
			  this.players.max,
			  this.serverId,
			  this.name,
			  this.gamemode
		 ].join(';') + ';';
	}
}
export default MOTD;