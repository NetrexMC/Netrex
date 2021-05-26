/**
 * ------------------------------------------------------------
 * Script: mod.ts
 * Description: Main script CLI entry point, simple bootstrap
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import { CommandHandler } from "./commands/Command.ts";
import CountCommand from "./commands/CountCommand.ts";
import DrunCommand from "./commands/DrunCommand.ts";
import HeaderCommand from "./commands/HeaderCommand.ts";
import UpdateCommand from "./commands/UpdateCommand.ts";
import WorkInit from "./commands/WorkInitCommand.ts";

if (import.meta.main) {
	const handler = new CommandHandler();
	handler.register(new CountCommand());
	handler.register(new HeaderCommand());
	handler.register(new UpdateCommand());
	handler.register(new WorkInit());
	handler.register(new DrunCommand());
	handler.runMain();
}